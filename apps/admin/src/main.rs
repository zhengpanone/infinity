use std::process::ExitCode;
use std::sync::Arc;

use admin::{VERSION, server, startup, telemetry};
use infinity_error::Result;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            telemetry::report_fatal(&err);
            ExitCode::FAILURE
        }
    }
}

#[tokio::main]
async fn run() -> Result<()> {
    let config = startup::load_config()?;
    startup::init_logger(config)?;

    tracing::info!(
        version = VERSION,
        app = config.app.name.as_str(),
        "admin server starting"
    );
    tracing::info!(
        host = config.server.host.as_str(),
        port = config.server.port,
        "admin server config loaded"
    );

    let started = infinity_utils::time::now_millis();

    let elapsed = infinity_utils::time::now_millis() - started;
    tracing::info!(elapsed_ms = elapsed, "admin bootstrap complete");

    let db = Arc::new(startup::init_database(config).await?);

    server::serve_all(config, db).await?;
    Ok(())
}
