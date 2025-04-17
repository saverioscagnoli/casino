use http::{ApiHandler, App, Request, Response, async_trait};
use hyper::{Method, StatusCode};

struct H;

#[async_trait]
impl ApiHandler for H {
    async fn incoming(&self, req: Request) -> Result<Response, hyper::Error> {
        let method = req.method();
        let segments = req.segments();

        match (method, &segments[..]) {
            // GET /
            (&Method::GET, []) => Ok(Response::empty()
                .status(StatusCode::OK)
                .text("Is this the '/' page?")),

            // GET /hello/{id}
            (&Method::GET, ["hello", name]) => Ok(Response::empty()
                .status(StatusCode::OK)
                .text(format!("Hello {}!", name))),

            // Any other
            _ => Ok(Response::empty().status(StatusCode::NOT_FOUND)),
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5050";

    if let Ok(app) = App::new(addr.parse().expect("Invalid address")).await {
        println!("App listening on http://{}", addr);

        if let Err(e) = app.run(H).await {
            eprintln!("Fatal error: {}", e);
        }
    }
}
