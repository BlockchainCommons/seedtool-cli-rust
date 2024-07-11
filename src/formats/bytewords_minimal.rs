use crate::{ cli::Cli, seed::Seed };
use anyhow::Result;
use bc_ur::prelude::*;

use super::{ Format, InputFormat, OutputFormat };

pub struct BytewordsMinimalFormat;

impl Format for BytewordsMinimalFormat {
    fn name(&self) -> &str {
        "btwm"
    }

    fn round_trippable(&self) -> bool {
        true
    }
}

impl InputFormat for BytewordsMinimalFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        state.seed = Some(
            Seed::new(bytewords::decode(&state.expect_input()?, bytewords::Style::Minimal)?)
        );
        Ok(state)
    }
}

impl OutputFormat for BytewordsMinimalFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        Ok(bytewords::encode(state.expect_seed().data(), bytewords::Style::Minimal))
    }
}
