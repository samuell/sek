SEK - The Sequence Exploration Kit
==================================

An experimental toolkit for visualizing biological sequence features.

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
samtools view somefile.sam | sek sam | column -t | less -SiR
```
