mod app_error;
mod fily;
// mod hyper;
// mod logy;
mod middleware;
// mod tracky;

use anyhow::Context;
use dotenv::dotenv;
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        // Configure formatting settings.
        .with_max_level(Level::DEBUG)
        .with_level(false)
        .with_thread_names(false)
        .with_target(false)
        .without_time()
        // Set the subscriber as the default.
        .init();

    // let tracky = tokio::spawn(tracky::run());
    let fily = tokio::spawn(fily::run());
    // let logy = tokio::spawn(logy::run());
    // let hypy = tokio::spawn(hyper::run());

    let _ = fily.await.context("failed staging fily")?;

    // let _ = join!(fily);

    // let (_, _, _, _) = join!(fily, logy, tracky, hypy);
    Ok(())
}
