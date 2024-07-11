use crate::{
    cli::Cli, random::deterministic_random, seed::Seed, util::{ data_to_ints, parse_ints }
};
use anyhow::Result;

use super::{ Format, InputFormat, OutputFormat };

pub struct IntsFormat;

impl Format for IntsFormat {
    fn name(&self) -> &str {
        "ints"
    }

    fn round_trippable(&self) -> bool {
        false
    }
}


impl InputFormat for IntsFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        let string = state.expect_input()?;
        let entropy = parse_ints(&string)?;
        state.seed = Some(Seed::new(deterministic_random(&entropy, state.count)));
        Ok(state)
    }
}

impl OutputFormat for IntsFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        data_to_ints(state.expect_seed().data(), state.low, state.high, " ")
    }
}
