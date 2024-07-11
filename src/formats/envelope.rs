use crate::{cli::Cli, seed::Seed};
use anyhow::Result;
use bc_envelope::prelude::*;

use super::{ Format, InputFormat, OutputFormat };

pub struct EnvelopeFormat;

impl Format for EnvelopeFormat {
    fn name(&self) -> &str {
        "envelope"
    }

    fn round_trippable(&self) -> bool {
        true
    }
}

impl InputFormat for EnvelopeFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        let string = state.expect_input()?;
        let envelope = Envelope::from_ur_string(string)?;
        state.seed = Some(Seed::try_from(envelope)?);
        Ok(state)
    }
}

impl OutputFormat for EnvelopeFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        Ok(state.to_envelope().ur_string())
    }
}
