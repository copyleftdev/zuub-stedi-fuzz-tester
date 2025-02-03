use serde::{Deserialize, Serialize};
// Import Subscriber and Provider from the request module.
use crate::models::request::{Subscriber, Provider};

#[derive(Debug, Serialize, Deserialize)]
pub struct EligibilityResponse {
    pub meta: MetaData,
    pub control_number: String,
    pub trading_partner_service_id: String,
    pub subscriber: Subscriber,
    pub provider: Provider,
    pub plan_status: Vec<PlanStatus>,
    pub benefit_information: Vec<BenefitInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub sender_id: String,
    pub submitter_id: String,
    pub application_mode: String,
    pub trace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanStatus {
    pub status_code: String,
    pub status: String,
    pub service_type_codes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenefitInfo {
    pub code: String,
    pub name: String,
    pub benefit_amount: Option<u32>,
    pub benefit_percent: Option<f32>,
}
