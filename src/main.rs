use axum::extract::Path;
use axum::http::StatusCode;
use axum::{routing::get, Router};

use std::collections::HashMap;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn zero_day_error() -> Result<String, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn day_one(Path(params): Path<HashMap<String, String>>) -> String {
    format!("Hello, {}!", params.get("numbers").unwrap())
}

async fn build_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(zero_day_error))
        .route("/:numbers", get(day_one))
}

#[cfg(not(tarpaulin_include))]
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = build_router().await;

    Ok(router.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_hello_world() {
        let server = TestServer::new(build_router().await).unwrap();
        let response = server.get("/").expect_success().await;
        assert_eq!(hello_world().await, response.text());
    }

    #[tokio::test]
    async fn test_zero_day_error() {
        let server = TestServer::new(build_router().await).unwrap();
        let response = server.get("/-1/error").expect_failure().await;
        response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
