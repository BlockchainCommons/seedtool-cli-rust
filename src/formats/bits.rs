use crate::{
    cli::Cli,
    random::sha256_deterministic_random_string,
    seed::Seed,
    util::{ data_to_ints, digits_to_data },
};
use anyhow::Result;

use super::{ InputFormat, OutputFormat };

pub struct BitsFormat;

impl InputFormat for BitsFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        // Compatibility with https://iancoleman.io/bip39/
        let string = state.expect_input()?;
        digits_to_data(&string, 0, 1)?; // syntax check only
        state.seed = Some(Seed::new(sha256_deterministic_random_string(&string, state.count)?));
        Ok(state)
    }
}

impl OutputFormat for BitsFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        data_to_ints(state.expect_seed().data(), 0, 1, "")
    }
}
