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
The benchmarks were run sequentially with output redirected to a text file (`cargo bench > bench.txt`, with minimal background processes.
## Results
There are two main types of JSON parsing library:
- _property-parsing libraries_ (Serde JSON, json-rust, simd-json, and tinyjson), which focus on retrieving the value of a particular JSON property, and have little to no ability to parse an entire JSON file at once
- _object-parsing libraries_ (GJSON, A-JSON, and Pikkr), which parse an entire JSON object/file at once, and subsequently allow access to any value within the JSON file with minimal overhead
Each of these library types has different applications; you need to figure out which one is best for your use case.

Although I did my best to implement each benchmark for every library, the most relevant benchmarks for the property-parsing libraries are the `top_level` and `fourth_level` benchmarks (which retrieve a single value).
Conversely, the most relevant benchmark for the object-parsing libraries is the `parse_all` benchmark; the other benchmarks will take about the same time, since these libraries must parse the entire object before retrieving any values.
### Object-parsing libraries
The fastest object-parsing JSON library benchmarked here was **json-rust**, which was about 2.7x faster than the second-fastest, serde_json, at parsing large objects and about 2x faster than serde_json at parsing small objects.
If you want the features of Serde, **Serde JSON** is 1.6x faster than simd-json for small objects; while parsing large objects, they are about equally matched, except that simd-json is 1.5x faster when parsing a single top-level property. (I'm not really sure why this is, since simd-json ostensibly parses the entire object at once. Maybe it's cleverly optimized at compile-time for top-level property accesses?)
Although its syntax is rather clean, **tinyjson** has the worst performance of all the object-parsing libraries benchmarked here.

![Performance graph of object-parsing libraries with large JSON objects](https://docs.google.com/spreadsheets/d/e/2PACX-1vQmREyK7BH0uoEHBLYVAhAqSQnPLf5sjwp__aa1MwuB0aZsRwPTGjAJTlkZAs7MQ6tjlwlI1AsYwBYG/pubchart?oid=1082717416&format=image)
![Performance graph of object-parsing libraries with small JSON objects](https://docs.google.com/spreadsheets/d/e/2PACX-1vQmREyK7BH0uoEHBLYVAhAqSQnPLf5sjwp__aa1MwuB0aZsRwPTGjAJTlkZAs7MQ6tjlwlI1AsYwBYG/pubchart?oid=1166695421&format=image)
### Property-parsing libraries
In many JSON-parsing applications, one only needs to retrieve a few properties from a JSON object. In this case, property-parsing libraries are significantly faster than object-parsing ones.

The fastest property-parsing library in every benchmark I ran is **GJSON**, which was two to five times faster than the next fastest property-parsing library (A-JSON). **Pikkr**, in spite of its speculative-parsing algorithm, was the slowest property-parsing library benchmarked (aside from in fourth-level property accesses on small JSON objects, where it outperforms A-JSON). In fact, the stateless Pikkr benchmarks (which introduce overhead from instantiating a new parser AND
can't take advantage of speculative parsing) were so slow on the small JSON object that I made a graph without them, to make it easier to compare A-JSON and GJSON.

![Performance graph of property-parsing libraries with large JSON objects](https://docs.google.com/spreadsheets/d/e/2PACX-1vQmREyK7BH0uoEHBLYVAhAqSQnPLf5sjwp__aa1MwuB0aZsRwPTGjAJTlkZAs7MQ6tjlwlI1AsYwBYG/pubchart?oid=572340211&format=image)
![Performance graph of property-parsing libraries with small JSON objects](https://docs.google.com/spreadsheets/d/e/2PACX-1vQmREyK7BH0uoEHBLYVAhAqSQnPLf5sjwp__aa1MwuB0aZsRwPTGjAJTlkZAs7MQ6tjlwlI1AsYwBYG/pubchart?oid=44509521&format=image)
![Performance graph of property-parsing libraries with small JSON objects (no stateless Pikkr)](https://docs.google.com/spreadsheets/d/e/2PACX-1vQmREyK7BH0uoEHBLYVAhAqSQnPLf5sjwp__aa1MwuB0aZsRwPTGjAJTlkZAs7MQ6tjlwlI1AsYwBYG/pubchart?oid=325671513&format=image)
## Benchmarks
All benchmarks are available in the `bench.txt` file as well as in a [spreadsheet](https://docs.google.com/spreadsheets/d/1NqRzyH68OGFwUw4es-CqmcxVzl-ohact7f9jiacXW8g/edit?usp=sharing).
### Serde JSON
Serde JSON is the most popular JSON parser, with 46 million all-time downloads. It allows integration of JSON values with Rust's typing system; this behavior is not utilized in these benchmarks.
Serde JSON is an object-parsing library, so the `serde_large_top_level` and `serde_small_top_level` benchmarks provide the most accurate measure of its performance.

Its performance is okay, but not the best:
```
test tests::serde_large_fourth_level           ... bench:     312,143 ns/iter (+/- 30,043)
test tests::serde_large_parse_all              ... bench:     311,408 ns/iter (+/- 13,977)
test tests::serde_large_top_level              ... bench:     309,296 ns/iter (+/- 7,600)
test tests::serde_small_fourth_level           ... bench:       1,283 ns/iter (+/- 41)
test tests::serde_small_parse_all              ... bench:       1,249 ns/iter (+/- 169)
test tests::serde_small_top_level              ... bench:       1,297 ns/iter (+/- 140)
```
### json-rust
json-rust is another object-parsing library.
The `json_rust_large_top_level` and `json_rust_small_top_level` benchmarks provide the most accurate measure of its performance.

It has the best performance out of all the object-parsing libraries here!
```
test tests::json_rust_large_fourth_level       ... bench:     112,779 ns/iter (+/- 5,560)
test tests::json_rust_large_parse_all          ... bench:     113,249 ns/iter (+/- 9,693)
test tests::json_rust_large_top_level          ... bench:     113,586 ns/iter (+/- 8,524)
test tests::json_rust_small_fourth_level       ... bench:       1,025 ns/iter (+/- 131)
test tests::json_rust_small_parse_all          ... bench:         982 ns/iter (+/- 139)
test tests::json_rust_small_top_level          ... bench:       1,009 ns/iter (+/- 185)
```
### simd-json
simd-json is an implementation of the Serde JSON API using x86_64 [SIMD instructions](https://en.wikipedia.org/wiki/SIMD); this means that it will only work on compatible Intel and AMD processors (ARM and other architectures will need to use a different library).
It is an object-parsing library, meaning that the `simd_large_top_level` and `simd_small_top_level` benchmarks provide the most accurate measure of its performance.

It performs slightly better than Serde JSON on large JSON files (and significantly better at top-level property accesses, although if that's all you care about, [GJSON](#GJSON) will perform better), and significantly worse on small ones:
```
test tests::simd_large_fourth_level            ... bench:     306,219 ns/iter (+/- 13,723)
test tests::simd_large_parse_all               ... bench:     308,222 ns/iter (+/- 20,767)
test tests::simd_large_top_level               ... bench:     205,169 ns/iter (+/- 14,734)
test tests::simd_small_fourth_level            ... bench:       2,066 ns/iter (+/- 110)
test tests::simd_small_parse_all               ... bench:       2,011 ns/iter (+/- 624)
test tests::simd_small_top_level               ... bench:       2,423 ns/iter (+/- 1,013)
```
### tinyjson
tinyjson is a JSON parsing that uses the `String::parse` API, so you can parse a string to JSON with a syntax as simple as `my_string.parse::<tinyjson::JsonValue>()?`.
It is an object-parsing library, so the `tinyjson_large_top_level` and `tinyjson_small_top_level` benchmarks provide the most accurate measure of its performance, which is quite poor:
```
test tests::tinyjson_large_fourth_level        ... bench:     372,128 ns/iter (+/- 10,674)
test tests::tinyjson_large_parse_all           ... bench:     377,557 ns/iter (+/- 33,221)
test tests::tinyjson_large_top_level           ... bench:     372,601 ns/iter (+/- 56,079)
test tests::tinyjson_small_fourth_level        ... bench:       2,116 ns/iter (+/- 191)
test tests::tinyjson_small_parse_all           ... bench:       1,993 ns/iter (+/- 81)
test tests::tinyjson_small_top_level           ... bench:       2,056 ns/iter (+/- 147)
```
### GJSON
GJSON is a port of the [Go JSON parser with the same name](https://github.com/tidwall/gjson) by the author of that package.
It is a property-parsing library, so the `gjson_large_fourth_level`, `gjson_large_top_level`, `gjson_small_fourth_level`, and `gjson_small_top_level` benchmarks are the ones to look at.

It is, by a significant margin, the fastest property-parsing library here!
```
test tests::gjson_large_fourth_level           ... bench:      12,075 ns/iter (+/- 1,436)
test tests::gjson_large_parse_all              ... ignored
test tests::gjson_large_top_level              ... bench:       3,120 ns/iter (+/- 139)
test tests::gjson_small_fourth_level           ... bench:         239 ns/iter (+/- 30)
test tests::gjson_small_parse_all              ... ignored
test tests::gjson_small_top_level              ... bench:          92 ns/iter (+/- 7)
```
(The `parse_all` benchmarks are ignored because GJSON does not have a way to perform all the parsing for an object at once.)
### A-JSON
A-JSON is a port, by a different author, of Go's GJSON package.
It is a property-parsing library, so the `ajson_large_fourth_level`, `ajson_large_top_level`, `ajson_small_fourth_level`, and `ajson_small_top_level` benchmarks are the ones to look at.

A-JSON performs worse than GJSON, but better than Pikkr at some tasks and worse at others:
```
test tests::ajson_large_fourth_level           ... bench:      37,171 ns/iter (+/- 2,681)
test tests::ajson_large_parse_all              ... ignored
test tests::ajson_large_top_level              ... bench:      16,511 ns/iter (+/- 753)
test tests::ajson_small_fourth_level           ... bench:         782 ns/iter (+/- 463)
test tests::ajson_small_parse_all              ... ignored
test tests::ajson_small_top_level              ... bench:         183 ns/iter (+/- 16)
```
(Like GJSON, A-JSON's `parse_all` benchmarks are ignored because it can't frontload the parsing for an entire object.)
### Pikkr
Pikkr is a property-parsing library with a unique 'speculative-parsing' approach. Its parser keeps track of state in an effort to make parsing more efficient.
Since this approach does not work in all situations (for example, concurrent programs must create a new Pikkr instance for each thread), I've benchmarked both `pikkr_stateful` (which uses one Pikkr instance for all benchmark iterations) and `pikkr_stateless` (which creates a new Pikkr instance on each benchmark iteration).

Also, Pikkr's [original repository](https://github.com/melanieseltzer/pikkr/) has not been committed to in 2.5 years, and fails to build on my system. This benchmark uses [my fork](https://crates.io/crates/pikkr-annika), which fixes the build errors but is not maintained (except for reported security vulnerabilities).

It's possible that Pikkr would perform better in an environment ideal for its speculative parsing, but in this benchmark, it performs quite poorly:
```
test tests::pikkr_stateful_large_fourth_level  ... bench:      20,237 ns/iter (+/- 627)
test tests::pikkr_stateful_large_parse_all     ... ignored
test tests::pikkr_stateful_large_top_level     ... bench:      19,985 ns/iter (+/- 2,192)
test tests::pikkr_stateful_small_fourth_level  ... bench:         693 ns/iter (+/- 48)
test tests::pikkr_stateful_small_parse_all     ... ignored
test tests::pikkr_stateful_small_top_level     ... bench:         508 ns/iter (+/- 18)
test tests::pikkr_stateless_large_fourth_level ... bench:      26,729 ns/iter (+/- 3,656)
test tests::pikkr_stateless_large_parse_all    ... ignored
test tests::pikkr_stateless_large_top_level    ... bench:      34,406 ns/iter (+/- 1,159)
test tests::pikkr_stateless_small_fourth_level ... bench:       4,327 ns/iter (+/- 290)
test tests::pikkr_stateless_small_parse_all    ... ignored
test tests::pikkr_stateless_small_top_level    ... bench:       2,419 ns/iter (+/- 287)
```
