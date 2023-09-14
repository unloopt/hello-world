use anyhow::Result;
use http::response::Builder;
use spin_sdk::{
    config,
    http::{Params, Request, Response, Router},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_private(req: Request) -> Result<Response> {
    let mut router = Router::default();
    //router.get("/hello-world", search::search);
    router.add("/...", http::Method::OPTIONS, process_preflight);
    router.handle(req)
}

fn builder_with_cors(origin: String, methods: String, status: http::StatusCode) -> Builder {
    http::Response::builder()
        .status(status)
        .header("Access-Control-Allow-Origin", origin)
        .header("Access-Control-Allow-Methods", methods)
        .header("Access-Control-Allow-Headers", "*")
}

pub(crate) fn process_preflight(_req: Request, _params: Params) -> Result<Response> {
    let Ok(origin) = config::get("cors_origin") else {
        println!("[ERROR]: Could not find CORS origin");
        return Ok(http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(None)?);
    };

    let Ok(methods) = config::get("cors_methods") else {
        println!("[ERROR]: Could not find CORS methods");
        return Ok(http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(None)?);
    };

    Ok(builder_with_cors(origin, methods, http::StatusCode::OK).body(None)?)
}
