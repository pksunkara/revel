use crate::router::Methods;
use proc_macro2::TokenStream;
use quote::quote;

pub fn actix(input: Methods) -> TokenStream {
    let Methods {
        methods,
        path,
        action,
        prev,
    } = input;

    let methods = methods.iter().map(|i| i).collect::<Vec<_>>();
    let (paths, params) = path.actix(false);

    paths
        .iter()
        .map(|path| {
            quote! {
                app = app
                    #(.route(#path, ::actix_web::web::#methods().to(#action)))*
            }
        })
        .collect()
}
