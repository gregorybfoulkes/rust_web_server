use hyper::service::{make_service_fn, service_fn};
use hyper::{Request, Response, Server, StatusCode};

/// The hyper handler is the entrypoint for the web server. It is a [hyper](https://hyper.rs/)
/// service that takes incoming requests and responds with a fixed "Hello, World!" message.
///
/// The handler is created by the `main` function, which is the entrypoint for the web server.
/// The `main` function returns a [hyper::Server] instance, which is a service that can be used
/// to serve incoming requests.
///
/// The handler is implemented as a closure that takes a reference to a [hyper::Request] as an
/// argument. The closure returns a [hyper::Response] instance, which contains the response to
/// the request.
///


async fn handle_request(_req: Request<hyper::body::Body>) -> Result<Response<hyper::body::Body>, hyper::Error> {
    Ok(Response::new("Hello, World!".into()))
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let make_service = make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(handle_request)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://localhost:3000");
    server.await
}
