use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// returns `#[repr(C, u8)]` if `enum_` has any non-unit variant,
/// Otherwise it would return `#[repr(u8)]`.
fn enum_repr(enum_: &syn::ItemEnum) -> TokenStream2 {
    if enum_.variants.iter().any(|var| !matches!(var.fields, syn::Fields::Unit)) {
        quote!(#[repr(C, u8)])
    } else {
        quote!(#[repr(u8)])
    }
}

/// This attribute serves two purposes,
/// First, it is a marker for our codegen to detect AST types. Furthermore.
/// It is also a lightweight macro; All of its computation is cached and
/// it only applies the following changes without any complex operation:
///
/// * Prepend `#[repr(C)]` to structs
/// * Prepend `#[repr(C, u8)]` to fieldful enums e.g. `enum E { X: u32, Y: u8 }`
/// * Prepend `#[repr(u8)]` to unit (fieldless) enums e.g. `enum E { X, Y, Z, }`
/// * Prepend `#[derive(oxc_ast_macros::Ast)]` to all structs and enums
///
#[proc_macro_attribute]
#[allow(clippy::missing_panics_doc)]
pub fn ast(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Item);

    let repr = match input {
        syn::Item::Enum(ref enum_) => enum_repr(enum_),
        syn::Item::Struct(_) => quote!(#[repr(C)]),

        _ => {
            unreachable!()
        }
    };

    let expanded = quote! {
        #[derive(::oxc_ast_macros::Ast)]
        #repr
        #input
    };
    TokenStream::from(expanded)
}

/// Dummy derive macro for a non-existent trait `Ast`.
///
/// Does not generate any code.
/// Only purpose is to allow using `#[scope]`, `#[visit]`, and other attrs in the AST node type defs.
/// These "marker" attributes are used in codegen.
#[proc_macro_derive(Ast, attributes(scope, visit, span, serde, tsify, generate_derive))]
pub fn ast_derive(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// Derive macro generating an impl of the trait `CloneIn`.
///
/// NOTE: This is an internal macro!
/// # Panics
///
#[proc_macro_derive(CloneIn)]
pub fn derive_clone_in(item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::Item);
    match &item {
        syn::Item::Struct(syn::ItemStruct { ident, generics, .. })
        | syn::Item::Enum(syn::ItemEnum { ident, generics, .. })
            if generics.params.is_empty() =>
        {
            quote! {
                #[automatically_derived]
                impl<'alloc> ::oxc_allocator::CloneIn<'alloc> for #ident {
                    type Cloned = #ident;

                    fn clone_in(&self, _: &'alloc ::oxc_allocator::Allocator) -> Self::Cloned {
                        std::clone::Clone::clone(self)
                    }
                }
            }
            .into()
        }
        _ => panic!("At the moment `CloneIn` derive macro only works for types without lifetimes and/or generic params"),
    }
}
