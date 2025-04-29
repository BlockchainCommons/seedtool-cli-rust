use anyhow::Result;
use bc_envelope::prelude::*;
use indoc::indoc;

mod common;
use common::*;

#[test]
fn test_seed() -> Result<()> {
    let seed = run_cli(&["--deterministic", "TEST"])?;
    run_cli_expect(&["--deterministic", "TEST"], &seed)?;
    assert_eq!(seed, "9d347f841a4e2ce6bc886e1aee74d824");
    Ok(())
}

#[test]
fn test_seed_2() -> Result<()> {
    let seed1 = run_cli(&[])?;
    let seed2 = run_cli(&[])?;
    assert_ne!(seed1, seed2);
    Ok(())
}

#[test]
fn test_formats() -> Result<()> {
    bc_envelope::register_tags();

    let hex = "9d347f841a4e2ce6bc886e1aee74d824";

    // Most format types are not round-trippable
    let base6 = "3123121543215241";
    assert_eq!(run_cli(&["--deterministic", "TEST", "--out", "base6", hex])?, base6);
    assert_eq!(
        run_cli(&["--in", "base6", "--out", "hex", base6])?,
        "cb97f8ff03b3434258a7a8974e3187a0"
    );

    let base10 = "6245132875418481";
    assert_eq!(run_cli(&["--deterministic", "TEST", "--out", "base10", hex])?, base10);
    assert_eq!(
        run_cli(&["--in", "base10", "--out", "hex", base10])?,
        "3f3830e7e4d4f95c3e037630c6ae811a"
    );

    let bits = "1001000111001010";
    assert_eq!(run_cli(&["--deterministic", "TEST", "--out", "bits", hex])?, bits);
    assert_eq!(
        run_cli(&["--in", "bits", "--out", "hex", bits])?,
        "980947e4f8cd49459819d9453fca085f"
    );

    let cards = "6hjckdah6c4dtc8skh2htd6ctsjd5s8c";
    assert_eq!(run_cli(&["--deterministic", "TEST", "--out", "cards", hex])?, cards);
    assert_eq!(
        run_cli(&["--in", "cards", "--out", "hex", cards])?,
        "1d0f2f3b502256cf56e3eaaa9f95ef71"
    );

    let dice = "4234232654326352";
    assert_eq!(run_cli(&["--deterministic", "TEST", "--out", "dice", hex])?, dice);
    assert_eq!(
        run_cli(&["--in", "dice", "--out", "hex", dice])?,
        "eefa19b88c5846e71fcb52d007066ae4"
    );

    let ints = "6 2 4 5 1 3 2 8 7 5 4 1 8 4 8 1";
    assert_eq!(run_cli(&["--deterministic", "TEST", "--out", "ints", hex])?, ints);
    assert_eq!(
        run_cli(&["--in", "ints", "--out", "hex", ints])?,
        "19a7830e032c0e027d176162112ee67e"
    );

    // Hex is round-trippable
    assert_eq!(run_cli(&["--in", "hex", "--out", "hex", hex])?, hex);

    // The Bytewords formats are round-trippable
    let btwm = "nteelblrcygldwvarflojtcywyjytpdklddyoymk";
    assert_eq!(run_cli(&["--in", "hex", "--out", "btwm", hex])?, btwm);
    assert_eq!(run_cli(&["--in", "btwm", "--out", "hex", btwm])?, hex);

    let btw =
        "next edge lamb liar city girl draw visa roof logo jolt city waxy jury trip dark loud duty obey monk";
    assert_eq!(run_cli(&["--in", "hex", "--out", "btw", hex])?, btw);
    assert_eq!(run_cli(&["--in", "btw", "--out", "hex", btw])?, hex);

    let btwu =
        "next-edge-lamb-liar-city-girl-draw-visa-roof-logo-jolt-city-waxy-jury-trip-dark-loud-duty-obey-monk";
    assert_eq!(run_cli(&["--in", "hex", "--out", "btwu", hex])?, btwu);
    assert_eq!(run_cli(&["--in", "btwu", "--out", "hex", btwu])?, hex);

    Ok(())
}

#[test]
fn test_envelope() -> Result<()> {
    let ur_string =
        "ur:envelope/lptpsogdnteelblrcygldwvarflojtcywyjytpdkoyadcsspoyaatpsojoghisinjkcxinjkcxjyisihcxjtjljyihoybdtpsoisguihihieglhsjnihoybetpsosecyiyjzvsayehspswda";
    assert_eq!(
        run_cli(
            &[
                "--out",
                "envelope",
                "--name",
                "SeedName",
                "--note",
                "This is the note",
                "--date",
                "2024-06-15T01:02:00Z",
                "--deterministic",
                "TEST",
            ]
        )?,
        ur_string
    );

    let renamed =
        "ur:envelope/lptpsogdnteelblrcygldwvarflojtcywyjytpdkoyadcsspoyaatpsojoghisinjkcxinjkcxjyisihcxjtjljyihoybdtpsokpfyhsjpjecxgdkpjpjojzihcxfpjskphscxgsjlkoihoybetpsosecyiyjzvsayaoiehshl";
    assert_eq!(
        run_cli(
            &["--in", "envelope", "--out", "envelope", "--name", "Dark Purple Aqua Love", ur_string]
        )?,
        renamed
    );

    let envelope = Envelope::from_ur_string(renamed)?;
    // println!("{}", envelope.format());
    #[rustfmt::skip]
    assert_eq!(envelope.format(), (indoc! {r#"
        Bytes(16) [
            'isA': 'Seed'
            'date': 2024-06-15T01:02:00Z
            'name': "Dark Purple Aqua Love"
            'note': "This is the note"
        ]
    "#}).trim());

    Ok(())
}

#[test]
fn test_sskr() -> Result<()> {
    let seed_envelope = run_cli(
        &["--name", "SeedName", "--note", "This is the note", "--date", "now", "--out", "envelope"]
    )?;
    let share_strings: Vec<String> = run_cli(
        &[
            "--in",
            "envelope",
            "--out",
            "sskr",
            "--group-threshold",
            "2",
            "--groups",
            "2-of-3",
            "3-of-5",
            "--",
            &seed_envelope,
        ]
    )?
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let selected_indexes: Vec<usize> = vec![0, 2, 3, 5, 7];
    let selected_share_strings: Vec<String> = share_strings
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if selected_indexes.contains(&i) { Some(s.clone()) } else { None }
        })
        .collect();

    let restored_envelope_ur_string = run_cli(
        &["--in", "sskr", "--out", "envelope", &selected_share_strings.join(" ")]
    )?;

    let restored_envelope = Envelope::from_ur_string(restored_envelope_ur_string)?;
    assert_eq!(restored_envelope.ur_string(), seed_envelope);

    Ok(())
}

#[test]
fn test_multipart() -> Result<()> {
    let seed_envelope = run_cli(
        &[
            "--count",
            "64",
            "--name",
            "SeedName",
            "--note",
            "This is the note",
            "--date",
            "now",
            "--out",
            "envelope",
        ]
    )?;
    let shares: Vec<String> = run_cli(
        &[
            "--in",
            "envelope",
            "--out",
            "multipart",
            "--max-fragment-len",
            "20",
            "--additional-parts",
            "50",
            &seed_envelope,
        ]
    )?
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let selected_shares: Vec<String> = shares.iter().skip(5).cloned().collect();

    let restored_envelope_ur_string = run_cli(
        &["--in", "multipart", "--out", "envelope", &selected_shares.join(" ")]
    )?;

    let restored_envelope = Envelope::from_ur_string(restored_envelope_ur_string)?;
    assert_eq!(restored_envelope.ur_string(), seed_envelope);

    Ok(())
}
