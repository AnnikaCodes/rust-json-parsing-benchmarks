# Rust JSON parsing benchmarks
This project aims to provide benchmarks to show how various JSON-parsing libraries in the Rust programming language perform at various JSON-parsing tasks.
## Benchmark methodology
The following Rust libraries for JSON parsing were benchmarked:
- [Serde JSON](https://crates.io/crates/serde_json)
- [GJSON](https://crates.io/crates/gjson)
- [A-JSON](https://crates.io/crates/ajson)
- [json-rust](https://crates.io/crates/json)
- [Pikkr](https://crates.io/crates/pikkr_annika)

For each library, each of the following tasks was benchmarked both on a large and a small JSON object (which can be found in the `json/` folder):
- Retrieving the value of a single top-level property
- Retrieving the value of a single fourth-level property
- Parsing the entire JSON object
## Benchmarking process
The benchmarks were taken under macOS 11.6 and rustc 1.57.0-nightly (8f8092cc3 2021-09-28) on a system with 16 GB of 2133 MHz RAM and a 2.4 GHz Intel i5-8279U CPU (with 4 physical and 8 logical cores). The benchmarks were all run at the same time, with minimal background processes.
## Benchmarks
### Serde JSON
### GJSON
### A-JSON
### json-rust
### Pikkr