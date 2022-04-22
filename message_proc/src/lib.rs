use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

#[proc_macro_derive(ImplIntoArma)]
/// Creates the to_arma function based on the attributes for this struct
pub fn derive_into_arma(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let struct_name = format_ident!("{}", &name);

    let data = &input.data;
    let mut arguments: Vec<syn::Ident> = Vec::new();

    let data = match data {
        syn::Data::Struct(data) => data,
        _ => panic!("ImplIntoArma is only available for Structs"),
    };

    // Get all the struct fields
    for field in &data.fields {
        if let syn::Visibility::Restricted(_) = field.vis {
            continue;
        }
        if field.ident.is_none() {
            continue;
        }

        arguments.push(field.ident.as_ref().unwrap().to_owned());
    }

    // Builds the IntoArma implementation
    let expanded = quote! {
        impl arma_rs::IntoArma for #struct_name {
            fn to_arma(&self) -> arma_rs::Value {
                let mut keys: Vec<arma_rs::Value> = Vec::new();
                let mut values: Vec<arma_rs::Value> = Vec::new();

                #(
                    keys.push(stringify!(#arguments).to_arma());
                    values.push(self.#arguments.to_arma());
                )*

                let hash: Vec<Vec<arma_rs::Value>> = vec![keys, values];
                hash.to_arma()
            }
        }
    };

    TokenStream::from(expanded)
}
