use anyhow::{ Result, bail };
use bc_envelope::Envelope;
use bc_ur::{MultipartDecoder, MultipartEncoder, URDecodable, UREncodable};

use crate::{ cli::Cli, seed::Seed };

use super::{ Format, InputFormat, OutputFormat };

pub struct MultipartFormat;

impl Format for MultipartFormat {
    fn name(&self) -> &str {
        "multipart"
    }

    fn round_trippable(&self) -> bool {
        true
    }
}

impl InputFormat for MultipartFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        let input = state.expect_input()?;
        let shares: Vec<&str> = input.split_whitespace().collect();

        let mut decoder = MultipartDecoder::new();
        for share in shares {
            decoder.receive(share)?;
            if decoder.is_complete() {
                break;
            }
        }

        if !decoder.is_complete() {
            bail!("Insufficient SSKR shares");
        }
        let ur = decoder.message().unwrap().unwrap();
        let envelope = Envelope::from_ur(&ur)?;
        let seed = Seed::try_from(envelope)?;
        state.seed = Some(seed);
        Ok(state)
    }
}

impl OutputFormat for MultipartFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        let ur = state.to_envelope().ur();
        let mut encoder = MultipartEncoder::new(&ur, state.max_fragment_len)?;
        let parts_count = encoder.parts_count() + state.additional_parts;
        let parts = (0..parts_count)
            .map(|_| encoder.next_part().map_err(anyhow::Error::from))
            .collect::<Result<Vec<String>>>()?;
        Ok(parts.join("\n"))
    }
}
