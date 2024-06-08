use crate::{ cli::Cli, seed::Seed };
use anyhow::Result;
use bc_ur::prelude::*;

use super::{ InputFormat, OutputFormat };

pub struct BytewordsMinimalFormat;

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
