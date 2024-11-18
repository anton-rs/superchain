#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/anton-rs/hilo/issues/")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use clap::Parser;
use eyre::Result;

mod telemetry;

/// CLI Arguments.
#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct NodeArgs {
    /// The L2 chain ID to use.
    #[clap(long, short = 'c', default_value = "10", help = "The L2 chain ID to use")]
    pub l2_chain_id: u64,
    /// A port to serve prometheus metrics on.
    #[clap(
        long,
        short = 'm',
        default_value = "9090",
        help = "The port to serve prometheus metrics on"
    )]
    pub metrics_port: u16,
    // The hilo Rollup node configuration.
    // #[clap(flatten)]
    // pub hilo_config: HiloArgsExt,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments.
    let args = NodeArgs::parse();

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
