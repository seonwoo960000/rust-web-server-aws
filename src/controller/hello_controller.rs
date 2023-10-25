use axum::Json;

pub async fn hello() -> Json<String> {
    Json("hello".to_string())
}
