use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use serde_json::Value;
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro]
#[allow(clippy::missing_panics_doc)]
pub fn generate_constraints(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = std::path::Path::new(&manifest_dir).join(input.value());
    let json_str = std::fs::read_to_string(path).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();

    let constraints = json["constraints"].as_object().unwrap();
    let generated = constraints.iter().map(|(pattern, id)| {
        let id = id.as_u64().unwrap().to_string();
        let struct_name = Ident::new(&format!("Constraint{id}"), Span::call_site());
        let pattern = pattern.to_string();

        quote! {
            pub struct #struct_name;
            impl PathConstraint for #struct_name {
                const NAME: &'static str = #id;

                fn check(segment: &str) -> bool {
                    static REGEX: LazyLock<Regex> = LazyLock::new(|| {
                        Regex::new(#pattern).unwrap()
                    });

                    REGEX.is_match(segment).unwrap_or(false)
                }
            }
        }
    });

    let constraint_structs: Vec<_> = constraints
        .iter()
        .map(|(_, id)| {
            let id = id.as_u64().unwrap();
            Ident::new(&format!("Constraint{id}"), Span::call_site())
        })
        .collect();

    let constraints_fn = quote! {
        pub fn constraints<T>(router: &mut Router<T>) {
            #(
                router.path.constraint::<#constraint_structs>().unwrap();
            )*
        }
    };

    let expanded = quote! {
        #(#generated)*
        #constraints_fn
    };

    TokenStream::from(expanded)
}
