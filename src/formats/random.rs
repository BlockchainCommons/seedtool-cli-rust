use anyhow::Result;

use super::{Format, InputFormat};
use crate::{cli::Cli, seed::Seed};

pub struct RandomFormat;

impl Format for RandomFormat {
    fn name(&self) -> &str { "random" }

    fn round_trippable(&self) -> bool { true }
}

impl InputFormat for RandomFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        state.seed = Some(Seed::new(state.random_data(state.count)));
        Ok(state)
    }
}
