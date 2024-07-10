# YARP - Yet Another Rearranger of Peptides

- YARP generates decoy sequences for a protein FASTA file. 
- YARP is written in Rust and is fast, lightweight, and easy to use.
- YARP can generate sequences using either pseudo-reverse or pseudo-shuffled sequences. That is, a protein sequence
  is in silico digested into peptides, and those peptides are reversed or shuffled while keeping the terminal residue
  fixed (e.g., n-terminal K or R for trypsin). Then those peptides are concatenated back together for the decoy protein
  sequence.

## Ways You Can Run YARP
There are several ways to run YARP:

### 1. Download and Run
Download the [latest release](https://github.com/mriffle/yarp/releases) for your platform. Currently Windows, 
Mac OS, and Linux are supported.

### 2. Run Using Docker
YARP exists as a Docker image on Docker Hub. You can run with a command similar to the following to run
the latest version:

``docker run --rm -it --user $(id -u):$(id -g) -v `pwd`:`pwd`  -w `pwd` mriffle/yarp:latest yarp``

## How to Run YARP
YARP is a command line program. Run with no arguments to see the following help text:

```
YARP (Yet Another Rearranger of Peptides) v1.0.2
Usage: yarp --fasta=FASTA [OPTIONS]
Options:
  --fasta=FASTA        Input FASTA file (required)
  --method=METHOD      Decoy generation method: 'shuffle' or 'reverse' (default: reverse)
  --decoy-string=STRING Prefix for decoy sequence headers (default: DECOY_)
  --seed=SEED          Random seed for shuffling (default: 1337)
  --protease=PROTEASE  Protease for in silico digestion: 'trypsin' (default: trypsin)
  --num-shuffles=N     Number of shuffles to perform when using 'shuffle' method (default: 1)
  --help               Print this help message
  --version            Print version information
```

Note: If using ``--num-shuffles=X`` with ``X`` greater than 1, and if using ``--method=shuffle``, YARP will
perform ``X`` shuffles of the peptide and choose the one least similar to the original peptide sequence. The
least similar peptide is the one with the fewest number of residues in the same position and the fewest number
of instances of identical adjacent residues. For example PEPTIDE and EPTIDEP have no residues in the same
position, but have a large number of identical adjacent residues, since the sequence is the same, just shifted
by one.

### Examples for running YARP
Here are examples of how to run YARP for the platforms available on the
[latest releases](https://github.com/mriffle/yarp/releases) page.

#### Windows Example:
``yarp-windows-amd64.exe --fasta=c:\data\yeast.fasta >yeast_plus_decoys.fasta``

#### Mac OS Example:
``yarp-windows-amd64.exe --fasta=c:\data\yeast.fasta --decoy-string=YARP_ >yeast_plus_decoys.fasta``

Note: this method overrides the default decoy string, changing it to "YARP_". All decoy protein
entries in the FASTA will begin with "YARP_". The default is "DECOY_".

#### Linux Example:
``yarp-macos-amd64 --fasta=c:\data\yeast.fasta --method=shuffle >yeast_plus_decoys.fasta``

Note: this method overrides the default shuffle method.

#### Docker Example:
``docker run --rm -it --user $(id -u):$(id -g) -v `pwd`:`pwd`  -w `pwd` mriffle/yarp:latest yarp --method=shuffle --num-shuffles=20 --decoy-string=YARP_ >yeast_plus_decoys.fasta``

Note: this method performs 20 shuffles and chooses the one that is least similar to the starting peptide and uses
"YARP_" as the decoy string.
