# linkage-by-mobility-behavior
This repository contains the code for the Master thesis "Tracking Individual Behavioral Patterns". This code allows to perform linkage attacks using a histogram-based approach as well as an sequence alignment-based approach. The code is fully parallelized and built in Rust for raw speed.

## Setup 
Requirement: 
* Rust: 1.55.0 

Installation:
```
$ git clone https://github.com/Madabaru/linkage-by-mobility-behavior.git
$ cd ..
$ cargo build --release && ./target/release/linkage-by-mobility-behavior
```
For help regarding the available parameters, simply run:
```
$ ./target/release/linkage-by-mobility-behavior --help
```
