use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{self, Parser},
    parse_macro_input, ItemStruct,
};

#[proc_macro_derive(FieldNames)]
pub fn field_names(input: TokenStream) -> TokenStream {
    let is = parse_macro_input!(input as ItemStruct);
    let name = &is.ident;
    let fields = match is.fields {
        syn::Fields::Named(ref fields) => fields,
        _ => panic!("FieldNames can only be derived for structs with named fields"),
    };
    let field_names = fields.named.iter().map(|f| &f.ident);
    let gen = quote! {
        impl #name {
            pub fn field_names() -> Vec<&'static str> {
                vec![#(stringify!(#field_names)),*]
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn with_basemodel(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut is = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(attr as parse::Nothing);
    if let syn::Fields::Named(ref mut fields) = is.fields {
        let basemodel_fields = vec![
            // quote! { pub id: u64 },
            quote! { pub created_at: DateTime<Local> },
            quote! { pub updated_at: DateTime<Local> },
            quote! { pub deleted_at: Option<DateTime<Local>> },
        ];
        for field in basemodel_fields {
            fields
                .named
                .push(syn::Field::parse_named.parse2(field).unwrap());
        }
    }
    quote! {
        #is
    }
    .into()
}