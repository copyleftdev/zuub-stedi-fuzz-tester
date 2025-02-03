use crate::models::response::EligibilityResponse;

pub struct X12Encoder;

impl X12Encoder {
    pub fn encode(response: EligibilityResponse) -> String {
        format!(
            "ISA*00*          *00*          *ZZ*STEDI       *001*{0:01234}*{1}*{2}",
            response.meta.sender_id,
            response.meta.submitter_id,
            response.control_number
        )
    }
}
