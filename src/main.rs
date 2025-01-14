use std::error::Error;
use axum::{body::Body, extract::Request, middleware::{self, Next}, response::Response, routing::get, Router};
use clap::Parser;
use log;
use tokio::net::TcpListener;
mod handlers;
mod fixture;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Local port to run the server on
    #[arg(short, long, default_value_t=9000)]
    port: u16,

}

async fn logging_middleware(
    request: Request,
    next: Next
) -> Response {
    let method = &request.method().to_owned();
    let uri = &request.uri().to_string();
    let headers = &request.headers().to_owned();

    let next_layer_response = next.run(request).await;
    
    log::info!("{} {} -> {}", method, uri, next_layer_response.status().as_u16());

    let response_headers = next_layer_response.headers();

    log::debug!("Request headers:");
    for (k,v) in headers {
        log::debug!("\t{}: {}", k, v.to_str().unwrap_or("invalid string"));
    }

    log::debug!("Response headers:");
    for (k,v) in response_headers {
        log::debug!("\t{}: {}", k, v.to_str().unwrap_or("invalid string"));
    }
    next_layer_response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    colog::init();

    let args = Args::parse();

    let app = Router::new()
        .route("/collections", get(handlers::get_user_collections))
        .fallback(handlers::generic404)
        .layer(middleware::from_fn(logging_middleware));

    let bind_addr = format!("0.0.0.0:{}", args.port);

    let listener = TcpListener::bind(bind_addr).await?;
    log::info!("Listening for connections on port {}", args.port);

    axum::serve(listener, app).await?;

    log::info!("Exiting");

    Ok( () )
}
