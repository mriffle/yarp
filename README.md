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

``docker run --rm --user $(id -u):$(id -g) -v `pwd`:`pwd`  -w `pwd` mriffle/yarp:latest yarp``

## How to Run YARP
YARP is a command line program. Run with no arguments to see the following help text:

```
Yet Another Rearranger of Peptides

Usage: yarp [OPTIONS] --fasta-file <FILE>

Options:
      --fasta-file <FILE>            Input FASTA file
      --decoy-method <DECOY_METHOD>  Decoy generation method [default: reverse] [possible values: shuffle, reverse]
      --decoy-prefix <DECOY_PREFIX>  Prefix for decoy sequence headers [default: DECOY_]
      --seed <SEED>                  Random seed for shuffling [default: 1337]
      --protease <PROTEASE>          Protease for in silico digestion [default: trypsin] [possible values: trypsin]
      --num-shuffles <NUM_SHUFFLES>  Number of shuffles to perform when using 'shuffle' method [default: 1]
  -h, --help                         Print help
  -V, --version                      Print version

```

Note: If using ``--num-shuffles=X`` with ``X`` greater than 1, and if using ``--decoy-method=shuffle``, YARP will
perform ``X`` shuffles of the peptide and choose the one least similar to the original peptide sequence. Similarity
is based on the number of fragment ion masses in common with the unshuffled original sequence.

### Examples for running YARP
Here are examples of how to run YARP for the platforms available on the
[latest releases](https://github.com/mriffle/yarp/releases) page.

#### Windows Example:
``yarp-windows-amd64.exe --fasta-file=c:\data\yeast.fasta >yeast_plus_decoys.fasta``

#### Mac OS Example:
``yarp-macos-amd64 --fasta-file=c:\data\yeast.fasta --decoy-prefix=YARP_ >yeast_plus_decoys.fasta``

Note: this method overrides the default decoy string, changing it to "YARP_". All decoy protein
entries in the FASTA will begin with "YARP_". The default is "DECOY_".

#### Linux Example:
``yarp-linux-amd64 --fasta-file=c:\data\yeast.fasta --decoy-method=shuffle >yeast_plus_decoys.fasta``

Note: this method overrides the default shuffle method.

#### Docker Example:
``docker run --rm --user $(id -u):$(id -g) -v `pwd`:`pwd`  -w `pwd` mriffle/yarp:latest yarp --fasta-file=c:\data\yeast.fasta --decoy-method=shuffle --num-shuffles=20 --decoy-prefix=YARP_ >yeast_plus_decoys.fasta``

Note: this method performs 20 shuffles and chooses the one that is least similar to the starting peptide and uses
"YARP_" as the decoy string.
