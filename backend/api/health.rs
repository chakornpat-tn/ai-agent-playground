use vercel_runtime::{Error, IntoServiceFn, Request, Response, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler.into_service_fn()).await
}

async fn handler(_request: Request) -> Result<Response<serde_json::Value>, Error> {
    let body = serde_json::json!({
        "status": "ok",
        "service": "farmart-backend",
        "runtime": "vercel-rust-poc",
    });

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)?)
}
