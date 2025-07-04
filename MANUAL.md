 # 🌱 Seedtool-CLI-Rust Manual (v0.1.10)

## July 9, 2024

`seedtool-cli-rust` is a command-line tool that allows for the creation, import, and export of seeds as well as the export of randomizers that could be used elsewhere to create seeds.

You may want to use it if:

* You want to create a seed.
* You want to import a seed from another source.
* You want to translate a seed into a different form.
* You want to back up a seed using SSKR.
* You want to create a seed Envelope.

As a reference app, `seedtool-cli-rust` is largely intended as a demonstration of how seed-related functionality works, and what the best practices are for interacting with seeds.

> ⁉️ **Why** ⁉️ Asides throughout this document support best practices by talking about why you would use particular functions.

## Table of Contents

- [🌱 Seedtool-CLI-Rust Manual (v0.1.10)](#-seedtool-cli-rust-manual-v0110)
  - [July 9, 2024](#july-9-2024)
  - [Table of Contents](#table-of-contents)
  - [Installing Seedtool-CLI-Rust](#installing-seedtool-cli-rust)
  - [Generating Seeds](#generating-seeds)
    - [Generating Different Sizes of Seeds](#generating-different-sizes-of-seeds)
  - [Generating Seeds with Inputs](#generating-seeds-with-inputs)
    - [Generating Seeds with Card Draws](#generating-seeds-with-card-draws)
    - [Generating Seeds with Coin Flips](#generating-seeds-with-coin-flips)
    - [Generating Seeds with Dice Throws](#generating-seeds-with-dice-throws)
    - [Generating Seeds with Base 6 Numbers](#generating-seeds-with-base-6-numbers)
    - [Generating Seeds with Base 10 Numbers](#generating-seeds-with-base-10-numbers)
    - [Generating Seeds with 8-Bit Integers](#generating-seeds-with-8-bit-integers)
    - [Generating Seeds with Deterministic Strings (***Development Only***)](#generating-seeds-with-deterministic-strings-development-only)
  - [Restoring Seeds](#restoring-seeds)
    - [Restoring Seeds with BIP-39](#restoring-seeds-with-bip-39)
    - [Restoring Seeds with Bytewords](#restoring-seeds-with-bytewords)
    - [Restoring Seeds with Envelope](#restoring-seeds-with-envelope)
    - [Restoring Seeds with Hex](#restoring-seeds-with-hex)
    - [Restoring Seeds with SSKR](#restoring-seeds-with-sskr)
  - [Storing Seeds Using Classic Methods](#storing-seeds-using-classic-methods)
  - [Storing Seed Shares Using SSKR](#storing-seed-shares-using-sskr)
  - [Storing Seed Metadata Using Envelopes](#storing-seed-metadata-using-envelopes)
    - [Storing Envelopes as QRs](#storing-envelopes-as-qrs)
    - [Storing Envelopes in Multi-Parts](#storing-envelopes-in-multi-parts)
  - [Translating Seeds](#translating-seeds)
  - [Using the Envelope-CLI](#using-the-envelope-cli)
    - [Encrypting an Envelope](#encrypting-an-envelope)
    - [Adding Additional Content to an Envelope](#adding-additional-content-to-an-envelope)
    - [Encrypting \& Adding Additional Content with a Single Line](#encrypting--adding-additional-content-with-a-single-line)
    - [Signing an Envelope](#signing-an-envelope)
    - [Using Attachments](#using-attachments)
  - [Appendix I: Generating Randomizers (***Development Only***)](#appendix-i-generating-randomizers-development-only)

## Installing Seedtool-CLI-Rust

To use the Rust version of Seedtool, you will first need to install Rust on your computer (if it's not there already). The Rust developers suggest using [rustup](https://www.rust-lang.org/tools/install), but Rust may also be available through `brew`, `apt-get`, or other methodologies depending on your operating system and setup.

If you've previously installed the older [Swift version of Seedtool](https://github.com/BlockchainCommons/seedtool-cli), you may wish to preserve it as follows:

```
sudo mv /usr/local/bin/seedtool /usr/local/bin/cseedtool
```

You can now install the new Rust-based Seedtool.
It can be cloned and compiled from the [seedtool-cli-rust repo](https://github.com/BlockchainCommons/seedtool-cli-rust). However, the easiest methodology is simply to install the Rust crate.

> ⁉️ **Why Install or Use a Crate** ⁉️ Seedtool is intended as a reference app and if you're using it for reference purposes such as testing out your own code, the crate should be great. However, if you were ever to decide to use Seedtool to work with real seeds holding real funds, you might want to compile it yourself from the repo, as a crate can be opaque.

To install a crate:

```
cargo install seedtool-cli
```

You should now have `seedtool` in your Cargo path.

```
which seedtool

│ /Users/YourAccount/.cargo/bin/seedtool
```

## Generating Seeds

Seedtool can be used to generate seeds, which are the foundation of keys in modern cryptographic systems.

The easiest way to do this is to run `seedtool` with no argument. The hex code for the seed will be displayed by default.

```
seedtool

│ 8af129674470185bcc5eb492f35c27e1
```

> ⁉️ **Why Use Seedtool's Internal Randomizer** ⁉️ We think it's safe. And it's definitely sufficient for reference testing.

Note that in these and all other examples, you can alternatively save a generated seed into a variable, for additional use, as follows:

```
SEED=`seedtool`
echo $SEED

│ 8af129674470185bcc5eb492f35c27e1
```

### Generating Different Sizes of Seeds
The default seed generated by `seedtool` is 16 bytes, or 128 bits. This is usually considered sufficient entropy for most modern uses, particularly including Bitcoin, which only enjoys 128 bits of entropy once a public key has been revealed.

However, `seedtool` can produce larger seeds if desired, using the `-c` flag, which denotes the size of a seed in bytes. The following generates a 32-byte (256-bit) seed:

```
seedtool -c 32

│ 5117109e6b712ff9b4aa92c914de1d9713c1285f23baeba71ecf1c7eb17e5634
```

> ⁉️ **Why Create Larger Seeds** ⁉️ You usually wouldn't in the modern day. But, if you wanted to future-proof a seed and if you were using it on a network that doesn't implicitly limit the entropy of funds, like Bitcoin does, then you might.

## Generating Seeds with Inputs

By default `seedtool` generates seeds using its  internal randomization. You might want to instead generate a seed using your own entropy.

> ⁉️ **Why Create Your Own Entropy** ⁉️ Generating your own entropy can give you more trust that the randomization was truly random, that it couldn't be known by anyone else, and that it has sufficient entropy (provided that you give it enough!). You also might save your randomized data as a way to regenerate your seed, which is helpful for developmental work (but not necessarily secure for real usage).

`seedtool` offers the following methods for generating seeds from inputs, so that the same input can also be applied to get the same seed.

**Seedtool Generation Inputs:**
| Type          | Seed Inputs             | Argument  |
| ------------- | ----------------------- | --------- |
| Random        | Card Draws              | -i cards  |
| Random        | Coin Flips (0-1)        | -i bits   |
| Random        | Dice Throws (1-6)       | -i dice   |
| Random        | Base 6 Numbers (0-5)    | -i base6  |
| Random        | Base 10 Numbers (0-9)   | -i base10 |
| Random        | 1-Byte Integers (0-255) | -i ints   |
| Deterministic | String                  | -d        |

> ⁉️ **Why Choose One Entropy Creation Method Over Another** ⁉️ Choose a method where you will generate truly random inputs. For example, most people won't be willing to flip coins, as it just takes too many, and most folks won't have a way to randomly generate 1-byte numbers. The best compromise might be cards, because most folks have a deck, and you can generate enough entropy with a fairly small number of draws.

### Generating Seeds with Card Draws

Cards can be drawn from a standard deck, with replacement after each draw. You can then feed those card draws into `seedtool` to generate a seed based on that randomization.

```
seedtool -i cards 7htdad5s7s2d9d6djc8c9s4s7dks7cahth5c7hkd6d5dad
│ e403721f42bf14cefc4431ef8510a08b
```

Each card is represented by a value (`a23456789tjqk`) and a suit (`cdhs`), thus `7htdad` represents the draw of a seven of hearts, a ten of diamonds, and an ace of diamonds.

23 card draws are required to generate 128 bits of entropy, as each draw creates log2(52), or 5.7, bits of entropy.

### Generating Seeds with Coin Flips

Seeds can be generated from any binary number, which most practically means that they can be generated with a series of coin flips, with heads recorded as `1`s and tails as `0`s. That binary string is then fed into `seedtool`.

```
seedtool -i bits 11001101000111010101011100101000000010111000111000111010101001011110101010101101001001101111111001110100011010000001001011001111

│ 49df3120bfc288cff40d26234b42e797
```

Unsurprisingly, 128 coin flips are required to generate 128 bits of entropy, with each flip creating log2(2) bits of entropy, or 1.

### Generating Seeds with Dice Throws

Throwing dice is another way to easily generate entropy as the foundation of a seed. The `dice` input option for `seedtool` assumes a standard six-sided die, not a whacky polyhedron.
```
seedtool -i dice 16113161515626232546262253235213162163614621565126

│ 8304a9f1151b0bb241023635edcc88e4
```
As shown, this is just a string of numbers from 1-6, as displayed on the die. Each "d6" die throw creates log2(6) entropy or 2.6 bits, meaning 50 throws are required to generate 128 bits of entropy.

### Generating Seeds with Base 6 Numbers

Base 6 numbers work exactly like dice except when input they're numbered 0-5 instead of 1-6.
```
seedtool -i base6 05002050404414121435151142124102051052503510454015

│ 04dd1794b97a4cd22374d1cd10ccfcbc
```

### Generating Seeds with Base 10 Numbers

A base-10 number has digits valued 0-9.

```
seedtool -i base10 266136567024312947832916763620750122098

│ 4621c1d939953fe29674dfcf386c59de
```

Each digit generates log2(10) bits of entropy, or 3.3, so you'll want 39.

### Generating Seeds with 8-Bit Integers

The integers (`ints`) option for input allows you to input a string of space-separated integers valued at most between 0 and 255 (e.g., an 8-bit or 1-byte integer). Smaller ranges could be used with loss in entropy

```
seedtool -i ints

│ 127 16 38 146 19 226 10 87 168 194 0 187 119 18 48 172 200 216 236 203 106 137
│ 60fec8d5284cb1a2f16184ee362d05c1
```

A full one-byte integer obviously gives 8 bits of entropy, thus 16 are required to provide 128 bits of entropy. Use of smaller ranges will require a total of `128/log2(high-low+1)` numbers to generate 128 bits of entropy.

### Generating Seeds with Deterministic Strings (***Development Only***)

You can use the `-d` flag to generate a seed based on any arbitrary string. This is great for development work:

```
seedtool -d hellodarknessmyoldfriend

│ 448f303b9218698f84a26d3d4fb8e6c9
```

This method is labeled deterministic as opposed to random because presumably you are feeding it a chosen string (though obviously it's possible to randomize letters or even words). For that reason, and because phrases are more easily guessable than truly randomized methods of  seeding, it should be used _only for testing_.

## Restoring Seeds

Seeds can be stored in a variety of forms, including BIP-39, Bytewords, Envelopes, Hex, and SSKR. A seed can be restored from any of these forms by using the `-i` input argument.

**Seedtool Restore Inputs:**
| Type   | Seed Inputs | Argument                     |
| ------ | ----------- | ---------------------------- |
| Backup | BIP-39      | -i bip39                     |
| Backup | Bytewords   | -i btw<br>-i btwm<br>-i btwu |
| Backup | Envelope    | -i envelope                  |
| Backup | Hex         | -i hex                       |
| Backup | SSKR        | -i sskr                      |


All of the following restore examples use the default [128-byte Blockchain Commons seed (59f2293a5bce7d4de59e71b4207ac5d2)](https://developer.blockchaincommons.com/seed-128/).

### Restoring Seeds with BIP-39

[BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) is a Bitcoin standard for generating (and more importantly restoring) seeds through the use of mnemonic words. Most BIP39 backups are between 12 words (for a 128-bit seed) and 24 words (for a 256-bit seed).

```
seedtool -i bip39 "fly mule excess resource treat plunge nose soda reflect adult ramp planet"

│ 59f2293a5bce7d4de59e71b4207ac5d2
```

### Restoring Seeds with Bytewords

[Bytewords](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2020-012-bytewords.md) is Blockchain Commons' own mnemonic word list. The words are meant to be easier to remember and more standardized (with all entries being unique four-letter words that are also uniquely recognizable by their first and last letters). Bytewords is also the foundation of [URs](https://developer.blockchaincommons.com/ur/).

`seedtool` recognizes three formats for Bytewords: normal Bytewords (`btw`), URI Bytewords (`btwu`), and minimal Bytewords that only use the first and last letters (`btwm`).

```
seedtool -i btw "hawk whiz diet fact help taco kiwi gift view noon jugs quiz crux kiln silk tied omit keno lung jade"

│ 59f2293a5bce7d4de59e71b4207ac5d2

seedtool -i btwu hawk-whiz-diet-fact-help-taco-kiwi-gift-view-noon-jugs-quiz-crux-kiln-silk-tied-omit-keno-lung-jade

│ 59f2293a5bce7d4de59e71b4207ac5d2

seedtool -i btwm hkwzdtfthptokigtvwnnjsqzcxknsktdotkolgje

│ 59f2293a5bce7d4de59e71b4207ac5d2
```

### Restoring Seeds with Envelope

[Envelope](https://developer.blockchaincommons.com/envelope/) is a smart document system that allows for the storage of not just seeds, but also additional information.

Whatever is additionally included, as long as there's a basic Envelope with the expected formatting, it can be used to restore that seed:
```
seedtool -i envelope ur:envelope/lftpsogdhkwzdtfthptokigtvwnnjsqzcxknsktdoyadcsspgmgreefy

│ 59f2293a5bce7d4de59e71b4207ac5d2
```

### Restoring Seeds with Hex

Hex is a standard methodology for storing cryptocurrency seeds:

```
seedtool -i hex 59f2293a5bce7d4de59e71b4207ac5d2

│ 59f2293a5bce7d4de59e71b4207ac5d2
```

Though that particular command might not look that useful, it become more useful when you realize that you can use `seedtool` for [**Translating Seeds**](#Translating-Seeds).

### Restoring Seeds with SSKR

SSKR is covered more completely in ["Storing Shares Using SSKR"](#Storing-Shares-Using-SSKR). Once shares have been created, they may be simply restored with the `-i sskr` option, _no matter what format they were stored in_ (Bytewords, Envelope, or UR), provided that a threshold of shares is input. Just put each share on its own line.

Here's an example of restoring from Envelope shares:

```
seedtool -i sskr

│ ur:envelope/lftansfwlrhdcebapfrhecdalyaobwvyskbzasgaknylnseeuochihpfcpcwfhvalgpepfgsbajpuedmfmhennskluhsaydmgdcyvsmtnslbjobzknpdsejycwlpktzsdmhddatansfphdcxlultemsglryauraaesnblndnfglbihmsehtbfsehlsroptkgswdyvdpkmyhpwynnoyamtpsotantkphddachdeaeadaefxmwkgykchwdclylksrefzoycxrdmwzojksrdrfzkpfeykemckrkhefeientfxkblfgmfwtk
│ ur:envelope/lftansfwlrhdcebapfrhecdalyaobwvyskbzasgaknylnseeuochihpfcpcwfhvalgpepfgsbajpuedmfmhennskluhsaydmgdcyvsmtnslbjobzknpdsejycwlpktzsdmhddatansfphdcxlultemsglryauraaesnblndnfglbihmsehtbfsehlsroptkgswdyvdpkmyhpwynnoyamtpsotantkphddachdeaeadadprmocawntbpmglbwttrstorhytemrnvabaesdppypszczeyaaxheenjyolhfmhrysrolctyt
│ 59f2293a5bce7d4de59e71b4207ac5d2
```

Here's an example of restoring the same (default) seed using Byteword shares:

```
seedtool -i sskr

│ tuna next keep gyro acid yawn able acid able leaf idle mild legs play ugly atom liar slot scar film redo tent poem wasp maze calm scar need toil
│ tuna next keep gyro acid yawn able acid acid holy keep when luau cook jazz yank rock grim toil stub dice keys very ruby work crux peck down iron
│ 59f2293a5bce7d4de59e71b4207ac5d2
```

## Storing Seeds Using Classic Methods

**Seedtool Backup Outputs:**

| Type   | Seed Output | Argument                     |
| ------ | ----------- | ---------------------------- |
| Backup | BIP-39      | -o bip39                     |
| Backup | Bytewords   | -o btw<br>-o btwm<br>-o btwu |
| Backup | Hex         | -o hex                       |

Any seed in `seedtool` can be backed up by any of the simple, classic methods such as BIP39, Bytewords, and Hex. The following examples show five different randomly generated seeds being backed up via simple methods.

```
seedtool -o bip39

│ inflict staff public state aim unable purpose immune record order task naive

seedtool -o btw

│ main crux void safe aunt jolt meow ruby fish mild user tomb owls vast yank many undo blue tiny grim

seedtool -o btwm

│ bycxdmrobnfddkeyonfnadpflgfrgudipmgedyay

seedtool -o btwu

│ note-arch-news-jazz-axis-gift-draw-logo-guru-claw-toil-puff-jugs-surf-view-fizz-game-glow-cats-limp

seedtool -o hex

│ 5c4886bfa55d50ee11721b17b8deb26a
```

You could then read any of these backups in to reconstruct the seed:

```
seedtool -i bip39 "inflict staff public state aim unable purpose immune record order task naive"

│ 737a82b46a6055d8ab9b8db3d3877849
```

Obviously, these methods become more useful when you're actually backing up or translating a known seed, as discussed in [**Translating Seeds**](#Translating-Seeds).

> ⁉️ **Why Use One Classic Backup Method Over Another** ⁉️ Obviously, use whichever backup method can be read by the device or app you might restore your seed onto. We generally suggest Bytewords > BIP-39 > Hex because the mnemonic words tend to be less likely to lead to loss, especially Bytewords, which was specifically created to include distinct words that can't be mistaken for each other.

There are also two other output formats that are somewhat more complex: SSKR and Envelope.

## Storing Seed Shares Using SSKR

[SSKR](https://developer.blockchaincommons.com/sskr/) is Sharded Secret Key Reconstruction. It's a way to back up a seed into multiple shares, some (but often not all) of which may be used to restore the seed.

**Seedtool SSKR Backup Outputs:**

| Type              | Argument                                             |
| ----------------- | ---------------------------------------------------- |
| Backup            | -o sskr                                              |
| (Groups)          | -g m-of-n ...                                        |
| (Format)          | -s btw<br>-s btwm<br>-s btwu<br>-s envelope<br>-s ur |
| (Group Threshold) | -t #                                                 |

> ⁉️ **Why Use SSKR for Backup** ⁉️ There are two huge advantages to SSKR. First, security. No individual share is enough to reconstruct a seed. Second, resilience. As long as you are using a m-of-n methodology where m < n, you can afford to lose some shares ... and things _do_ sometimes get lost!

A [common methodology](https://github.com/BlockchainCommons/SmartCustody/blob/master/Docs/SSKR-Sharing.md) is to create SSKR shares with a [2-of-3 threshold](https://github.com/BlockchainCommons/SmartCustody/blob/master/Docs/SSKR-Sharing.md#2-of-3-shares), which means that the seed may be restored from any two of the three shares that are generated.

```
seedtool -o sskr -g 2-of-3

│ ur:envelope/lftansfwlrhdceetmoiemswswnplqdsepegytdgabkltbkrtmktbaxmygyteadiycwinsbgsurimdltpmyparpihskjpzslrgddwieaxbtktcxpehtylrdhlhybgkordaxhddatansfphdcxhpdyjyrdsomojyvsmueslymnfdfxntjlyacfmttikbytiyeopeolahplvtfyoladoyamtpsotantkphddawerlaeadaefepkgytigtfmtecnbaoyaybesbnykonnbavtisvolodygtflsektntsegwlakesbftftspon
│ ur:envelope/lftansfwlrhdceetmoiemswswnplqdsepegytdgabkltbkrtmktbaxmygyteadiycwinsbgsurimdltpmyparpihskjpzslrgddwieaxbtktcxpehtylrdhlhybgkordaxhddatansfphdcxhpdyjyrdsomojyvsmueslymnfdfxntjlyacfmttikbytiyeopeolahplvtfyoladoyamtpsotantkphddawerlaeadadpsykeslsswnlknrymklshnnsdiwnhnenbywtsaclkkftlflotshyuosflevlingttaplkney
│ ur:envelope/lftansfwlrhdceetmoiemswswnplqdsepegytdgabkltbkrtmktbaxmygyteadiycwinsbgsurimdltpmyparpihskjpzslrgddwieaxbtktcxpehtylrdhlhybgkordaxhddatansfphdcxhpdyjyrdsomojyvsmueslymnfdfxntjlyacfmttikbytiyeopeolahplvtfyoladoyamtpsotantkphddawerlaeadaolkbblykofzjenyaaesvwtpbwaygshttldyrtdilbjsdkspsawedactuyuefghfuojyfmkewd
```

As shown, `seedtool` stores its SSKR shares as Envelopes by default. However, output may instead be designated as `btw`, `btwm`, `btwu`, or [`ur`](https://developer.blockchaincommons.com/ur/) by using the `-s` flag.

```
seedtool -o sskr -g 2-of-3 -s btwm

│ tantkpgokkmwaeadaehldwetvsenhfihlrcylogslkhlaotksovovwbenl
│ tantkpgokkmwaeadaddmglwypehyvdfhntftylkkmerlndjttaecdaeheh
│ tantkpgokkmwaeadaorkvsmyiyvadlttrphtkodsrpmodnmtwlgynnnefd
```

SSKR also allows shares to be created in multiple groups, with each group having their own threshold and with a threshold of those groups required for reconstruction. The classic example of this is a 2-of-3 of Two of Three groups SSKR: nine shares are created, organized into three groups of three shares each, then the seed can be reconstructed from any two shares each from two different groups (four shares total). Our [SSKR scenarios #SmartCustody document](https://github.com/BlockchainCommons/SmartCustody/blob/master/Docs/SSKR-Sharing.md#2-of-3-shares-of-two-of-three-groups) discusses the option more fully.

To output SSKR shares of this type from `seedtool`, list multiple space-separated m-of-n groups after the `-g` argument, then add a `-t` argument to define how many groups must have their threshold met to meet the overall threshold for reconstruction of the seed.

The 2-of-3 of two of three groups example is thus:

```
seedtool -o sskr -g 2-of-3 2-of-3 2-of-3 -t 2
```

The following more complex example instead requires the threshold to be met from two groups of a 2-of-3 group, a 3-of-5 group, and a 1-of-2 group and also outputs the shares as `btwm`.

```
seedtool -o sskr -g 2-of-3 3-of-5 1-of-2 -t 2 -s btwm

│ tantkpgohllabgadaeptlkndfzrdwfihtdmtrpeovtsofnfxfzvwtypkot
│ tantkpgohllabgadadyllamsenpklgcfceoxdycyjtvwfsjetbjtspuetl
│ tantkpgohllabgadaobzmwlspsnybsntgowzoyhsvdmefmbwktfptolurp
│ tantkpgohllabgbgaeiowpzmcksndlrybaynflmdbdbybslocnatimrpsp
│ tantkpgohllabgbgadqdvajltsbswehpprnbttfmetptfxnbgegrjniyim
│ tantkpgohllabgbgaohplnmwpdinzehlsfmerhjlecdissdtiorduorlbe
│ tantkpgohllabgbgaxmylkaahspyfnrkjostdlssamneloadbaynuyiopr
│ tantkpgohllabgbgaabzuyjpmdjprerdcnesptmhbnbgtletskldkgmnzt
│ tantkpgohllabgcxaentdrecrfcxledmsklujndwmnmyurhkmnkihgenox
│ tantkpgohllabgcxadntdrecrfcxledmsklujndwmnmyurhkmnzswnzcvd

```
Here's what restoring would look like, taking two shares from the first (2-of-3) group and one from the last (1-of-2) group:
```
seedtool -i sskr

│ tantkpgohllabgadaeptlkndfzrdwfihtdmtrpeovtsofnfxfzvwtypkot
│ tantkpgohllabgadadyllamsenpklgcfceoxdycyjtvwfsjetbjtspuetl
│ tantkpgohllabgcxadntdrecrfcxledmsklujndwmnmyurhkmnzswnzcvd
│ 7042842963c788571776c4adfa4ed8df
```

## Storing Seed Metadata Using Envelopes

The final `seedtool` output format is [Gordian Envelope](https://developer.blockchaincommons.com/envelope/). Though a format such as hex or BIP-39 might be better for interoperability, the smart-document Envelope system is preferred for the secure storage of data.

**Seedtool Envelope Backup Outputs:**

| Type            | Argument                                                                  |
| --------------- | ------------------------------------------------------------------------- |
| Backup          | -o envelope                                                               |
| (Name)          | \-\-name name                                                             |
| (Creation Date) | \-\-date [ISO-8601 date](https://en.wikipedia.org/wiki/ISO_8601) or `now` |
| (Note)          | \-\-note note                                                             |

> ⁉️ **Why Use Envelope for Backup** ⁉️ The main reason to store in an Envelope is so that you encode other data, which might tell you more about what a seed is and what it's used for. However, an Envelope can also be useful because of the other manipulation you can do of an Envelope, such as encryption or elision.

The standard usage is simply to output with the `-o` argument:

```
seedtool -o envelope

│ ur:envelope/lftpsogddafdlrmygrismkwdoygmdkvswsetehproyadcsspetjogunn
```

One of the major advantages of storing a seed in an Envelope is that metadata may be included, such as the name (`--name`), creation date (`--date`), and notes (`--note`) about the seed. These are done with simple arguments.

The following example shows all of them being used:

```
seedtool -o envelope \
    --name "Blockchain Commons petty cash" \
    --date now \
    --note "Meant to hold no more than \$500, for payment of minor costs"

│ ur:envelope/lptpsogdnewfoxdskotbemhymofsuojefmfdhlryoyadcsspoybdtpsokscafwjzjliajeiaishsinjtcxfxjljnjnjljtjkcxjoihjyjykkcxiahsjkisoybetpsosezofptaoydihecszeaaoyaatpsoksfrgtihhsjtjycxjyjlcxisjljziecxjtjlcxjnjljpihcxjyishsjtcxdkecdydydwcxiyjljpcxjohskkjnihjtjycxjliycxjninjtjljpcxiajljkjyjkztsatshs
```

Even with all that extra information, `seedtool` will just restore the seed:

```
seedtool -i envelope ur:envelope/lptpsogdnewfoxdskotbemhymofsuojefmfdhlryoyadcsspoybdtpsokscafwjzjliajeiaishsinjtcxfxjljnjnjljtjkcxjoihjyjykkcxiahsjkisoybetpsosezofptaoydihecszeaaoyaatpsoksfrgtihhsjtjycxjyjlcxisjljziecxjtjlcxjnjljpihcxjyishsjtcxdkecdydydwcxiyjljpcxjohskkjnihjtjycxjliycxjninjtjljpcxiajljkjyjkztsatshs

│ 9ff3a42676d6375e923ddc6b3e485dbd
```

However the `envelope` CLI may be used to view the additional information, as further discussing in ["Using the Envelope-CLI"](#Using-the-Envelope-CLI).

```
│ envelope format ur:envelope/lptpsogdnewfoxdskotbemhymofsuojefmfdhlryoyadcsspoybdtpsokscafwjzjliajeiaishsinjtcxfxjljnjnjljtjkcxjoihjyjykkcxiahsjkisoybetpsosezofptaoydihecszeaaoyaatpsoksfrgtihhsjtjycxjyjlcxisjljziecxjtjlcxjnjljpihcxjyishsjtcxdkecdydydwcxiyjljpcxjohskkjnihjtjycxjliycxjninjtjljpcxiajljkjyjkztsatshs

│ Bytes(16) [
│     'isA': 'Seed'
│     'date': 2024-07-03T00:38:20Z
│     'name': "Blockchain Commons petty cash"
│     'note': "Meant to hold no more than $500, for payment of minor costs"
│ ]
```
That can give you crucial context when you're recovering a seed, telling you which seed it is, and what it was intended for.

### Storing Envelopes as QRs

One of the advantages of Blockchain Commons' [UR format](https://developer.blockchaincommons.com/ur/), (which Envelope is built atop) is that when formatted in all capital letters, it is very efficient for storage and transmittal in QR codes.

> ⁉️ **Why Use QRs to Encode Envelopes** ⁉️ QR-encoding allows for an Envelope to be printed for storage in a way that may be more robust and supports transmission across [airgaps](https://developer.blockchaincommons.com/airgap/).

To format an Envelope as a QR requires the use of software such as `qrencode`, which can be installed using [Homebrew on a Mac](https://formulae.brew.sh/formula/qrencode).

The following example shows the creation of a seed Envelope, its conversion to all-caps, and its storage as a QR.
```
$ seedtool -o envelope | tr '[:lower:]' '[:upper:]' | qrencode -o ~/seedtool-example-seed.png
```

Here's the QR:

![](https://raw.githubusercontent.com/BlockchainCommons/seedtool-cli-rust/master/images/seedtool-example-seed.png)


### Storing Envelopes in Multi-Parts

Envelopes can also be output in multiparts with the `-o multipart` argument. This takes advantage of UR's ability to split and sequence longer messages.

**Seedtool Envelope Multipart Backup Outputs:**
| Type             | Argument             |
| ---------------- | -------------------- |
| Multipart Backup | -o multipart         |
| (Fragment Size)  | --max-fragment-len # |
| (Add'l Fragment) | --additional-parts # |

> ⁉️ **Why Encode Envelopes with Multiparts** ⁉️ Because the data is too big! Primarily, because it's too big to fit in a readable QR.

If you use it in its default form, you'll probably get a single fragment:

```
seedtool -o multipart

│ ur:envelope/1-1/lpadadcscscyseeekpdshdcslftpsogdutcsmncedpnsjzutguwpzomhloesqzahoyadcsspztsomths
```

That's because the max-size of a fragment is set to 500 bytes by default. But, you can also use the `--max-fragment-len` argument to make fragments smaller and/or the `--additional-parts` to produce additional (redundant) fragments, using the [fountain code algorithms](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2024-001-multipart-ur.md) central to multipart URs.

Here's an example of reducing the fragment length to 20 bytes:

```
seedtool -o multipart --max-fragment-len 20

│ ur:envelope/1-2/lpadaocscscyutbwvttsgslftpsogdbzbncyroqddpbwpattnspkes
│ ur:envelope/2-2/lpaoaocscscyutbwvttsgsttasienljpgsaataoyadcsspditehllf
```

Here's doing so with two additional parts:

```
seedtool -o multipart --max-fragment-len 20 --additional-parts 2

│ ur:envelope/1-2/lpadaocscscyrfoxtsvdgslftpsogdjorfghoxbebzmtnstsaogleh
│ ur:envelope/2-2/lpaoaocscscyrfoxtsvdgsuoaalupantbejpdkoyadcsspamhlmyfw
│ ur:envelope/3-2/lpaxaocscscyrfoxtsvdgsuoaalupantbejpdkoyadcsspbykpwmlf
│ ur:envelope/4-2/lpaaaocscscyrfoxtsvdgslftpsogdjorfghoxbebzmtnsnslurhwn
```

> ⁉️ **Why Generate Additional Parts** ⁉️ The core fragments will be enough to piece together an Envelope. Additional parts allow an Envelope to be put back together even if some parts were missed. Yes, you could just keep repeating the original fragments, but the additional fragments use fountain codes, which are more efficient: you'll probably need fewer fragments and there's less concern about missing a particular fragment. The [MUR Guide](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2024-001-multipart-ur.md) explains more.

The main purpose of generating an Envelope as a multipart UR is to allow the generation of an [Animated QR](https://developer.blockchaincommons.com/animated-qrs/). Each of the UR fragments can be individually encoded as a QR, then they can be converted to an animated GIF (which is to say an animated QR) using a program such as [`imagemagick`](https://imagemagick.org/index.php).

## Translating Seeds

The above examples all showed `seedtool` either (1) generating seeds from random input; (2) restoring seeds from stored input; or (3) outputting a random seed in a variety of formats. However, these inputs and outputs may be mixed together, allowing `seedtool` to become a translator app for seeds.

Here's an example of generating a seed from cards and then storing it as minimal Bytewords:

```
seedtool -i cards 7htdad5s7s2d9d6djc8c9s4s7dks7cahth5c7hkd6d5dad \
    -o btwm

│ veaxjpctfwrsbbtoztfyehwslpbenbluykrolrwp
```

Here's an example of importing a seed from its hex and then outputting it as BIP-39:

```
seedtool -i hex 59f2293a5bce7d4de59e71b4207ac5d2 \
    -o bip39

│ fly mule excess resource treat plunge nose soda reflect adult ramp planet
```

Here's an example of converting that same (default) hex seed into `btwm`-format SSKR shares:

```
seedtool -i hex 59f2293a5bce7d4de59e71b4207ac5d2 \
    -o sskr -g 2-of-3 -s btwm

│ tantkpgowkftaeadaehhmnrkdrlybzwpdlyaaededllbckcnrtdnqznduy
│ tantkpgowkftaeadaddydttnwnfpsoptzopewnglprgylfahcnjoroctve
│ tantkpgowkftaeadaolruykkltcyrpiynshfytvebacnfsjlcawpkpbgne
```

Here's an example of using Envelope to add metadata notes to the default hex seed:

```
seedtool -i hex 59f2293a5bce7d4de59e71b4207ac5d2 \
    -o envelope \
    --note "This is the seed for mom's Bitcoin keys."

│ ur:envelope/lstpsogdhkwzdtfthptokigtvwnnjsqzcxknsktdoyaatpsoksdeghisinjkcxinjkcxjyisihcxjkihihiecxiyjljpcxjnjljndijkcxfwinjyiajlinjtcxjeihkkjkdmoyadcsspiadrfxts
```

Any of the `-i` inputs and `-o` outputs described to this point many be mixed together to translate as you see fit (or to backup an existing key or to output a random key in a specific format).

However note that this isn't true for the randomizer outputs in the [Appendix](#Appendix-I-Generating-Randomizers-Development-Only), which are allowed only with random inputs.

## Using the Envelope-CLI

`seedtool` is designed as a domain-specific application that focuses on one precise set of activities: reading, creating, and storing seeds. As such, its Envelope-related functionality is also limited to seed-related abilities, such as storing a name, creation date, and notes and outputting as either an Envelope or a set of SSKR envelopes.

However, Envelope can do much more. `seedtool` can be integrated into full Envelope functionality by also installing the [Rust-based Envelope CLI](https://github.com/BlockchainCommons/bc-envelope-cli-rust). This is most easily done with `cargo install`:

```
cargo install bc-envelope-cli
```

Using pipes or command-line variables, you can take your `seedtool`-generated seed Envelope and sign it, encrypt it, or otherwise add additional content.

The simplest example of this is to simply output an Envelope from `seedtool` and then format it (display it) with `envelope`:

```
seedtool -i hex 59f2293a5bce7d4de59e71b4207ac5d2 -o envelope \
    | envelope format

│ Bytes(16) [
│     'isA': 'Seed'
│ ]
```

The examples in the following section use the default seed with metadata:

```
ENVSEED=`seedtool -i hex 59f2293a5bce7d4de59e71b4207ac5d2 \
    -o envelope \
    --name "Blockchain Commons 128-Bit Default Seed" \
    --date now \
    --note "This is the 128-bit example key for Blockchain Commons. Do not use it for real funds\!"`
```

`envelope` displays it as follows:

```
envelope format $ENVSEED

│ Bytes(16) [
│     'isA': 'Seed'
│     'date': 2024-07-02T22:42:54Z
│     'name': "Blockchain Commons 128-Bit Default Seed"
│     'note': "This is the 128-bit example key for Blockchain Commons. Do not use it for real funds!"
│ ]
```

The examples also use the following keys:

```
SYM_KEY=`envelope generate key`
PRV_KEY=`envelope generate prvkeys`
PUB_KEY=`envelope generate pubkeys $PRV_KEY`
```

> ⁉️ **Is It Safe to Use Envelope to Generate Keys** ⁉️ It definitely is if you're doing reference/testing work. If you're generating seeds to create real privacy protections or to hold funds, definitely use something that you've checked out as safe (and definitely don't save them in command-line variables!).

### Encrypting an Envelope

An envelope's subject can be encrypted with a symmetric key:

```
echo $ENVSEED | envelope encrypt --key $SYM_KEY
```

Or if you prefer:

```
envelope subject type envelope $ENVSEED | envelope encrypt --key $SYM_KEY
```

Because assertions (which includes encryption) are applied to a subject, this modifies the existing Envelope so that the seed is no longer visible:

```
│ ENCRYPTED [
│     'isA': 'Seed'
│     'date': 2024-07-02T22:42:54Z
│     'name': "Blockchain Commons 128-Bit Default Seed"
│     'note': "This is the 128-bit example key for Blockchain Commons. Do not use it for real funds!"
│ ]
```
That _might_ be the right amount of information to retain, to make it easy to reference the seed while still securing it.

However, maybe even that is too much info. In order to encrypt the entirety of an Envelope, you "wrap" it, which basically embeds the whole previous Envelope as the subject of a new Envelope, and then encrypt that:

```

echo $ENVSEED \
    | envelope subject type wrapped \
    | envelope encrypt --key $SYM_KEY
```

The Envelope now only shows:

```
│ ENCRYPTED
```

### Adding Additional Content to an Envelope

You can add additional metadata to any Envelope. For example, after wrapping and encrypting an Envelope, you could add on a new note to help with the future decryption of the Envelope:

```
echo $ENVSEED \
    | envelope subject type wrapped \
    | envelope assertion add pred-obj known note string "This is encrypted with the July 2, 2024 sym key created on erzo." \
    | envelope encrypt --key $SYM_KEY
```

Which results in:

```
│ ENCRYPTED [
│     'note': "This is encrypted with the July 2, 2024 sym key created on erzo."
│ ]
```

### Encrypting & Adding Additional Content with a Single Line

These CLIs are all set up to take `stdin` and to output as `stdout`, so you can even connect _everything_ together into one big, lovely (messy) pipe:

```
seedtool -i hex 59f2293a5bce7d4de59e71b4207ac5d2 -o envelope --name "Blockchain Commons 128-Bit Default Seed" --date now --note "This is the 128-bit example key for Blockchain Commons. Do not use it for real funds\!" \
    | envelope subject type wrapped \
    | envelope assertion add pred-obj known note string "This is encrypted with the July 2, 2024 sym key created on erzo." \
    | envelope encrypt --key $SYM_KEY \
    | envelope format

│ ENCRYPTED [
│     'note': "This is encrypted with the July 2, 2024 sym key created on erzo."
│ ]
```

### Signing an Envelope

That encrypted and annotated Envelope could also be signed using Envelope's `sign` function:

```
echo $ENVSEED \
    | envelope subject type wrapped \
    | envelope assertion add pred-obj known note string "This is encrypted with the July 2, 2024 sym key created on erzo." \
    | envelope encrypt --key $SYM_KEY \
    | envelope sign --signer $PRV_KEY
```

This results in:

```
│ ENCRYPTED [
│     'note': "This is encrypted with the July 2, 2024 sym key created on erzo."
│     'signed': Signature
│ ]
```

Again, remember that every assertion (which, yes, also includes a signature) is applied to a subject, so in this example, the signing key is applied to the Encrypted envelope of the seed. Which might be what you want.

But, if you instead want to sign the note too, then you need to (again) wrap the Envelope and sign that:

```
SIGNED_SEED=`echo $ENVSEED \
    | envelope subject type wrapped \
    | envelope assertion add pred-obj known note string "This is encrypted with the July 2, 2024 sym key created on erzo." \
    | envelope encrypt --key $SYM_KEY \
    | envelope subject type wrapped \
    | envelope sign --signer $PRV_KEY`
```

This instead produces:

```
echo $SIGNED_SEED | envelope format

│ {
│     ENCRYPTED [
│         'note': "This is encrypted with the July 2, 2024 sym key created on erzo."
│     ]
│ } [
│     'signed': Signature
│ ]
```

Once signed with a private key, an Envelope's signature can be verified with the corresponding public key:

```
envelope verify --silent $SIGNED_SEED --verifier $PUB_KEY
```

Silent means that `envelope` only show a response if there's a problem. Here's what a problem would look like:

```
envelope verify --silent $SIGNED_SEED --verifier $NEW_PUB

│ Error: could not verify a signature
```

### Using Attachments

The above examples all demonstrate how to use Envelopes within Envelopes to take advantage of abilities to encrypt, annotate, and sign (or use other Envelope functionality).

Envelope also has an [attachment functionality](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2023-006-envelope-attachment.md) that allows vendors to create Envelopes using their own preferred formatting.

Say a vendor prefers Envelopes with BIP-39 words in them. You could create the BIP-39 words with `seedtool`:

```
BIP39_SEED=`seedtool -i hex \

│ 59f2293a5bce7d4de59e71b4207ac5d2 -o bip39`
```

Then embed that in an attachment:

```
envelope subject type string $BIP39_SEED \
    | envelope attachment create \
        --conforms-to "https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki" \
        "com.blockchaincommons"
```

[BCR-2023-006](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2023-006-envelope-attachment.md) has the full info on how this works.

This looks as follows:

```
│ 'attachment': {
│     "fly mule excess resource treat plunge nose soda reflect adult ramp planet"
│ } [
│     'conformsTo': "https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki"
│     'vendor': "com.blockchaincommons"
│ ]
```

Alternatively, perhaps a vendor just wants to have an attachment with specific information as part of a seed Envelope.

They could create an attachment:

```
SEED_ATTACH=`envelope subject type string "attachment data" \
    | envelope attachment create \
        --conforms-to "https://joe-bob-s.com/cheap-attachments.html" \
        "com.joe-bob-s"`
```

That attachment could then be added to the seed package:

```
echo $ENVSEED \
    | envelope subject type wrapped \
    | envelope encrypt --key $SYM_KEY \
    | envelope assertion add envelope $SEED_ATTACH \
    | envelope sign --prvkeys $PRV_KEY
```

Resulting in:

```
│ ENCRYPTED [
│     'attachment': {
│         "attachment data"
│     } [
│         'conformsTo': "https://joe-bob-s.com/cheap-attachments.html"
│         'vendor': "com.joe-bob-s"
│     ]
│     'signed': Signature
│ ]
```

## Appendix I: Generating Randomizers (***Development Only***)

`seedtool` can also be used to generate all of the randomizers that can be used as input for `seedtool`: `base6`, `base10`, `bits`, `cards`, `dice`, and `ints`. It will generate 16 digits of each randomizer at a time, so you will need to generate multiple outputs to create enough entropy for many input types.

**Seedtool Randomizer Outputs:**

| Type   | Seed Outputs            | Argument  |
| ------ | ----------------------- | --------- |
| Random | Card Draws              | -o cards  |
| Random | Coin Flips (0-1)        | -o bits   |
| Random | Dice Throws (1-6)       | -o dice   |
| Random | Base 6 Numbers (0-5)    | -o base6  |
| Random | Base 10 Numbers (0-9)   | -o base10 |
| Random | 1-Byte Integers (0-255) | -o ints   |

This is another feature that should **only** be used for development testing. If you're comfortable with `seedtool's` innate randomization (where you don't feed it any input), just use that. Contrariwise, if you're not, then draw your cards, roll your dice, or whatever by hand. But, if you want to use those inputs as part of a developmental test (or if you want to generate card draws, dice rolls, or numbers for use elsewhere), then the randomizer outputs should work fine.

To output a specific randomization, use the `-o` flag and the appropriate name. You cannot provide a randomizer with any input: it'll use `seedtool's` built-in randomization

```
seedtool -o base6

│ 5033020223254151

seedtool -o base10

│ 3642324148314505

seedtool -o bits

│ 1011111101010010

seedtool -o cards

9hjdjh2sas5hjcah8h5hjsad3h8htcts

seedtool -o dice

4324321245325254

```

the `-o ints` function is a little more complex. By default, it generates integers between 0 and 9, effectively the same as `base10` but with different formatting. You can expand that range with the `--low` and `-high` arguments, which can go as low as 0 and as high as 255, respectively.

```
seedtool -o ints

│ 8 2 1 8 8 3 5 5 8 2 3 7 0 1 1 3

seedtool -o ints --low 0 --high 255

│ 197 208 115 99 112 30 22 154 69 139 20 213 137 83 252 16

seedtool -o ints --low 100 --high 199

│ 142 164 168 133 187 110 151 177 145 108 140 159 131 153 121 157
```

Remember that changing the range changes the amount of entropy.

*Copyright © 2024 by Blockchain Commons, LLC*<br/>*Licensed under the "BSD-2-Clause Plus Patent License"*
