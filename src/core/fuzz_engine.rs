// src/core/fuzz_engine.rs

use crate::models::response::EligibilityResponse;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct FuzzEngine;

impl FuzzEngine {
    /// Fuzz the response by applying one of several strategies.
    pub fn fuzz_response(mut response: EligibilityResponse) -> EligibilityResponse {
        // Define fuzzing strategies with labels and corresponding functions.
        let strategies: Vec<(&str, fn(&mut EligibilityResponse))> = vec![
            ("MISSING_FIELD", Self::omit_benefit_information),
            ("MALFORMED_DATA", Self::malform_control_number),
            ("TRACE_ERROR", Self::alter_trace_id),
        ];

        // Choose a strategy using weighted probabilities.
        let chosen = strategies
            .choose_weighted(&mut rand::thread_rng(), |item| match item.0 {
                "MISSING_FIELD" => 1,
                "MALFORMED_DATA" => 3,
                "TRACE_ERROR" => 1,
                _ => 1,
            })
            .unwrap();

        (chosen.1)(&mut response);
        response
    }

    fn omit_benefit_information(response: &mut EligibilityResponse) {
        response.benefit_information.clear();
    }

    fn malform_control_number(response: &mut EligibilityResponse) {
        response.control_number = "!@#$%^&*()".to_string();
    }

    fn alter_trace_id(response: &mut EligibilityResponse) {
        let mut rng = rand::thread_rng();
        response.meta.trace_id = format!("ERR-{}", rng.gen_range(1000..9999));
    }
}
