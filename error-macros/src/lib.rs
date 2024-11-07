use proc_macro::TokenStream;
use syn::{Attribute, DeriveInput, Ident, Lifetime, LitInt};

use ty::gql_err::GqlErrCategory;

#[proc_macro_derive(Diagnostic, attributes(diag, code, category, http_code))]
pub fn diagnostic(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();

    let struct_name = ast.ident;
    let mut struct_lifetimes: Vec<&Lifetime> = Vec::new();
    for lt in ast.generics.lifetimes() {
        struct_lifetimes.push(&lt.lifetime);
    }

    let (ftl, code, category, http_code) = get_attrs(ast.attrs);

    let mut names: Vec<Ident> = Vec::new();
    let mut strs: Vec<String> = Vec::new();
    if let syn::Data::Struct(data) = ast.data {
        for field in data.fields.into_iter() {
            if let Some(ident) = field.ident {
                strs.push(ident.to_string());
                names.push(ident);
            }
        }
    }

    quote::quote! {
        impl<'a> IntoDiagnostic<'a> for #struct_name<#(#struct_lifetimes)*> {
            fn to_diagnostic(&self) -> DiagnosticBuilder<'a> {
                DiagnosticBuilder::new(#ftl, #code, #category, #http_code)
            }

            fn get_struct_info(&self) -> Vec<(&str, &str)> {
                let mut r: Vec<(&str, &str)> = Vec::new();
                #( r.push((#strs, &self.#names)); )*
                r
            }
        }
    }
    .into()
}

/// Return fluent message key and error code
fn get_attrs(attrs: Vec<Attribute>) -> (String, String, String, u16) {
    let (mut diag, mut code, mut category, mut http_code) =
        (String::new(), String::new(), String::new(), 0_u16);
    for attr in attrs {
        let ident = attr.path().get_ident().unwrap();

        match ident.to_string().as_str() {
            "diag" => {
                let list = attr.meta.require_list().unwrap();
                let ident = list.parse_args::<Ident>().unwrap();
                diag = ident.to_string();
            }
            "code" => {
                let list = attr.meta.require_list().unwrap();
                let ident = list.parse_args::<Ident>().unwrap();
                code = ident.to_string();
            }
            "category" => {
                let list = attr.meta.require_list().unwrap();
                let ident = list.parse_args::<Ident>().unwrap();
                category = get_err_category(ident.to_string()).to_string();
            }
            "http_code" => {
                let list = attr.meta.require_list().unwrap();
                let lit = list.parse_args::<LitInt>().unwrap();
                http_code = lit.base10_parse::<u16>().unwrap();
            }
            _ => unimplemented!(),
        }
    }
    (diag, code, category, http_code)
}

fn get_err_category(s: String) -> GqlErrCategory {
    match s.as_str() {
        "bad_user_input" => GqlErrCategory::BadUserInput,
        "internal_server_error" => GqlErrCategory::InternalServerError,
        "unauthorized" => GqlErrCategory::Unauthorized,
        _ => unimplemented!(),
    }
}
