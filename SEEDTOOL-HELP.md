This is the help of the original C++ seedtool:

```
Usage: seedtool [OPTION...] INPUT
Converts cryptographic seeds between various forms.

  -c, --count=1-1024         The number of output units (default: 16)
  -i, --in=random|hex|btw|btwu|btwm|bits|cards|dice|base6|base10|ints|bip39|sskr|ur                                                          The input format
                             (default: random)
  -o, --out=hex|btw|btwu|btwm|bits|cards|dice|base6|base10|ints|bip39|sskr
                             The output format (default: hex)
  -p, --parts=FOUNTAIN_PARTS For multi-part URs, the number of additional UR parts above the minimum to generate using fountain encoding.
  -u, --ur[=MAX_FRAGMENT_LENGTH]   Encode output as a Uniform Resource (UR). If necessary the UR will be segmented into parts with fragments no larger than MAX_FRAGMENT_LENGTH.

 ints Input and Output Options:
  -h, --high=1-255           The highest int returned (default: 9)
  -l, --low=0-254            The lowest int returned (default: 1)
      --low < high

 SSKR Output Options:
  -g, --group=M-of-N         The group specification (default: 1-of-1)
      --M < N
  -t, --group-threshold=1-16 The number of groups that must meet their threshold (default: 1)
      --The --group option may appear more than once.
      --The group threshold must be <= the number of group specifications.

 Deterministic Random Numbers:
  -d, --deterministic=SEED   Use a deterministic random number generator with the given seed.

  -?, --help                 Give this help list
      --usage                Give a short usage message
  -V, --version              Print program version

Mandatory or optional arguments to long options are also mandatory or optional
for any corresponding short options.

Report bugs to ChristopherA@BlockchainCommons.com. © 2020 Blockchain Commons.
```
