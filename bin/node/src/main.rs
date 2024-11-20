#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/anton-rs/hilo/issues/")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod cli;
mod telemetry;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Parse arguments.
    use clap::Parser;
    let args = cli::NodeArgs::parse();

    // Initialize the telemetry stack.
    telemetry::init_stack(args.metrics_port)?;

    // info!(
    //     "Running the Hilo Node in Standalone mode. Attributes validation: {}",
    //     self.hera_config.validation_mode
    // );
    //
    // let cfg = self.hera_config.get_l2_config()?;
    // let driver = Driver::standalone(self.hera_config, cfg).await?;
    //
    // if let Err(e) = driver.start().await {
    //     bail!("[CRIT] Rollup driver failed: {:?}", e)
    // }

    Ok(())
}
