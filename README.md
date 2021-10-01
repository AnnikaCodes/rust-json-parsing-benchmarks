# Rust JSON parsing benchmarks
This project aims to provide benchmarks to show how various JSON-parsing libraries in the Rust programming language perform at various JSON-parsing tasks. It is only concerned with performance, and does not take into account other factors that you should consider when choosing a JSON parser (such as validation, RAM usage, code readability, and active maintainace).
## Benchmark methodology
The following Rust libraries for JSON parsing were benchmarked:
- [Serde JSON](https://crates.io/crates/serde_json)
- [GJSON](https://crates.io/crates/gjson)
- [A-JSON](https://crates.io/crates/ajson)
- [json-rust](https://crates.io/crates/json)
- [Pikkr](https://crates.io/crates/pikkr_annika)
- [simd-json](https://crates.io/crates/simd-json)
- [tinyjson](https://crates.io/crates/tinyjson)

For each library, each of the following tasks was benchmarked both on a large and a small JSON object (which can be found in the `json/` folder):
- Retrieving the value of a single top-level property
- Retrieving the value of a single fourth-level property
- Parsing the entire JSON object
## Benchmarking process
The benchmarks were taken under macOS 11.6 and rustc 1.57.0-nightly (8f8092cc3 2021-09-28) on a system with 16 GB of 2133 MHz RAM and a 2.4 GHz Intel i5-8279U CPU (with 4 physical and 8 logical cores).
The benchmarks were run sequentially with output redirected to a text file (`cargo bench > bench.txt`), with minimal background processes.
## Results
There are two main types of JSON parsing library:
- _property-parsing libraries_ (Serde JSON, json-rust, simd-json, and tinyjson), which focus on retrieving the value of a particular JSON property, and have little to no ability to parse an entire JSON file at once
- _object-parsing libraries_ (GJSON, A-JSON, and Pikkr), which parse an entire JSON object/file at once, and subsequently allow access to any value within the JSON file with minimal overhead
Each of these library types has different applications; you need to figure out which one is best for your use case.

Although I did my best to implement each benchmark for every library, the most relevant benchmarks for the property-parsing libraries are the `top_level` and `fourth_level` benchmarks (which retrieve a single value).
Conversely, the most relevant benchmark for the object-parsing libraries is the `parse_all` benchmark; the other benchmarks will take about the same time, since these libraries must parse the entire object before retrieving any values.

TODO: run benchmarks, put pretty bar graphs here
## Benchmarks
### Serde JSON
Serde JSON is the most popular JSON parser, with 46 million all-time downloads. It allows integration of JSON values with Rust's typing system; this behavior is not utilized in these benchmarks.
Serde JSON is an object-parsing library, so the `serde_large_top_level` and `serde_small_top_level` benchmarks provide the most accurate measure of its performance.
### json-rust
json-rust is another object-parsing library with a similar syntax to Serde.
The `json_rust_large_top_level` and `json_rust_small_top_level` benchmarks provide the most accurate measure of its performance.
### simd-json
simd-json is an implementation of the Serde JSON API using x86_64 [SIMD instructions](https://en.wikipedia.org/wiki/SIMD); this means that it will only work on compatible Intel and AMD processors (ARM and other architectures will need to use a different library).
It is an object-parsing library, meaning that the `simd_large_top_level` and `simd_small_top_level` benchmarks provide the most accurate measure of its performance.
### tinyjson
tinyjson is a JSON parsing that uses the `String::parse` API, so you can parse a string to JSON with a syntax as simple as `my_string.parse::<tinyjson::JsonValue>()?`.
It is an object-parsing library, so the `tinyjson_large_top_level` and `tinyjson_small_top_level` benchmarks provide the most accurate measure of its performance.
### GJSON
GJSON is a port of the [Go JSON parser with the same name](https://github.com/tidwall/gjson) by the author of that package.
It is a property-parsing library, so the `gjson_large_fourth_level`, `gjson_large_top_level`, `gjson_small_fourth_level`, and `gjson_small_top_level` benchmarks are the ones to look at.
### A-JSON
A-JSON is a port, by a different author, of Go's GJSON package.
It is a property-parsing library, so the `ajson_large_fourth_level`, `ajson_large_top_level`, `ajson_small_fourth_level`, and `ajson_small_top_level` benchmarks are the ones to look at.
### Pikkr
Pikkr is a property-parsing library with a unique 'speculative-parsing' approach. Its parser keeps track of state in an effort to make parsing more efficient.
Since this approach does not work in all situations (for example, concurrent programs must create a new Pikkr instance for each thread), I've benchmarked both `pikkr_stateful` (which uses one Pikkr instance for all benchmark iterations) and `pikkr_stateless` (which creates a new Pikkr instance on each benchmark iteration).

Also, Pikkr's [original repository](https://github.com/melanieseltzer/pikkr/) has not been committed to in 2.5 years, and fails to build on my system. This benchmark uses [my fork](https://crates.io/crates/pikkr-annika), which fixes the build errors but is not maintained (except for reported security vulnerabilities).
