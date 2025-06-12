use anyhow::Result;

use super::{Format, InputFormat, OutputFormat};
use crate::{cli::Cli, seed::Seed};

pub struct HexFormat;

impl Format for HexFormat {
    fn name(&self) -> &str { "hex" }

    fn round_trippable(&self) -> bool { true }
}

impl InputFormat for HexFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        let input = state.expect_input()?;
        let seed = Seed::new(hex::decode(input)?);
        state.seed = Some(seed);
        Ok(state)
    }
}

impl OutputFormat for HexFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        Ok(hex::encode(state.expect_seed().data()))
    }
}
