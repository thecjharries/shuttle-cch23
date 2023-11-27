use axum::http::StatusCode;
use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn zero_day_error() -> Result<String, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn build_router() -> Router {
    Router::new().route("/", get(hello_world))
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
}
