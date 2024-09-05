use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::LateCtx,
    schema::{EnumDef, GetGenerics, StructDef, ToType, TypeDef},
    util::ToIdent,
};

use super::{define_derive, Derive, DeriveOutput};

define_derive! {
    pub struct DeriveContentHash;
}

const IGNORE_FIELDS: [(/* field name */ &str, /* field type */ &str); 6] = [
    ("span", "Span"),
    ("trailing_comma", "Span"),
    ("this_span", "Span"),
    ("scope_id", "ScopeId"),
    ("symbol_id", "SymbolId"),
    ("reference_id", "ReferenceId"),
];

impl Derive for DeriveContentHash {
    fn trait_name() -> &'static str {
        "ContentHash"
    }

    fn derive(&mut self, def: &TypeDef, _: &LateCtx) -> TokenStream {
        let (hasher, body) = match &def {
            TypeDef::Enum(it) => derive_enum(it),
            TypeDef::Struct(it) => derive_struct(it),
        };

        impl_content_hash(def, hasher, &body)
    }

    fn prelude() -> TokenStream {
        quote! {
            #![allow(clippy::match_same_arms)]

            ///@@line_break
            use std::{hash::Hasher, mem::discriminant};

            ///@@line_break
            use oxc_span::hash::ContentHash;
        }
    }
}

fn derive_enum(def: &EnumDef) -> (&str, TokenStream) {
    let mut body = quote! {
        ContentHash::content_hash(&discriminant(self), state);
    };

    body.extend(if def.is_unit() {
        TokenStream::default()
    } else {
        let mut non_exhaustive = false;
        let matches = def
            .all_variants()
            .filter_map(|var| {
                let ident = var.ident();
                if var.is_unit() {
                    non_exhaustive = true;
                    None
                } else {
                    Some(quote!(Self :: #ident(it) => it.content_hash(state)))
                }
            })
            .collect_vec();
        let exhaust = non_exhaustive.then(|| quote!(_ => {}));
        quote! {
            match self {
                #(#matches),*
                #exhaust
            }
        }
    });

    ("state", body)
}

fn derive_struct(def: &StructDef) -> (&str, TokenStream) {
    if def.fields.is_empty() {
        ("_", TokenStream::default())
    } else {
        let fields = def
            .fields
            .iter()
            .filter(|field| {
                let Some(name) = field.name.as_ref() else { return false };
                !IGNORE_FIELDS
                    .iter()
                    .any(|it| name == it.0 && field.typ.name().inner_name() == it.1)
            })
            .map(|field| {
                let ident = field.ident();
                quote!(self.#ident.content_hash(state);)
            })
            .collect_vec();
        if fields.is_empty() {
            ("_", TokenStream::default())
        } else {
            ("state", quote!(#(#fields)*))
        }
    }
}

fn impl_content_hash(def: &TypeDef, hasher_name: &str, body: &TokenStream) -> TokenStream {
    let ty = def.to_type();
    let generics = def.generics();
    let hasher = hasher_name.to_ident();

    quote! {
        impl #generics ContentHash for #ty {
            fn content_hash<H: Hasher>(&self, #hasher: &mut H) {
                #body
            }
        }
    }
}
