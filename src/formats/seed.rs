use anyhow::Result;
use bc_components::Seed as ComponentsSeed;
use bc_ur::prelude::*;

use super::{Format, InputFormat, OutputFormat};
use crate::{cli::Cli, seed::Seed};

pub struct SeedFormat;

impl Format for SeedFormat {
    fn name(&self) -> &str { "seed" }

    fn round_trippable(&self) -> bool { true }
}

impl InputFormat for SeedFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        let input = state.expect_input()?;
        let components_seed = ComponentsSeed::from_ur_string(&input)?;
        state.seed = Some(Seed::from(components_seed));
        Ok(state)
    }
}

impl OutputFormat for SeedFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        let seed = state.seed_with_overrides();
        let components_seed = ComponentsSeed::try_from(seed)?;
        Ok(components_seed.ur_string())
    }
}
