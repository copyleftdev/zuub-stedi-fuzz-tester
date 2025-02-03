use actix_web::{HttpResponse, Responder};
use crate::core::response_generator::ResponseGenerator;
use crate::core::fuzz_engine::FuzzEngine;
use serde_json::json;

pub async fn handle_eligibility_check() -> impl Responder {
    let response = ResponseGenerator::generate();
    let fuzzed_response = FuzzEngine::fuzz_response(response);
    
    HttpResponse::Ok().json(json!(fuzzed_response))
}
