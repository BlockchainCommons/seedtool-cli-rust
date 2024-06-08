use anyhow::Result;

use crate::{cli::Cli, seed::Seed};

use super::InputFormat;

pub struct RandomFormat;

impl InputFormat for RandomFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        state.seed = Some(Seed::new(state.random_data(state.count)));
        Ok(state)
    }
}
