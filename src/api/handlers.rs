// src/api/handlers.rs

use actix_web::{HttpResponse, Responder};
use crate::core::response_generator::ResponseGenerator;
use serde_json::json;

pub async fn handle_eligibility_check() -> impl Responder {
    // Use the dental-specific generator.
    let response = ResponseGenerator::generate_dental();
    HttpResponse::Ok().json(json!(response))
}
