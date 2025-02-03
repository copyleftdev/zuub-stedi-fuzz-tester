use actix_web::{web, App, HttpServer};
use crate::api::handlers::handle_eligibility_check;

pub async fn run_server(address: &str, port: u16) -> std::io::Result<()> {
    let server_address = format!("{}:{}", address, port);
    
    HttpServer::new(|| {
        App::new()
            .route(
                "/change/medicalnetwork/eligibility/v3",
                web::post().to(handle_eligibility_check),
            )
    })
    .bind(server_address)?
    .run()
    .await
}
