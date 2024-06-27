use crate::{cli::Cli, seed::Seed};
use anyhow::{bail, Result};
use bc_components::{sskr_generate, SSKRShare, SymmetricKey, tags};
use bc_envelope::prelude::*;
use clap::ValueEnum;
use sskr::{Secret, Spec};

use super::{ InputFormat, OutputFormat };

pub struct SSKRFormat;

impl InputFormat for SSKRFormat {
    fn process_input(&self, mut state: Cli) -> Result<Cli> {
        let input = state.expect_input()?;
        state.seed = Some(parse_sskr_seed(&input)?);
        Ok(state)
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SSKRFormatKey {
    Envelope,
    Btw,
    Btwm,
    Btwu,
    Ur,
}

impl OutputFormat for SSKRFormat {
    fn process_output(&self, state: Cli) -> Result<String> {
        let spec = state.sskr_spec()?;
        let seed = state.expect_seed();
        let format = state.sskr_format;
        output_sskr_seed(seed, &spec, &format)
    }
}

//
// Output Helpers
//

fn output_sskr_seed(seed: &Seed, spec: &Spec, format: &SSKRFormatKey) -> Result<String> {
    match format {
        SSKRFormatKey::Envelope => {
            let envelope = seed.to_envelope();
            let content_key = SymmetricKey::new();
            let encrypted_envelope = envelope.wrap_envelope().encrypt_subject(&content_key)?;
            let share_envelopes = encrypted_envelope.sskr_split_flattened(spec, &content_key)?;
            let share_envelopes_strings = share_envelopes.iter().map(|envelope| envelope.ur_string()).collect::<Vec<_>>();
            Ok(share_envelopes_strings.join("\n"))
        },
        SSKRFormatKey::Btw => {
            make_bytewords_shares(spec, seed, bytewords::Style::Standard)
        },
        SSKRFormatKey::Btwm => {
            make_bytewords_shares(spec, seed, bytewords::Style::Minimal)
        },
        SSKRFormatKey::Btwu => {
            make_bytewords_shares(spec, seed, bytewords::Style::Uri)
        },
        SSKRFormatKey::Ur => {
            let shares = make_shares(spec, seed)?
                .iter()
                .map(|share| {
                    UR::new("sskr", CBOR::to_byte_string(share.data()))
                    .map(|ur| ur.string())
                })
                .collect::<Result<Vec<_>>>()?
                .join("\n");
            Ok(shares)
        },
    }
}

fn make_shares(spec: &sskr::Spec, seed: &Seed) -> Result<Vec<SSKRShare>> {
    let secret = Secret::new(seed.data())?;
    let shares = sskr_generate(spec, &secret)?.into_iter().flatten().collect();
    Ok(shares)
}

fn make_bytewords_shares(spec: &sskr::Spec, seed: &Seed, style: bytewords::Style) -> Result<String> {
    let shares = make_shares(spec, seed).unwrap();
    let cbor_shares = shares.iter().map(|share| CBOR::to_tagged_value(tags::SSKR_SHARE, CBOR::to_byte_string(share.data()))).collect::<Vec<_>>();
    let shares_strings = cbor_shares.iter().map(|share| bytewords::encode(share.to_cbor_data(), style)).collect::<Vec<_>>();
    Ok(shares_strings.join("\n"))
}

//
// Input Helpers
//

fn parse_envelopes(input: &str) -> Result<Seed> {
    let share_strings: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let share_envelopes: Vec<Envelope> = share_strings.iter().map(Envelope::from_ur_string).collect::<Result<Vec<_>>>()?;
    let share_envelopes_refs: Vec<&Envelope> = share_envelopes.iter().collect();
    let recovered_envelope = Envelope::sskr_join(&share_envelopes_refs)?.unwrap_envelope()?;
    Seed::try_from(recovered_envelope)
}

fn from_untagged_cbor_shares(untagged_cbor_shares: Vec<CBOR>) -> Result<Seed> {
    let data_shares: Vec<Vec<u8>> = untagged_cbor_shares
        .into_iter().map(|cbor| cbor.try_into_byte_string())
        .collect::<Result<Vec<_>>>()?;
    let recovered_secret: Secret = sskr::sskr_combine(&data_shares)?;
    Ok(Seed::new(recovered_secret.data()))
}

fn from_tagged_cbor_shares(tagged_cbor_shares: Vec<CBOR>) -> Result<Seed> {
    let untagged_cbor_shares: Vec<CBOR> = tagged_cbor_shares
        .into_iter().map(|cbor| cbor.try_into_expected_tagged_value(tags::SSKR_SHARE))
        .collect::<Result<Vec<_>>>()?;
    from_untagged_cbor_shares(untagged_cbor_shares)
}

fn parse_bytewords(input: &str, style: bytewords::Style) -> Result<Seed> {
    // Standard bytewords include spaces, so we can only split on newlines.
    let share_strings: Vec<String> = match style {
        bytewords::Style::Standard => input.split('\n').map(|s| s.to_string()).collect(),
        _ => input.split_whitespace().map(|s| s.to_string()).collect(),
    };
    let cbor_data_shares: Vec<Vec<u8>> = share_strings
        .iter().map(|s| bytewords::decode(s, style))
        .collect::<Result<Vec<_>>>()?;
    let tagged_cbor_shares: Vec<CBOR> = cbor_data_shares
        .into_iter().map(CBOR::try_from_data)
        .collect::<Result<Vec<_>>>()?;
    from_tagged_cbor_shares(tagged_cbor_shares)
}

fn parse_ur(input: &str, expected_tag: &Tag, allow_tagged_cbor: bool) -> Result<Seed> {
    let share_strings: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let urs: Vec<UR> = share_strings.iter().map(UR::from_ur_string).collect::<Result<Vec<_>>>()?;
    // ensure every UR is of the expected type
    for ur in &urs {
        ur.check_type(expected_tag.name().unwrap())?;
    }
    let untagged_cbor_shares: Vec<CBOR> = urs.into_iter().map(|ur| {
        // Legacy SSKR shares might have tagged CBOR, even though they're URs so they shouldn't be.
        let mut cbor = ur.cbor();
        if allow_tagged_cbor {
            if let Ok(untagged_cbor) = cbor.clone().try_into_expected_tagged_value(expected_tag.clone()) {
                cbor = untagged_cbor;
            }
        }
        Ok(cbor)
    }).collect::<Result<Vec<_>>>()?;
    from_untagged_cbor_shares(untagged_cbor_shares)
}

fn parse_sskr_seed(input: &str) -> Result<Seed> {
    if let Ok(seed) = parse_envelopes(input) {
        return Ok(seed);
    }

    if let Ok(seed) = parse_bytewords(input, bytewords::Style::Standard) {
        return Ok(seed);
    }

    if let Ok(seed) = parse_bytewords(input, bytewords::Style::Minimal) {
        return Ok(seed);
    }

    if let Ok(seed) = parse_bytewords(input, bytewords::Style::Uri) {
        return Ok(seed);
    }

    if let Ok(seed) = parse_ur(input, &tags::SSKR_SHARE, false) {
        return Ok(seed);
    }

    if let Ok(seed) = parse_ur(input, &tags::SSKR_SHARE_V1, true) {
        return Ok(seed);
    }

    bail!("Could not parse input as SSKR shares");
}

#[cfg(test)]
mod tests {
    use bc_rand::{RandomNumberGenerator, SecureRandomNumberGenerator};
    use dcbor::Date;
    use sskr::{GroupSpec, Spec};
    use hex_literal::hex;
    use indoc::indoc;

    use crate::seed::Seed;

    use super::{output_sskr_seed, parse_sskr_seed, SSKRFormatKey};

    fn test_format(format: &SSKRFormatKey, check_metadata: bool) {
        let mut rng = SecureRandomNumberGenerator;
        let seed = Seed::new_opt(rng.random_data(16), "SeedName", "This is the note.", Some(Date::from_string("2024-06-20").unwrap()));
        let spec = Spec::new(2, vec![GroupSpec::new(2, 3).unwrap(), GroupSpec::new(3, 5).unwrap()]).unwrap();

        let output = output_sskr_seed(&seed, &spec, format).unwrap();
        let share_strings = output.split('\n').map(|s| s.to_string()).collect::<Vec<_>>();
        let selected_indexes = [0, 2, 3, 5, 7];
        let selected_share_strings = share_strings.iter().enumerate().filter_map(|(i, s)| {
            if selected_indexes.contains(&i) {
                Some(s.clone())
            } else {
                None
            }
        }).collect::<Vec<_>>();
        let input = selected_share_strings.join("\n");
        println!("Input: {}", input);
        let recovered_seed = parse_sskr_seed(&input).unwrap();
        if check_metadata {
            assert_eq!(recovered_seed, seed);
        } else {
            assert_eq!(recovered_seed.data(), seed.data());
        }
    }

    #[test]
    fn test_formats() {
        test_format(&SSKRFormatKey::Envelope, true);
        test_format(&SSKRFormatKey::Btw, false);
        test_format(&SSKRFormatKey::Btwm, false);
        test_format(&SSKRFormatKey::Btwu, false);
        test_format(&SSKRFormatKey::Ur, false);
    }

    #[test]
    fn test_legacy() {
        let input = indoc!("ur:crypto-sskr/taadecgomymwbyadaenndtrehegwjkktoljphehtkshhbnhgiofmsebabs
            ur:crypto-sskr/taadecgomymwbyadaobthhluwlfsishthsnngapdckhytpoteeeeglwfcm
            ur:crypto-sskr/taadecgomymwbybgaekiplylurmhglfsgtfeptwnlrknvwidbztbjlhfht
            ur:crypto-sskr/taadecgomymwbybgaoswleqddlidjnehclnbdaaawdvsosiachtbihzees
            ur:crypto-sskr/taadecgomymwbybgaaeconwemnhhcmeotivdpdftknsptyltjntamtmtvs");
        let seed = parse_sskr_seed(input).unwrap();
        assert_eq!(seed.data().to_vec(), hex!("9d347f841a4e2ce6bc886e1aee74d824"));
    }
}
