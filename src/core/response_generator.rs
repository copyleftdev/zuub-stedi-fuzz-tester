use crate::models::response::{EligibilityResponse, MetaData, PlanStatus, BenefitInfo};
use crate::models::request::{Subscriber, Provider};
use crate::utils::randomizer::Randomizer;
use chrono::Utc;
use rand::seq::SliceRandom;

pub struct ResponseGenerator;

impl ResponseGenerator {
    pub fn generate() -> EligibilityResponse {
        let mut rng = rand::thread_rng();
        let status_code = ["1", "2"].choose(&mut rng).unwrap().to_string();
        let status = if status_code == "1" {
            "Active Coverage"
        } else {
            "Inactive Coverage"
        };

        EligibilityResponse {
            meta: MetaData {
                sender_id: Randomizer::generate_string(10),
                submitter_id: Randomizer::generate_string(10),
                application_mode: "test".to_string(),
                trace_id: format!("ER-{}", Utc::now().timestamp_millis()),
            },
            control_number: Randomizer::generate_string(9),
            trading_partner_service_id: Randomizer::generate_string(5),
            subscriber: Subscriber {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                member_id: Randomizer::generate_string(8),
                date_of_birth: "19800101".to_string(),
            },
            provider: Provider {
                organization_name: "ACME Health Services".to_string(),
                npi: Randomizer::generate_string(10),
            },
            plan_status: vec![
                PlanStatus {
                    status_code,
                    status: status.to_string(),
                    service_type_codes: vec!["30".to_string()],
                },
            ],
            benefit_information: vec![
                BenefitInfo {
                    code: "G".to_string(),
                    name: "Out of Pocket (Stop Loss)".to_string(),
                    benefit_amount: Some(5000),
                    benefit_percent: None,
                },
                BenefitInfo {
                    code: "C".to_string(),
                    name: "Deductible".to_string(),
                    benefit_amount: Some(1000),
                    benefit_percent: None,
                },
            ],
        }
    }
}
