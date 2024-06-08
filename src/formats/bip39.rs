use crate::{ cli::Cli, seed::Seed };
use anyhow::Result;
use bip39::Mnemonic;
use bytes::Bytes;

use super::{ InputFormat, OutputFormat };

pub struct Bip39Format;

impl InputFormat for Bip39Format {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        let mnemonic = Mnemonic::parse_normalized(&state.expect_input()?)?;
        state.seed = Some(Seed::new(Bytes::from(mnemonic.to_entropy())));
        Ok(state)
    }
}

impl OutputFormat for Bip39Format {
    fn process_output(&self, state: Cli) -> Result<String> {
        let mnemonic = Mnemonic::from_entropy(state.expect_seed().data())?;
        let words = mnemonic.word_iter().collect::<Vec<&str>>().join(" ");
        Ok(words)
    }
}
