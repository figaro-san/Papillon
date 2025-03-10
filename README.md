# Papillon

## description
This is a toolset for Hacking

## commands
### calc
Calculate expression and convert a answer to Hexadecimal, Decimal and Binary

Only four arithmetic operations are supported (+, -, *, /)

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
papillon patt 50 4
```

### find
Calculate the offset of subsequence in de_bruijn sequence

`de_bruijn_sequence` is original sequence to calculate the offset of the location where `subsequence` exists

`subsequence` is subsequence in `de_bruijn_sequence`
```
papillon find <de_bruijn_sequence> <subsequence>
papillon find AAAABAAACAAADAAAEAAAFAAAGAAAHAAAIAAAJAAAKAAALAAAMAAAN BAAA
```

### readelf
This program is a degraded version of readelf.

Currently, it only supports reading Little Endian ELF Header on x86-64 architecture.

`filepath` is filepath to target binary.

```
papillon readelf </path/to/binary>
```

## contribute
Contributors are encouraged to submit Git pull requests to fix bugs and add new features
