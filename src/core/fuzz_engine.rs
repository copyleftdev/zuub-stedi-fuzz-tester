use crate::models::response::EligibilityResponse;
use rand::{Rng, seq::SliceRandom};

pub struct FuzzEngine;

impl FuzzEngine {
    pub fn fuzz_response(mut response: EligibilityResponse) -> EligibilityResponse {
        let mut rng = rand::thread_rng();

        let errors = vec![
            ("MISSING_FIELD", "Omit a required field"),
            ("MALFORMED_DATA", "Inject garbled values"),
            ("RANDOMIZED_ERROR", "Introduce unknown error codes"),
        ];

        if let Some(error) = errors.choose(&mut rng) {
            match error.0 {
                "MISSING_FIELD" => {
                    response.benefit_information.clear();
                }
                "MALFORMED_DATA" => {
                    response.control_number = "!@#$%^&*()".to_string();
                }
                "RANDOMIZED_ERROR" => {
                    response.meta.trace_id = format!("ERR-{}", rng.gen_range(1000..9999));
                }
                _ => {},
            }
        }
        response
    }
}
