use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EligibilityRequest {
    pub control_number: String,
    pub trading_partner_service_id: String,
    pub external_patient_id: String,
    pub subscriber: Subscriber,
    pub provider: Provider,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscriber {
    pub first_name: String,
    pub last_name: String,
    pub member_id: String,
    pub date_of_birth: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub organization_name: String,
    pub npi: String,
}
