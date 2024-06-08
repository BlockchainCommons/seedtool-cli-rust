use clap::ValueEnum;
use anyhow::Result;
use crate::cli::Cli;

use super::{
    RandomFormat,
    HexFormat,
    BytewordsStandardFormat,
    BytewordsUriFormat,
    BytewordsMinimalFormat,
    BitsFormat,
    CardsFormat,
    DiceFormat,
    Base6Format,
    Base10Format,
    IntsFormat,
    Bip39Format,
    SskrFormat,
    EnvelopeFormat,
    MultipartFormat,
};

pub trait InputFormat {
    fn process_input(&self, state: Cli) -> Result<Cli>;
}

pub trait OutputFormat {
    fn process_output(&self, _state: Cli) -> Result<String>;
}

// pub trait Format: InputFormat + OutputFormat { }

// impl<T> Format for T where T: InputFormat + OutputFormat { }

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputFormatKey {
    Random,
    Hex,
    Btw,
    Btwu,
    Btwm,
    Bits,
    Cards,
    Dice,
    Base6,
    Base10,
    Ints,
    Bip39,
    Sskr,
    Envelope,
    Multipart,
}

pub fn select_input_format(input_format: InputFormatKey) -> Box<dyn InputFormat> {
    match input_format {
        InputFormatKey::Random => Box::new(RandomFormat),
        InputFormatKey::Hex => Box::new(HexFormat),
        InputFormatKey::Btw => Box::new(BytewordsStandardFormat),
        InputFormatKey::Btwu => Box::new(BytewordsUriFormat),
        InputFormatKey::Btwm => Box::new(BytewordsMinimalFormat),
        InputFormatKey::Bits => Box::new(BitsFormat),
        InputFormatKey::Cards => Box::new(CardsFormat),
        InputFormatKey::Dice => Box::new(DiceFormat),
        InputFormatKey::Base6 => Box::new(Base6Format),
        InputFormatKey::Base10 => Box::new(Base10Format),
        InputFormatKey::Ints => Box::new(IntsFormat),
        InputFormatKey::Bip39 => Box::new(Bip39Format),
        InputFormatKey::Sskr => Box::new(SskrFormat),
        InputFormatKey::Envelope => Box::new(EnvelopeFormat),
        InputFormatKey::Multipart => Box::new(MultipartFormat),
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutputFormatKey {
    Hex,
    Btw,
    Btwu,
    Btwm,
    Bits,
    Cards,
    Dice,
    Base6,
    Base10,
    Ints,
    Bip39,
    Sskr,
    Envelope,
    Multipart,
}

pub fn select_output_format(output_format: OutputFormatKey) -> Box<dyn OutputFormat> {
    match output_format {
        OutputFormatKey::Hex => Box::new(HexFormat),
        OutputFormatKey::Btw => Box::new(BytewordsStandardFormat),
        OutputFormatKey::Btwu => Box::new(BytewordsUriFormat),
        OutputFormatKey::Btwm => Box::new(BytewordsMinimalFormat),
        OutputFormatKey::Bits => Box::new(BitsFormat),
        OutputFormatKey::Cards => Box::new(CardsFormat),
        OutputFormatKey::Dice => Box::new(DiceFormat),
        OutputFormatKey::Base6 => Box::new(Base6Format),
        OutputFormatKey::Base10 => Box::new(Base10Format),
        OutputFormatKey::Ints => Box::new(IntsFormat),
        OutputFormatKey::Bip39 => Box::new(Bip39Format),
        OutputFormatKey::Sskr => Box::new(SskrFormat),
        OutputFormatKey::Envelope => Box::new(EnvelopeFormat),
        OutputFormatKey::Multipart => Box::new(MultipartFormat),
    }
}
