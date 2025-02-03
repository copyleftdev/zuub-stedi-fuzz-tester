use actix_web::{test, web, App};
use zuub_stedi_fuzz_tester::api::handlers;
use serde_json::Value;

#[actix_web::test]
async fn test_eligibility_check() {
    // Initialize an in-memory test service with the eligibility route.
    let app = test::init_service(
        App::new().route(
            "/change/medicalnetwork/eligibility/v3",
            web::post().to(handlers::handle_eligibility_check),
        ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/change/medicalnetwork/eligibility/v3")
        .set_json(&serde_json::json!({
            "controlNumber": "123456789",
            "tradingPartnerServiceId": "AHS",
            "externalPatientId": "UAa111222333",
            "subscriber": {
                "firstName": "Jane",
                "lastName": "Doe",
                "memberId": "123456789",
                "dateOfBirth": "19000101"
            },
            "provider": {
                "organization_name": "ACME Health Services",
                "npi": "1234567890"
            }
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Value = test::read_body_json(resp).await;
    assert!(body.get("control_number").is_some());
}
