// Originally copied from https://docs.rs/hyper/0.14.8/hyper/server/index.html

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use once_cell::sync::OnceCell;
use slog::{debug, Drain};
use std::{convert::Infallible, net::SocketAddr};

static LOGGER: OnceCell<slog::Logger> = OnceCell::new();

async fn handle(req: Request<Body>) -> Result<Response<Body>, String> {
    debug!(LOGGER.get().unwrap(), "Request: {:#?}", req);
    if req.uri().path().ends_with("panic") {
        panic!("I was told to panic");
    }
    if req.uri().path().ends_with("err") {
        return Err("gus".to_string());
    }
    Ok(Response::new(Body::from("Hello World")))
}

#[tokio::main]
async fn main() {
    // Setup ALL the logging
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let drain = slog_term::FullFormat::new(decorator).build();
    let drain = slog::LevelFilter(drain, slog::Level::Debug).fuse();
    let log = slog::Logger::root(drain, slog::o!());
    let _scope_guard = slog_scope::set_global_logger(log.clone());
    let _log_guard = slog_stdlog::init().unwrap();
    LOGGER.set(log).unwrap();

    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn: &hyper::server::conn::AddrStream| async {
        Ok::<_, Infallible>(service_fn(handle))
    });
    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
