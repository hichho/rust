use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub q1: String,
    pub q2: String,
}
#[derive(Serialize, Deserialize)]
struct ApiResponseData {
    pub json: ApiResponse,
}
pub async fn test_api(q1: String, q2: String) -> ApiResponse {
    let body = json!({"q1":q1,"q2":q2});
    let response = Request::post("https://echo.apifox.com/post")
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<ApiResponseData>()
        .await
        .unwrap();
    response.json
}
