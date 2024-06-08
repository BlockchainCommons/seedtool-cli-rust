use crate::{cli::Cli, seed::Seed};
use anyhow::Result;
use bc_components::SymmetricKey;
use bc_envelope::prelude::*;

use super::{ InputFormat, OutputFormat };

pub struct SskrFormat;

impl InputFormat for SskrFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        let input = state.expect_input()?;
        let share_strings: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let share_envelopes: Vec<Envelope> = share_strings.iter().map(Envelope::from_ur_string).collect::<Result<Vec<_>>>()?;
        let share_envelopes_refs: Vec<&Envelope> = share_envelopes.iter().collect();
        let recovered_envelope = Envelope::sskr_join(&share_envelopes_refs)?.unwrap_envelope()?;
        state.seed = Some(Seed::try_from(recovered_envelope)?);
        Ok(state)
    }
}

impl OutputFormat for SskrFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        let spec = state.sskr_spec()?;
        let envelope = state.to_envelope();
        let content_key = SymmetricKey::new();
        let encrypted_envelope = envelope.wrap_envelope().encrypt_subject(&content_key)?;
        let share_envelopes = encrypted_envelope.sskr_split_flattened(&spec, &content_key)?;
        let share_envelopes_strings = share_envelopes.iter().map(|envelope| envelope.ur_string()).collect::<Vec<_>>();
        Ok(share_envelopes_strings.join("\n"))
    }
}
