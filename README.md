# Papillon

## description
This is a toolset for Hacking

## commands
### calc
Calculate expression and convert a answer to Hexadecimal, Decimal and Binary

`calc` require a specific prefix to specify the number. (Excluding Decimal)

Hex: 0x, Bin: 0b
```
papillon calc '0xFF - 0b1101 + 256'
```

### patt
Generate De Bruijn Sequence of a given length and subsequence length

`len` is length of the sequence to generate

`n` is length of the subsequences
```
papillon patt <len> <n>
papilln patt 50 4
```

### find
Calculate the offset of subsequence in de_bruijn sequence

`de_bruijn_sequence` is original sequence to calculate the offset of the location where `subsequence` exists

`subsequence` is subsequence in `de_bruijn_sequence`
```
papillon find AAAABAAACAAADAAAEAAAFAAAGAAAHAAAIAAAJAAAKAAALAAAMAAAN BAAA
```

## contribute
Contributors are encouraged to submit Git pull requests to fix bugs and add new features
