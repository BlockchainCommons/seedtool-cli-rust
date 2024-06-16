# 🌱 Seedtool Usage Examples

## Basic Information

### Display usage and help information

```
$ seedtool --help
```

## Seed Generation

### Generate a cryptographically-strong 16-byte seed

```
$ seedtool
8935a8068526d84da555cdb741a3b8a8
```

### Generate a seed using entropy provided as coin flips

```
$ seedtool --in bits 1110110001110111
8d933e43b1bc8f2e3fc27adc98ad4534
```

### Generate a 32-byte seed using entropy provided as cards drawn from a deck of playing cards

```
$ seedtool --count 32 --in cards 6c9s8c7c9c4cah6c2sjs7d5c2s4c4dqs
7df301924511326d7350be14c9e7176d98e945f9ad0ed034726ad4ee0de59c25
```

## BIP-39 Mnemonics

### Encode a 16-byte seed as BIP-39

```
$ seedtool --in hex --out bip39 8935a8068526d84da555cdb741a3b8a8
matrix pull accuse apart horn chat next rifle resemble artist until eye
```

### Decode BIP-39 mnemonic to hex

```
$ seedtool --in bip39 "matrix pull accuse apart horn chat next rifle resemble artist until eye"
8935a8068526d84da555cdb741a3b8a8
```

## Bytewords

### Decode Bytewords to hex

```
$ seedtool --in btw "deli need cats taxi dice door webs vows free zero legs wall half waxy trip oval memo sets rock hill"
279b18d0282aefe845fb83e956eed8a6
```

## Metadata

### Generate a seed with a name, a note, and a creation date.

```
$ seedtool --out envelope --name "My Seed" --note "This is a note" --date "2024-07-01"
ur:envelope/lptpsogddezsdmjosscylolpettysbeetkjsrykpoyadcsspoybdtpsoiogtkkcxguihihieoybetpsosecyiylywnlaoyaatpsojtghisinjkcxinjkcxhscxjtjljyihmhyktswe
```

### Use the envelope Rust command line tool to show the envelope structure

```
ur:envelope/lptpsogddezsdmjosscylolpettysbeetkjsrykpoyadcsspoybdtpsoiogtkkcxguihihieoybetpsosecyiylywnlaoyaatpsojtghisinjkcxinjkcxhscxjtjljyihmhyktswe
Bytes(16) [
    'isA': 'Seed'
    'date': 2024-07-01
    'hasName': "My Seed"
    'note': "This is a note"
]
```

## SSKR

### Generate a 16-byte seed and encode it using SSKR as 3 shares, 2 of which are required for recovery

```
$ seedtool --out sskr --groups 2-of-3
ur:envelope/lftansfwlrhdcebegwdwmhspkeetamjyhslfsndslbahktuegyjlrfykmnrtltrdidwmsbgsbszojyesdmvwenehfnbblasrgdjoleahsofdlsiydybzctnenntoyteenehddatansfphdcxjtgtvdfszogdeoykntftcylsdecnrpknwnmwinsgfhhtptrfisbgcxrngldyjslooyamtpsotantkphddagluraeadaevwhpeshdrdjnzsfrlaeokpjzkevyemceurmegurefzpmwtrnlphproytjtfdrnfgfxcwykts
ur:envelope/lftansfwlrhdcebegwdwmhspkeetamjyhslfsndslbahktuegyjlrfykmnrtltrdidwmsbgsbszojyesdmvwenehfnbblasrgdjoleahsofdlsiydybzctnenntoyteenehddatansfphdcxjtgtvdfszogdeoykntftcylsdecnrpknwnmwinsgfhhtptrfisbgcxrngldyjslooyamtpsotantkphddagluraeadadtbdesersehoeoyiacnhtpmdrdwnyzmtsiycmdecxwminzejsvlhfgrkpiakteofxsrhdplck
ur:envelope/lftansfwlrhdcebegwdwmhspkeetamjyhslfsndslbahktuegyjlrfykmnrtltrdidwmsbgsbszojyesdmvwenehfnbblasrgdjoleahsofdlsiydybzctnenntoyteenehddatansfphdcxjtgtvdfszogdeoykntftcylsdecnrpknwnmwinsgfhhtptrfisbgcxrngldyjslooyamtpsotantkphddagluraeadaolsrytdlgrlvsgsluutvyuevtuochrfmerplronlrbtfmwpfrgafpfezsjyenrsgsnytkweoe
```

### Recover an SSKR-encoded seed using 2 of the 3 shares

```
$ seedtool --in sskr
ur:envelope/lftansfwlrhdcebegwdwmhspkeetamjyhslfsndslbahktuegyjlrfykmnrtltrdidwmsbgsbszojyesdmvwenehfnbblasrgdjoleahsofdlsiydybzctnenntoyteenehddatansfphdcxjtgtvdfszogdeoykntftcylsdecnrpknwnmwinsgfhhtptrfisbgcxrngldyjslooyamtpsotantkphddagluraeadaevwhpeshdrdjnzsfrlaeokpjzkevyemceurmegurefzpmwtrnlphproytjtfdrnfgfxcwykts
ur:envelope/lftansfwlrhdcebegwdwmhspkeetamjyhslfsndslbahktuegyjlrfykmnrtltrdidwmsbgsbszojyesdmvwenehfnbblasrgdjoleahsofdlsiydybzctnenntoyteenehddatansfphdcxjtgtvdfszogdeoykntftcylsdecnrpknwnmwinsgfhhtptrfisbgcxrngldyjslooyamtpsotantkphddagluraeadaolsrytdlgrlvsgsluutvyuevtuochrfmerplronlrbtfmwpfrgafpfezsjyenrsgsnytkweoe
^D
6171f066a3727480ea435aa5c5baeb90
```

## UR

### Generate a seed, encode it as UR, transform it to upper case, display it on the console, and encode it to a QR Code in the file "seedqrcode.png"

```
$ seedtool --out envelope | tr '[:lower:]' '[:upper:]' | tee /dev/tty | qrencode -o seedqrcode.png -l L
UR:ENVELOPE/LFTPSOGDIEVEADGHSARHHNFPDEKTZTJYSEDACFDPOYADCSSPGOHFURVS
```

![](images/seedqrcode.png)

### Generate a 64-byte seed using a deterministic random number generator and encode it as a multi-part UR with a maximum fragment size of 20 bytes

```
$ seedtool --deterministic=TEST --count 64 --out multipart --max-fragment-len=20
ur:envelope/1-4/lpadaacsgacygaioursfgulftpsohdfznteelblrcygldwvarflojtcywyjylyvlckjp
ur:envelope/2-4/lpaoaacsgacygaioursfgutpdkfwprylienshnjnpluypmamtkmybsjkspvsbtlklobt
ur:envelope/3-4/lpaxaacsgacygaioursfgueesawmrltdlnlgkplfbkqzzoglfeoyaegsnedtsrkppecy
ur:envelope/4-4/lpaaaacsgacygaioursfgurowsdpgtimmwzspfqdjkhshyoyadcsspaeaeaegeiesbje
```

### Same as above, but generate 5 additional parts using fountain codes

```
$ seedtool --deterministic=TEST --count 64 --out multipart --max-fragment-len=20 --additional-parts 5
ur:envelope/1-4/lpadaacsgacygaioursfgulftpsohdfznteelblrcygldwvarflojtcywyjylyvlckjp
ur:envelope/2-4/lpaoaacsgacygaioursfgutpdkfwprylienshnjnpluypmamtkmybsjkspvsbtlklobt
ur:envelope/3-4/lpaxaacsgacygaioursfgueesawmrltdlnlgkplfbkqzzoglfeoyaegsnedtsrkppecy
ur:envelope/4-4/lpaaaacsgacygaioursfgurowsdpgtimmwzspfqdjkhshyoyadcsspaeaeaegeiesbje
ur:envelope/5-4/lpahaacsgacygaioursfgurpcycpwsmocwrhbkambezstspdytdtjthfjshlfrfxktro
ur:envelope/6-4/lpamaacsgacygaioursfgurowsdpgtimmwzspfqdjkhshyoyadcsspaeaeaelkjekkcn
ur:envelope/7-4/lpataacsgacygaioursfgutbttgtbebswmurtntpsnfzdkbsemrnptdarhrednskhfbs
ur:envelope/8-4/lpayaacsgacygaioursfgughaslrfdgwkowmonhhtsbaaywlluenstfhhgseckdemodn
ur:envelope/9-4/lpasaacsgacygaioursfgutpdkfwprylienshnjnpluypmamtkmybsjkspvsamhddssr
```

### Recover the seed using a subset of the generated parts

```
$ seedtool --in multipart
ur:envelope/1-4/lpadaacsgacygaioursfgulftpsohdfznteelblrcygldwvarflojtcywyjylyvlckjp
ur:envelope/3-4/lpaxaacsgacygaioursfgueesawmrltdlnlgkplfbkqzzoglfeoyaegsnedtsrkppecy
ur:envelope/5-4/lpahaacsgacygaioursfgurpcycpwsmocwrhbkambezstspdytdtjthfjshlfrfxktro
ur:envelope/7-4/lpataacsgacygaioursfgutbttgtbebswmurtntpsnfzdkbsemrnptdarhrednskhfbs
ur:envelope/9-4/lpasaacsgacygaioursfgutpdkfwprylienshnjnpluypmamtkmybsjkspvsamhddssr
^D
9d347f841a4e2ce6bc886e1aee74d82442b2f7649c606daedbad06cf8f0f73c8e834c2ebb7d2868d75820ab4fb4e45a1004c9f29b8ef2d4d6a94fab0b373615e
```
