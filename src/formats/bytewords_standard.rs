use crate::{ cli::Cli, seed::Seed };
use anyhow::Result;
use bc_ur::prelude::*;

use super::{ InputFormat, OutputFormat };

pub struct BytewordsStandardFormat;

impl InputFormat for BytewordsStandardFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        state.seed = Some(
            Seed::new(bytewords::decode(&state.expect_input()?, bytewords::Style::Standard)?)
        );
        Ok(state)
    }
}

impl OutputFormat for BytewordsStandardFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        Ok(bytewords::encode(state.expect_seed().data(), bytewords::Style::Standard))
    }
}
