//! A tool for generating and transforming cryptographic seeds.
#![warn(rust_2018_idioms)]

#[doc(hidden)]
mod cli;
#[doc(hidden)]
mod exec;
#[doc(hidden)]
mod formats;
#[doc(hidden)]
mod random;
#[doc(hidden)]
mod seed;
#[doc(hidden)]
mod styles;
#[doc(hidden)]
mod util;

use anyhow::{Result, bail};
use bc_rand::SecureRandomNumberGenerator;
use clap::Parser;
use cli::{Cli, RngSource};
use formats::{select_input_format, select_output_format};
use random::DeterministicRandomNumberGenerator;

#[doc(hidden)]
fn main() -> Result<()> {
    bc_envelope::register_tags();

    let mut cli = Cli::parse();
    //println!("{:?}", cli);

    if let Some(deterministic) = &cli.deterministic {
        cli.rng = Some(RngSource::Deterministic(
            DeterministicRandomNumberGenerator::new_with_seed(deterministic),
        ));
    } else {
        cli.rng = Some(RngSource::Secure(SecureRandomNumberGenerator));
    }

    let input_format = select_input_format(cli.r#in);
    let output_format = select_output_format(cli.out);

    if !output_format.round_trippable() && input_format.name() != "random" {
        bail!(
            "Input for output form \"{}\" must be random.",
            output_format.name()
        );
    }

    cli = input_format.process_input(cli)?;
    let output = output_format.process_output(cli)?;
    println!("{}", output);

    Ok(())
}
