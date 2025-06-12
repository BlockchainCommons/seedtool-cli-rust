use anyhow::Result;
use bc_ur::prelude::*;

use super::{Format, InputFormat, OutputFormat};
use crate::{cli::Cli, seed::Seed};

pub struct BytewordsStandardFormat;

impl Format for BytewordsStandardFormat {
    fn name(&self) -> &str { "btw" }

    fn round_trippable(&self) -> bool { true }
}

impl InputFormat for BytewordsStandardFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        state.seed = Some(Seed::new(bytewords::decode(
            &state.expect_input()?,
            bytewords::Style::Standard,
        )?));
        Ok(state)
    }
}

impl OutputFormat for BytewordsStandardFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        Ok(bytewords::encode(
            state.expect_seed().data(),
            bytewords::Style::Standard,
        ))
    }
}
