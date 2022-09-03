use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server,
};
use opentelemetry::Context;
use prometheus::{Encoder, TextEncoder};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::SystemTime;

use crate::{config::ExporterConfig, state::AppState};

async fn serve_req(
    cx: Context,
    req: Request<Body>,
    state: Arc<AppState>,
) -> Result<Response<Body>, hyper::Error> {
    println!("Receiving request at path {}", req.uri());
    let request_start = SystemTime::now();

    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/metrics") => {
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = state.gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();

            Response::builder()
                .status(200)
                .header(CONTENT_TYPE, encoder.format_type())
                .body(Body::from(buffer))
                .unwrap()
        }
        (&Method::GET, "/") => Response::builder()
            .status(200)
            .body(Body::from("Hello World"))
            .unwrap(),
        _ => Response::builder()
            .status(404)
            .body(Body::from("Missing Page"))
            .unwrap(),
    };
    Ok(response)
}

// #[tokio::main]
pub async fn start_server(
    config: &ExporterConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cx = Context::new();
    let state = Arc::new(AppState::init(config));
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(move |_conn| {
        let state = state.clone();
        let cx = cx.clone();
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                serve_req(cx.clone(), req, state.clone())
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 8526).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
