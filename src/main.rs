use zuub_stedi_fuzz_tester::config::settings::Settings;
use zuub_stedi_fuzz_tester::server::run_server;
use log::info;
use pretty_env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let settings = Settings::new().expect("Failed to load configuration");
    info!("Starting server on {}:{}", settings.server_address, settings.port);

    // Borrow the server address as &str.
    run_server(&settings.server_address, settings.port).await
}
