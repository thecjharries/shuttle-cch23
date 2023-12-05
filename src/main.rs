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
struct StrengthReindeer {
    name: String,
    strength: i32,
}

async fn day_four_strength(
    Json(payload): Json<Vec<StrengthReindeer>>,
) -> Result<String, StatusCode> {
    println!("{:?}", payload);
    let strength_sum = payload.iter().fold(0, |acc, r| acc + r.strength);
    Ok(format!("{}", strength_sum))
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ContestReindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: u64,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn day_four_contest(
    Json(payload): Json<Vec<ContestReindeer>>,
) -> Result<Json<ContestResponse>, StatusCode> {
    println!("{:?}", payload);
    let mut fastest_score = i32::MAX;
    let mut fastest_name = String::new();
    let mut tallest_score = i32::MAX;
    let mut tallest_name = String::new();
    let mut magician_score = 0;
    let mut magician_name = String::new();
    let mut consumer_score = 0;
    let mut consumer_name = String::new();
    for r in payload {
        if r.strength < fastest_score {
            fastest_score = r.strength;
            fastest_name = r.name.clone();
        }
        if r.antler_width < tallest_score {
            tallest_score = r.antler_width;
            tallest_name = r.name.clone();
        }
        if r.snow_magic_power > magician_score {
            magician_score = r.snow_magic_power;
            magician_name = r.name.clone();
        }
        if r.candies > consumer_score {
            consumer_score = r.candies;
            consumer_name = r.name.clone();
        }
    }
    // {
    //   "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
    //   "tallest": "Dasher is standing tall with his 36 cm wide antlers",
    //   "magician": "Dasher could blast you away with a snow magic power of 9001",
    //   "consumer": "Dancer ate lots of candies, but also some grass"
    // }
    Ok(Json(ContestResponse {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest_score, fastest_name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest_name, tallest_score
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician_name, magician_score
        ),
        consumer: format!("{} ate lots of candies, but also some grass", consumer_name),
    }))
}

async fn build_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(zero_day_error))
        .route("/1/*numbers", get(day_one))
        .route("/4/strength", post(day_four_strength))
        .route("/4/contest", post(day_four_contest))
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
    async fn test_day_four_contest() {
        let server = TestServer::new(build_router().await).unwrap();
        let response = server
            .post("/4/contest")
            .json(
                // [
                // {
                //     "name": "Dasher",
                //     "strength": 5,
                //     "speed": 50.4,
                //     "height": 80,
                //     "antler_width": 36,
                //     "snow_magic_power": 9001,
                //     "favorite_food": "hay",
                //     "cAnD13s_3ATeN-yesT3rdAy": 2
                //   },
                //   {
                //     "name": "Dancer",
                //     "strength": 6,
                //     "speed": 48.2,
                //     "height": 65,
                //     "antler_width": 37,
                //     "snow_magic_power": 4004,
                //     "favorite_food": "grass",
                //     "cAnD13s_3ATeN-yesT3rdAy": 5
                //   }
                // ]
                &vec![
                    ContestReindeer {
                        name: "Dasher".to_string(),
                        strength: 5,
                        speed: 50.4,
                        height: 80,
                        antler_width: 36,
                        snow_magic_power: 9001,
                        favorite_food: "hay".to_string(),
                        candies: 2,
                    },
                    ContestReindeer {
                        name: "Dancer".to_string(),
                        strength: 6,
                        speed: 48.2,
                        height: 65,
                        antler_width: 37,
                        snow_magic_power: 4004,
                        favorite_food: "grass".to_string(),
                        candies: 5,
                    },
                ],
            )
            .expect_success()
            .await;
        // {
        //   "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
        //   "tallest": "Dasher is standing tall with his 36 cm wide antlers",
        //   "magician": "Dasher could blast you away with a snow magic power of 9001",
        //   "consumer": "Dancer ate lots of candies, but also some grass"
        // }
        let expected_result = ContestResponse {
            fastest: "Speeding past the finish line with a strength of 5 is Dasher".to_string(),
            tallest: "Dasher is standing tall with his 36 cm wide antlers".to_string(),
            magician: "Dasher could blast you away with a snow magic power of 9001".to_string(),
            consumer: "Dancer ate lots of candies, but also some grass".to_string(),
        };
        assert_eq!(
            serde_json::to_string(&expected_result).unwrap(),
            response.text()
        );
    }

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
                    StrengthReindeer {
                        name: "Dasher".to_string(),
                        strength: 5,
                    },
                    StrengthReindeer {
                        name: "Dancer".to_string(),
                        strength: 6,
                    },
                    StrengthReindeer {
                        name: "Prancer".to_string(),
                        strength: 4,
                    },
                    StrengthReindeer {
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
