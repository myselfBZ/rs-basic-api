use axum::{extract::Request, middleware::Next, response::Response};

pub async fn logger_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let response = next.run(req).await;
    println!("method: {method}");
    response
}
