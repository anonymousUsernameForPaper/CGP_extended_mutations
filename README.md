# Revisiting (Extended Versions of) Cartesian Genetic Programming's Mutation Operators
Code and Benchmarks for the paper: Revisiting (Extended Versions of) Cartesian Genetic Programming's Mutation Operators

# Rust
The code is written in Rust only.  
For installation, see: https://github.com/rust-lang/rust/blob/master/README.md

# Building
You have to build everything yourself. You will need a working `Rust` and `Cargo` setup. [Rustup](https://rustup.rs/) is the simplest way to set this up on either Windows, Mac or Linux.

Once the prerequisites have been installed, compilation on your native platform is as simple as running the following in a terminal:

```
cargo build --release
```


# Usage
Run the build executable on your machine via:
```
./target/release/cgp
```
or 
```
./target/release/cgp.exe
```

Outputs will be placed into a folder called
`Experiments_Output`

You can configure the run via following command line arguments:
- `run-id`
  - The ID of the run
  - Only important for saving results
  - default: 0
- `dataset_type`:
  - Which Dataset Type to use: Boolean or symbolic regression ones 
  - "f32"
  - "bool   
- `dataset`
  - which dataset to use. For Boolean:  
        0: Parity  
        1: Encode  
        2: Decode  
        3: Multiply  
  - for symbolic regression:  
        0: nguyen_7  
        1: koza_3  
        2: pagie_1  
        3: keijzer_6  
  - default: 0
    
- `mutation_type`
  - Can be either:
  - "Single"
  - "Point"
  - "Multi"
  - "Split"
- `mutation_rate`
  - For "Point" mutation. Normal Mutation rate

- `mutation_multi_n`
  - For "Multi" mutation.

- `split_mutation_rate_active`
  - For "Split" mutation. Mutation rate for active nodes

- `split_mutation_rate_inactive`
  - For "Spilt" mutation. Mutation rate for inactive nodes
