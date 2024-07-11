use crate::{ cli::Cli, random::deterministic_random, seed::Seed, util::data_to_alphabet };
use anyhow::{ Result, bail };

use super::{ Format, InputFormat, OutputFormat };

pub struct CardsFormat;

impl Format for CardsFormat {
    fn name(&self) -> &str {
        "cards"
    }

    fn round_trippable(&self) -> bool {
        false
    }
}

impl InputFormat for CardsFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        let entropy = cards_to_data(&state.expect_input()?)?;
        let seed = Seed::new(deterministic_random(&entropy, state.count));
        state.seed = Some(seed);
        Ok(state)
    }
}

impl OutputFormat for CardsFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        Ok(data_to_alphabet(state.expect_seed().data(), 52, to_card))
    }
}

// Arrangement of cards per:
// https://github.com/iancoleman/bip39/blob/master/src/js/entropy.js
static CARD_SUITS: &str = "cdhs";
static CARD_RANKS: &str = "a23456789tjqk";

pub fn parse_rank(c: char) -> Result<usize> {
    let c = c.to_ascii_lowercase();
    for (i, rank) in CARD_RANKS.chars().enumerate() {
        if c == rank {
            return Ok(i);
        }
    }
    bail!("Invalid card rank. Allowed: [A,2-9,T,J,Q,K]");
}

pub fn parse_suit(c: char) -> Result<usize> {
    let c = c.to_ascii_lowercase();
    for (i, suit) in CARD_SUITS.chars().enumerate() {
        if c == suit {
            return Ok(i);
        }
    }
    bail!("Invalid card rank. Allowed: [D,C,H,S]");
}

pub fn cards_to_data(cards: &str) -> Result<Vec<u8>> {
    let len = cards.len();
    if len % 2 != 0 {
        bail!("Cards string must have even number of characters.");
    }
    let count = len / 2;
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        let rank = parse_rank(
            cards
                .chars()
                .nth(i * 2)
                .unwrap()
        )?;
        let suit = parse_suit(
            cards
                .chars()
                .nth(i * 2 + 1)
                .unwrap()
        )?;
        let n = suit * 13 + rank;
        result.push(n as u8);
    }

    Ok(result)
}

pub fn to_card(n: usize) -> String {
    assert!(n <= 51);
    let rank = n % 13;
    let suit = n / 13;
    let mut buf = String::new();
    buf.push(CARD_RANKS.chars().nth(rank).unwrap());
    buf.push(CARD_SUITS.chars().nth(suit).unwrap());

    // test value round trip
    let v = cards_to_data(&buf).unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], n as u8);

    buf
}
