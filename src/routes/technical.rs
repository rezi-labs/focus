use actix_web::{HttpResponse, Result, get};

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(""))
}
