SEK - The Sequence Exploration Kit
==================================

An experimental toolkit for visualizing biological sequence features.

The initial purpose of the toolkit is two-fold:

1. For me to learn Rust, and in particular how to parse some common
   bioinformatics formats with it.
2. To scratch some of my own itches in terms of making it easier to navigate
   some common file formats manually.

## Commands

- `sek col`: Pipe DNA text to this command to have it colorized.
- `sek sam`: Pipe samtools output (without headers), to have the CIGAR and SEQ fields colorized.

## Examples

*Colorize a fasta file:*

```bash
seqkit seq -s somefile.fa | sek col | less -SiR
```

*Colorize a sam file:*

```bash
samtools view --no-header somefile.sam | sek sam | column -t | less -SiR
```
