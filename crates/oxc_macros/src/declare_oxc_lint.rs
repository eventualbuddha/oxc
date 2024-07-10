use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Error, Expr, Ident, Lit, LitStr, Meta, Result, Token,
};

pub struct LintRuleMeta {
    name: Ident,
    category: Ident,
    /// Struct implementing [`JsonSchema`] that describes the rule's config.
    schema: Option<Ident>,
    documentation: String,
    pub used_in_test: bool,
}

impl Parse for LintRuleMeta {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut documentation = String::new();
        for attr in input.call(Attribute::parse_outer)? {
            if let Some(lit) = parse_attr(["doc"], &attr) {
                let value = lit.value();
                let line = value.strip_prefix(' ').unwrap_or(&value);

                documentation.push_str(line);
                documentation.push('\n');
            } else {
                return Err(Error::new_spanned(attr, "unexpected attribute"));
            }
        }

        let struct_name = input.parse()?;
        input.parse::<Token!(,)>()?;
        let category = input.parse()?;

        let schema = if input.peek(Ident) {
            Some(input.parse()?)
        } else {
            None
        };

        // Ignore the rest
        input.parse::<proc_macro2::TokenStream>()?;

        Ok(Self { name: struct_name, category, schema, documentation, used_in_test: false })
    }
}

pub fn declare_oxc_lint(metadata: LintRuleMeta) -> TokenStream {
    let LintRuleMeta { name, category, schema, documentation, used_in_test } = metadata;
    // e.g. NoDebugger to no-debugger
    let canonical_name = name.to_string().to_case(Case::Kebab);
    let category = match category.to_string().as_str() {
        "correctness" => quote! { RuleCategory::Correctness },
        "suspicious" => quote! { RuleCategory::Suspicious },
        "pedantic" => quote! { RuleCategory::Pedantic },
        "perf" => quote! { RuleCategory::Perf },
        "style" => quote! { RuleCategory::Style },
        "restriction" => quote! { RuleCategory::Restriction },
        "nursery" => quote! { RuleCategory::Nursery },
        _ => panic!("invalid rule category"),
    };

    let import_statement = if used_in_test {
        None
    } else {
        Some(quote! { use crate::rule::{RuleCategory, RuleMeta}; })
    };

    let schema = schema.map(|s| quote! { Some(gen.subschema_for::<#s>()) }).unwrap_or_else(|| quote! { None });

    let output = quote! {
        #import_statement

        impl RuleMeta for #name {
            const NAME: &'static str = #canonical_name;

            const CATEGORY: RuleCategory = #category;

            fn documentation() -> Option<&'static str> {
                Some(#documentation)
            }
            #[allow(unused_variables)]
            fn schema(gen: &mut schemars::gen::SchemaGenerator) -> Option<schemars::schema::Schema> {
                #schema
            }
        }

        impl schemars::JsonSchema for #name {
            #[inline]
            fn schema_name() -> String {
                Self::NAME.to_string()
            }

            #[inline]
            fn schema_id() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(#canonical_name)
            }

            fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                use schemars::schema::{Schema, SchemaObject};

                let mut schema: Schema = #name::schema(gen)
                    .unwrap_or_else(|| {
                        let mut obj = SchemaObject::default();
                        obj.object().additional_properties = Some(Box::new(Schema::Bool(true)));
                        obj.into()
                    });

                let schema = match schema {
                    Schema::Object(mut obj) => {
                        let meta = obj.metadata();
                        meta.title = Some("Config for ".to_string() + Self::NAME);
                        // meta.description = Self::documentation().map(Into::into);
                        // if let Some(docs) = Self::documentation() {
                        //     obj.extensions.insert("markdownDescription".into(), docs.into());
                        // }
                        Schema::Object(obj)
                    },
                    s => s
                };

                schema
            }
        }
    };

    TokenStream::from(output)
}

fn parse_attr<'a, const LEN: usize>(
    path: [&'static str; LEN],
    attr: &'a Attribute,
) -> Option<&'a LitStr> {
    if let Meta::NameValue(name_value) = &attr.meta {
        let path_idents = name_value.path.segments.iter().map(|segment| &segment.ident);
        if itertools::equal(path_idents, path) {
            if let Expr::Lit(expr_lit) = &name_value.value {
                if let Lit::Str(s) = &expr_lit.lit {
                    return Some(s);
                }
            }
        }
    }
    None
}
