use std::io::{self, Read};

use anyhow::Result;
use bc_components::{SSKRGroupSpec, SSKRSpec};
use bc_envelope::prelude::*;
use bc_rand::{RandomNumberGenerator, SecureRandomNumberGenerator};
use clap::Parser;
use clap_num::number_range;

use crate::{
    formats::{InputFormatKey, OutputFormatKey, SSKRFormatKey},
    random::DeterministicRandomNumberGenerator,
    seed::Seed,
    styles,
};

fn parse_low_int(s: &str) -> Result<usize, String> { number_range(s, 0, 254) }

fn parse_high_int(s: &str) -> Result<usize, String> { number_range(s, 1, 255) }

pub fn parse_group_threshold(s: &str) -> Result<usize, String> {
    number_range(s, 1, 16)
}

fn parse_date(s: &str) -> Result<Date, String> {
    if s == "now" {
        Ok(Date::now())
    } else {
        Date::from_string(s).map_err(|e| e.to_string())
    }
}

/// A tool for generating and transforming cryptographic seeds.
///
/// by Wolf McNally and Christopher Allen
///
/// Report bugs to ChristopherA@BlockchainCommons.com.
/// © 2024 Blockchain Commons.
#[derive(Debug, Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(styles = styles::get_styles())]
#[doc(hidden)]
pub struct Cli {
    /// The input to be transformed. If required and not present,
    /// it will be read from stdin.
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,

    /// The number of output units (hex bytes, base-10 digits, etc.)
    #[arg(short, long, default_value_t = 16)]
    pub count: usize,

    /// The input format.
    /// If not specified, a new random seed is generated using a secure random
    /// number generator.
    #[arg(
        value_enum,
        short,
        long,
        value_name = "INPUT_TYPE",
        default_value_t = InputFormatKey::Random,
    )]
    pub r#in: InputFormatKey,

    /// The output format.
    #[arg(
        value_enum,
        short,
        long,
        value_name = "OUTPUT_TYPE",
        default_value_t = OutputFormatKey::Hex,
    )]
    pub out: OutputFormatKey,

    /// The lowest int returned (0-254)
    #[arg(
        help_heading = Some("Integer Input and Output"),
        long,
        value_name = "LOW",
        default_value_t = 0
    )]
    #[clap(value_parser = parse_low_int)]
    pub low: usize,

    /// The highest int returned (1-255), low < high
    #[arg(
        help_heading = Some("Integer Input and Output"),
        long,
        value_name = "HIGH",
        default_value_t = 9
    )]
    #[clap(value_parser = parse_high_int)]
    pub high: usize,

    /// The name of the seed.
    #[arg(help_heading = Some("Metadata"), long, value_name = "NAME")]
    pub name: Option<String>,

    /// The note associated with the seed.
    #[arg(help_heading = Some("Metadata"), long, value_name = "NOTE")]
    pub note: Option<String>,

    /// The seed's creation date, in ISO-8601 format.
    /// May also be `now`.
    #[arg(help_heading = Some("Metadata"), long, value_name = "DATE")]
    #[clap(value_parser = parse_date)]
    pub date: Option<Date>,

    /// For `multipart` output, the UR will be segmented into parts with
    /// fragments no larger than MAX_FRAG_LEN
    #[arg(
        help_heading = Some("Multipart Encoding"),
        long,
        value_name = "MAX_FRAG_LEN",
        default_value_t = 500
    )]
    pub max_fragment_len: usize,

    /// For `multipart` output, the number of additional parts above the
    /// minimum to generate using fountain encoding.
    #[arg(
        help_heading = Some("Multipart Encoding"),
        long,
        value_name = "NUM_PARTS",
        default_value_t = 0
    )]
    pub additional_parts: usize,

    /// Group specifications.
    /// May appear more than once.
    /// M must be < N
    #[arg(
        help_heading = Some("SSKR Output"),
        short,
        long,
        value_name = "M-of-N",
        num_args = 1..16
    )]
    #[clap(value_parser = SSKRGroupSpec::parse)]
    pub groups: Vec<SSKRGroupSpec>,

    /// The number of groups that must meet their threshold.
    /// Must be <= the number of group specifications.
    #[arg(
        help_heading = Some("SSKR Output"),
        short = 't',
        long,
        value_name = "THRESHOLD",
        default_value_t = 1
    )]
    #[clap(value_parser = parse_group_threshold)]
    pub group_threshold: usize,

    /// Output format.
    #[arg(
        value_enum,
        help_heading = Some("SSKR Output"),
        value_name = "SSKR_FORMAT",
        short,
        long,
        default_value_t = SSKRFormatKey::Envelope,
    )]
    pub sskr_format: SSKRFormatKey,

    /// Use a deterministic random number generator with the given seed string.
    ///
    /// Output generated from this seed will be the same every time,
    /// so generated seeds are only as secure as the seed string.
    #[arg(
        help_heading = Some("Deterministic Random Numbers"),
        short,
        long,
        value_name = "SEED_STRING"
    )]
    pub deterministic: Option<String>,

    #[clap(skip)]
    pub seed: Option<Seed>,

    #[clap(skip)]
    pub rng: Option<RngSource>,
}

#[derive(Debug, Clone)]
pub enum RngSource {
    Secure(SecureRandomNumberGenerator),
    Deterministic(DeterministicRandomNumberGenerator),
}

impl Cli {
    pub fn expect_input(&self) -> Result<String> {
        if let Some(input) = &self.input {
            Ok(input.clone())
        } else {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            Ok(input.trim().to_string())
        }
    }

    pub fn expect_seed(&self) -> &Seed {
        self.seed.as_ref().expect("Seed not initialized")
    }

    pub fn random_data(&mut self, size: usize) -> Vec<u8> {
        match &mut self.rng {
            Some(RngSource::Secure(rng)) => rng.random_data(size),
            Some(RngSource::Deterministic(rng)) => {
                rng.deterministic_random_data(size)
            }
            None => panic!("RNG not initialized"),
        }
    }

    pub fn to_envelope(&self) -> Envelope {
        let mut seed = self.expect_seed().clone();
        if let Some(name) = &self.name {
            seed.set_name(name);
        }
        if let Some(note) = &self.note {
            seed.set_note(note);
        }
        if let Some(date) = &self.date {
            seed.set_creation_date(Some(date));
        }
        seed.into_envelope()
    }

    pub fn sskr_spec(&self) -> Result<SSKRSpec> {
        Ok(SSKRSpec::new(self.group_threshold, self.groups.clone())?)
    }
}
