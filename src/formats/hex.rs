use crate::{cli::Cli, seed::Seed};
use anyhow::Result;

use super::{ InputFormat, OutputFormat };

pub struct HexFormat;

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
