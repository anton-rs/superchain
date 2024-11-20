//! Contains the core `HiloDriver`.

use crate::{HiloDerivationPipeline, HiloExecutor, HiloExecutorConstructor, HiloPipeline};
use kona_driver::{Driver, PipelineCursor};

/// HiloDriver is a wrapper around the `Driver` that
/// provides methods of constructing the driver.
#[derive(Debug)]
pub struct HiloDriver {
    /// The driver instance.
    pub driver: Driver<HiloExecutor, HiloExecutorConstructor, HiloPipeline, HiloDerivationPipeline>,
}

impl HiloDriver {
    /// Constructs a new [HiloDriver].
    pub fn new(
        cursor: PipelineCursor,
        exec: HiloExecutorConstructor,
        pipeline: HiloPipeline,
    ) -> Self {
        Self { driver: Driver::new(cursor, exec, pipeline) }
    }

    /// Continuously run the [Driver].
    pub async fn start(&mut self) {
        todo!("upstream implementation of a continuous driver func")
    }
}
