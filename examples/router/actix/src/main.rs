#![feature(proc_macro_hygiene)]

use actix_web::HttpResponse;
use reign::{
    prelude::*,
    router::middleware::{HeadersDefault, Runtime},
};
use serde_json::{from_str, to_string, Value};

mod errors;

#[action]
fn str_() {
    Ok("str")
}

#[action]
fn string() {
    Ok("string".to_string())
}

#[action]
fn response() {
    Ok(HttpResponse::Ok().body("response"))
}

#[action]
fn error() {
    let value = from_str::<Value>("{name}")?;
    Ok(to_string(&value)?)
}

#[action]
fn post() {
    Ok("post")
}

#[action]
fn put() {
    Ok("put")
}

#[action]
fn patch() {
    Ok("patch")
}

#[action]
fn delete() {
    Ok("delete")
}

#[action]
fn methods() {
    Ok("methods")
}

async fn param(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("param {}", req)
}

async fn param_optional(req: actix_web::web::HttpRequest) -> impl actix_web::Responder {
    format!(
        "param_optional {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn param_regex(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("param_regex {}", req)
}

async fn param_optional_regex(req: actix_web::web::HttpRequest) -> impl actix_web::Responder {
    format!(
        "param_optional_regex {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn param_glob(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("param_glob {}", req)
}

async fn param_optional_glob(req: actix_web::web::HttpRequest) -> impl actix_web::Responder {
    format!(
        "param_optional_glob {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn param_glob_after(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("param_glob_after {}", req)
}

async fn param_optional_glob_after(req: actix_web::web::HttpRequest) -> impl actix_web::Responder {
    format!(
        "param_optional_glob_after {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

#[action]
fn scope_static_b() {
    Ok("scope_static_b")
}

async fn scope_param(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("scope_param {}", req)
}

async fn scope_param_b(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("scope_param_b {}", req)
}

async fn scope_param_optional(req: actix_web::web::HttpRequest) -> impl actix_web::Responder {
    format!(
        "scope_param_optional {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn scope_param_optional_b(req: actix_web::web::HttpRequest) -> impl actix_web::Responder {
    format!(
        "scope_param_optional_b {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn scope_param_regex(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("scope_param_regex {}", req)
}

async fn scope_param_regex_b(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("scope_param_regex_b {}", req)
}

async fn scope_param_optional_regex(req: actix_web::web::HttpRequest) -> impl actix_web::Responder {
    format!(
        "scope_param_optional_regex {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn scope_param_optional_regex_b(
    req: actix_web::web::HttpRequest,
) -> impl actix_web::Responder {
    format!(
        "scope_param_optional_regex_b {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn scope_param_glob(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("scope_param_glob {}", req)
}

async fn scope_param_glob_b(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("scope_param_glob_b {}", req)
}

async fn scope_param_optional_glob(req: actix_web::web::HttpRequest) -> impl actix_web::Responder {
    format!(
        "scope_param_optional_glob {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn scope_param_optional_glob_b(
    req: actix_web::web::HttpRequest,
) -> impl actix_web::Responder {
    format!(
        "scope_param_optional_glob_b {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn scope_param_glob_after(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("scope_param_glob_after {}", req)
}

async fn scope_param_glob_after_b(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("scope_param_glob_after_b {}", req)
}

async fn scope_param_optional_glob_after(
    req: actix_web::web::HttpRequest,
) -> impl actix_web::Responder {
    format!(
        "scope_param_optional_glob_after {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn scope_param_optional_glob_after_b(
    req: actix_web::web::HttpRequest,
) -> impl actix_web::Responder {
    format!(
        "scope_param_optional_glob_after_b {}",
        match req.match_info().get("id") {
            Some(id) => id,
            None => "",
        }
    )
}

async fn nested_scope(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("nested_scope {}", req)
}

async fn nested_scope_b(req: actix_web::web::Path<String>) -> impl actix_web::Responder {
    format!("nested_scope_b {}", req)
}

#[action]
fn double_slashes() {
    Ok("double_slashes")
}

#[action]
fn param_typed() {
    Ok("param_typed")
}

#[action]
fn sibling_scope_higher() {
    Ok("sibling_scope_higher")
}

#[action]
fn sibling_scope_common_higher() {
    Ok("sibling_scope_common_higher")
}

#[action]
fn sibling_scope_common_lower() {
    Ok("sibling_scope_common_lower")
}

#[action]
fn sibling_scope_common_c() {
    Ok("sibling_scope_common_c")
}

#[action]
fn sibling_scope_lower() {
    Ok("sibling_scope_lower")
}

#[action]
fn scope_static() {
    Ok("scope_static")
}

#[action]
fn pipe() {
    Ok("pipe")
}

#[action]
fn pipe_empty() {
    Ok("pipe_empty")
}

router!(
    pipelines!(
        common: [
            HeadersDefault::empty().add("x-powered-by", "reign"),
        ],
        app: [
            HeadersDefault::empty().add("x-content-type-options", "nosniff"),
        ],
        timer: [
            Runtime::default(),
        ],
        api: [
            HeadersDefault::empty().add("x-version", "1.0"),
            HeadersDefault::empty().add("content-type", "application/json"),
        ],
    );

    // TODO:(router) make path optional here
    scope!("/", [common, app], {
        get!("str", str_);
        get!("string", string);
        get!("response", response);

        get!("error", error);

        post!("post", post);
        put!("put", put);
        patch!("patch", patch);
        delete!("delete", delete);

        methods!([post, put], "methods", methods);

        app = app.route("param/{id}", actix_web::web::get().to(param));

        app = app
            .route("param_optional", actix_web::web::get().to(param_optional))
            .route("param_optional/{id}", actix_web::web::get().to(param_optional));

        app = app.route("param_regex/{id:[0-9]+}", actix_web::web::get().to(param_regex));

        app = app
            .route("param_optional_regex", actix_web::web::get().to(param_optional_regex))
            .route("param_optional_regex/{id:[0-9]+}", actix_web::web::get().to(param_optional_regex));

        app = app.route("param_glob/{id:.+}", actix_web::web::get().to(param_glob));

        app = app
            .route("param_optional_glob", actix_web::web::get().to(param_optional_glob))
            .route("param_optional_glob/{id:.+}", actix_web::web::get().to(param_optional_glob));

        app = app.route("param_glob_after/{id:.+}/b", actix_web::web::get().to(param_glob_after));

        app = app
            .route("param_optional_glob_after/b", actix_web::web::get().to(param_optional_glob_after))
            .route("param_optional_glob_after/{id:.+}/b", actix_web::web::get().to(param_optional_glob_after));

        get!("double//slashes", double_slashes);

        // get!("/param/{foo}", param);
        // / "param" / foo
        // get!("/param_optional/{foo?}", param_optional);
        // "param" / foo?
        // "param" / foo: Option<String>
        // get!("/param_typed/{foo:u16}", param_typed);
        // "param" / foo: u16
        // get!("/param_regex/{foo:[a-f]{6}}/{bar:\\d+}", param_regex);
        // "param" / foo: u16 @ "[0-9]+"
        // get!("/param_glob/{foo*}", param_glob);
        // "param" / foo*
        // "param" / foo: Vec<String>
        // get!("/param_optional_glob/{foo*?}", param_optional_glob);
        // "param" / foo*?
        // "param" / foo: Option<Vec<String>>

        // TODO: Trailing slashes

        app = app.service({
            let mut app = actix_web::web::scope("scope_static");

            app = app.route("/b", actix_web::web::get().to(scope_static_b));
            app = app.route("", actix_web::web::get().to(scope_static));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param/{id}");

            app = app.route("/b", actix_web::web::get().to(scope_param_b));
            app = app.route("", actix_web::web::get().to(scope_param));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_optional");

            app = app.route("/b", actix_web::web::get().to(scope_param_optional_b));
            app = app.route("", actix_web::web::get().to(scope_param_optional));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_optional/{id}");

            app = app.route("/b", actix_web::web::get().to(scope_param_optional_b));
            app = app.route("", actix_web::web::get().to(scope_param_optional));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_regex/{id:[0-9]+}");

            app = app.route("/b", actix_web::web::get().to(scope_param_regex_b));
            app = app.route("", actix_web::web::get().to(scope_param_regex));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_optional_regex");

            app = app.route("/b", actix_web::web::get().to(scope_param_optional_regex_b));
            app = app.route("", actix_web::web::get().to(scope_param_optional_regex));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_optional_regex/{id:[0-9]+}");

            app = app.route("/b", actix_web::web::get().to(scope_param_optional_regex_b));
            app = app.route("", actix_web::web::get().to(scope_param_optional_regex));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_glob/{id:.+}");

            app = app.route("/b", actix_web::web::get().to(scope_param_glob_b));
            app = app.route("", actix_web::web::get().to(scope_param_glob));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_optional_glob");

            app = app.route("/b", actix_web::web::get().to(scope_param_optional_glob_b));
            app = app.route("", actix_web::web::get().to(scope_param_optional_glob));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_optional_glob/{id:.+}");

            app = app.route("/b", actix_web::web::get().to(scope_param_optional_glob_b));
            app = app.route("", actix_web::web::get().to(scope_param_optional_glob));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_glob_after/{id:.+}/foo");

            app = app.route("/b", actix_web::web::get().to(scope_param_glob_after_b));
            app = app.route("", actix_web::web::get().to(scope_param_glob_after));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_optional_glob_after/foo");

            app = app.route("/b", actix_web::web::get().to(scope_param_optional_glob_after_b));
            app = app.route("", actix_web::web::get().to(scope_param_optional_glob_after));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("scope_param_optional_glob_after/{id:.+}/foo");

            app = app.route("/b", actix_web::web::get().to(scope_param_optional_glob_after_b));
            app = app.route("", actix_web::web::get().to(scope_param_optional_glob_after));

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("nested_scope/{id}");

            app = app.service({
                let mut app = actix_web::web::scope("nested_scope_inner");

                app = app.route("/b", actix_web::web::get().to(nested_scope_b));
                app = app.route("", actix_web::web::get().to(nested_scope));

                app
            });

            app
        });

        app = app.service({
            let mut app = actix_web::web::scope("sibling_scope/higher");
            app = app.route("", actix_web::web::get().to(sibling_scope_higher));
            app
        });
        app = app.service({
            let mut app = actix_web::web::scope("sibling_scope");
            app = app.route("/higher", actix_web::web::get().to(sibling_scope_common_higher));
            app = app.route("/lower", actix_web::web::get().to(sibling_scope_common_lower));
            app = app.route("/c", actix_web::web::get().to(sibling_scope_common_c));
            app
        });
        app = app.service({
            let mut app = actix_web::web::scope("sibling_scope/lower");
            app = app.route("", actix_web::web::get().to(sibling_scope_lower));
            app
        });

        scope!("/scope-static", {
            get!("", scope_static);
        });

        scope!("/pipe", [timer], {
            get!("", pipe);
        });

        scope!("/pipe-empty", [], {
            get!("", pipe_empty);
        });

        // TODO:(router) any
        // TODO:(router) 301, 302
    });

    // scope!("/api", [common, api], {
    //     get!("/", api);
    // });
);

async fn server() {
    router("127.0.0.1:8100").await.unwrap();
}

#[actix_rt::main]
async fn main() {
    server().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::{spawn, time::delay_for};
    use std::time::Duration;
    use test_examples::router::{test, StatusCode};

    #[actix_rt::test]
    async fn test_server() {
        spawn(server());

        let client = async {
            delay_for(Duration::from_millis(100)).await;
            test(StatusCode::NOT_FOUND).await
        };

        client.await;
    }
}
