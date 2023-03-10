use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{self, Parser},
    parse_macro_input, ItemStruct,
};

#[proc_macro_derive(BaseModel)]
pub fn field_names(input: TokenStream) -> TokenStream {
    let is = parse_macro_input!(input as ItemStruct);
    let name = &is.ident;
    let table_name = name.to_string().to_lowercase();
    let fields = match is.fields {
        syn::Fields::Named(ref fields) => fields,
        _ => panic!("BaseModel can only be derived for structs with named fields"),
    };
    let columns = fields.named.iter().map(|f| &f.ident);
    let length = columns.len();
    let gen = quote! {
        impl #name {
            const COLUMN_NAMES: [&'static str; #length] = [#(stringify!(#columns)),*];
        }
        impl Schema for #name {

            fn table_name() -> &'static str {
                #table_name
            }

            fn column_names() -> &'static [&'static str] {
                &Self::COLUMN_NAMES
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
            quote! { pub created_at: Option<DateTime<Local>> },
            quote! { pub updated_at: Option<DateTime<Local>> },
            quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub deleted_at: Option<DateTime<Local>>
            },
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
