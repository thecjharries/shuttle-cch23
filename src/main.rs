use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn zero_day_error() -> Result<String, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn day_one(Path(params): Path<HashMap<String, String>>) -> Result<String, StatusCode> {
    println!("{:?}", params);
    if let Some(numbers) = params.get("numbers") {
        let mut numbers = numbers
            .split('/')
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if 20 < numbers.len() {
            return Err(StatusCode::BAD_REQUEST);
        }
        let mut accumulator = numbers.pop().unwrap();
        while 0 < numbers.len() {
            accumulator ^= numbers.pop().unwrap();
        }
        return Ok(format!("{}", accumulator.pow(3)));
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
}

async fn day_four_strength(Json(payload): Json<Vec<Reindeer>>) -> Result<String, StatusCode> {
    println!("{:?}", payload);
    let strength_sum = payload.iter().fold(0, |acc, r| acc + r.strength);
    Ok(format!("{}", strength_sum))
}

async fn build_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(zero_day_error))
        .route("/1/*numbers", get(day_one))
        .route("/4/strength", post(day_four_strength))
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
    async fn test_day_four_strength() {
        let server = TestServer::new(build_router().await).unwrap();
        let response = server
            .post("/4/strength")
            .json(
                // { "name": "Dasher", "strength": 5 },
                // { "name": "Dancer", "strength": 6 },
                // { "name": "Prancer", "strength": 4 },
                // { "name": "Vixen", "strength": 7 }
                &vec![
                    Reindeer {
                        name: "Dasher".to_string(),
                        strength: 5,
                    },
                    Reindeer {
                        name: "Dancer".to_string(),
                        strength: 6,
                    },
                    Reindeer {
                        name: "Prancer".to_string(),
                        strength: 4,
                    },
                    Reindeer {
                        name: "Vixen".to_string(),
                        strength: 7,
                    },
                ],
            )
            .expect_success()
            .await;
        assert_eq!("22", response.text());
    }

    #[tokio::test]
    async fn test_day_one() {
        let server = TestServer::new(build_router().await).unwrap();
        let response = server.get("/1/10").expect_success().await;
        assert_eq!("1000", response.text());
        let response = server.get("/1/4/5/8/10").expect_success().await;
        assert_eq!("27", response.text());
        server.get("/1").expect_failure().await;
        let response = server
            .get("/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1")
            .expect_success()
            .await;
        assert_eq!("0", response.text());
        server
            .get("/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1/1")
            .expect_failure()
            .await;
    }

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
