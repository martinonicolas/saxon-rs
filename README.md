# SaxonRS

Rust bindings for SaxonC API.

This is a work in progress to build Rust bindings for the [SaxonC](https://www.saxonica.com/saxon-c/index.xml) library.



# Introduction

This repository includes two crates:  
- `saxonrs-sys`, a low-level crate that provides access to the SaxonC API methods via bindings, and thus employs unsafe code.  
- `saxonrs`, which leverages `saxonrs-sys` to expose a higher-level API, providing a safe and Rust-friendly way to use the API methods.



# How to build
1. Download SaxonC
2. Build saxonrs-sys
3. Build saxonrs

Before compiling **SaxonRS**, you must first download SaxonC and place it in the appropriate directory in order to build the `saxonrs-sys` crate.  
You can use the `download_saxon.sh` script, which automates all the necessary steps.  
This script automatically downloads the corresponding version of SaxonC and places it with the appropriate name in the correct directory.
```bash
cd <path_to_repository>
./download_saxonc.sh
```