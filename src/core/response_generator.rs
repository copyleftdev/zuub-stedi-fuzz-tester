// src/core/response_generator.rs

use crate::models::response::{EligibilityResponse, MetaData, PlanStatus, BenefitInfo};
use crate::models::request::{Subscriber, Provider};
use chrono::NaiveDate;
use fake::Fake;
use fake::faker::name::raw::{FirstName, LastName};
use fake::faker::company::raw::CompanyName;
use fake::locales::EN;
use rand::Rng;

pub struct ResponseGenerator;

impl ResponseGenerator {
    /// Generate a dental-specific eligibility response with an expanded set of dental parameters.
    pub fn generate_dental() -> EligibilityResponse {
        let mut rng = rand::thread_rng();
        
        // Generate subscriber data with realistic names and date of birth.
        let first_name: String = FirstName(EN).fake();
        let last_name: String = LastName(EN).fake();
        let year: i32 = rng.gen_range(1950..2000);
        let month: u32 = rng.gen_range(1..13);
        let day: u32 = rng.gen_range(1..29);
        let dob = NaiveDate::from_ymd_opt(year, month, day)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(1980, 1, 1).unwrap())
            .format("%Y%m%d")
            .to_string();
        
        let subscriber = Subscriber {
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            member_id: rng.gen_range(10000000..100000000).to_string(),
            date_of_birth: dob,
        };
        
        let provider = Provider {
            organization_name: CompanyName(EN).fake(),
            npi: rng.gen_range(1000000000_i64..9999999999_i64).to_string(),
        };
        
        let meta = MetaData {
            sender_id: rng.gen_range(1000000000_i64..9999999999_i64).to_string(),
            submitter_id: rng.gen_range(1000000000_i64..9999999999_i64).to_string(),
            application_mode: "test".to_string(),
            trace_id: format!("ER-DENTAL-{}", rng.gen_range(1000..10000)),
        };
        
        // Assume a high likelihood of active dental coverage.
        let active = rng.gen_bool(0.85);
        let status_code = if active { "1".to_string() } else { "2".to_string() };
        let status = if active { "Dental Coverage Active".to_string() } else { "Dental Coverage Inactive".to_string() };
        
        // Define an expanded set of dental service type codes.
        let dental_service_codes = vec![
            "40".to_string(), "41".to_string(), "42".to_string(),
            "43".to_string(), "44".to_string(), "45".to_string(),
            "46".to_string(), "47".to_string(), "48".to_string(),
        ];
        
        // Build an expanded set of dental benefit information.
        let mut benefits = Vec::new();
        benefits.push(BenefitInfo {
            code: "DP".to_string(),
            name: "Preventive Dental Care".to_string(),
            benefit_amount: Some(rng.gen_range(100..500)),
            benefit_percent: None,
        });
        benefits.push(BenefitInfo {
            code: "DR".to_string(),
            name: "Restorative Dental Care".to_string(),
            benefit_amount: Some(rng.gen_range(500..1500)),
            benefit_percent: None,
        });
        benefits.push(BenefitInfo {
            code: "DC".to_string(),
            name: "Cosmetic Dental Care".to_string(),
            benefit_amount: Some(rng.gen_range(1000..3000)),
            benefit_percent: None,
        });
        benefits.push(BenefitInfo {
            code: "DE".to_string(),
            name: "Emergency Dental Care".to_string(),
            benefit_amount: Some(rng.gen_range(2000..4000)),
            benefit_percent: None,
        });
        benefits.push(BenefitInfo {
            code: "DI".to_string(),
            name: "Dental Implants".to_string(),
            benefit_amount: Some(rng.gen_range(3000..6000)),
            benefit_percent: None,
        });
        
        // Optionally add an orthodontic benefit with a 50% chance.
        if rng.gen_bool(0.5) {
            benefits.push(BenefitInfo {
                code: "DO".to_string(),
                name: "Orthodontic Dental Care".to_string(),
                benefit_amount: Some(rng.gen_range(1500..3500)),
                benefit_percent: None,
            });
        }
        
        EligibilityResponse {
            meta,
            control_number: rng.gen_range(100000000..999999999).to_string(),
            trading_partner_service_id: rng.gen_range(10000..100000).to_string(),
            subscriber,
            provider,
            plan_status: vec![
                PlanStatus {
                    status_code,
                    status,
                    service_type_codes: dental_service_codes,
                },
            ],
            benefit_information: benefits,
        }
    }
}
