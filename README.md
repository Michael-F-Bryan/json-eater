# JSON Eater

[![Continuous integration](https://github.com/Michael-F-Bryan/json-eater/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/Michael-F-Bryan/json-eater/actions)

([API Docs])

Turn nested JSON objects into a form that can easily be searched by other tool.

## Examples

The [`eat_json()`] function is where you want to start. You can use the
[`CsvWriter`] if you don't want to implement your own [`Visitor`].

```rust
use json_eater::CsvWriter;

let json = r#"{
    "name": "John",
    "isAlive": true,
    "age": -27,
    "address": {
        "streetAddress": "21 2nd Street"
    },
    "numbers": [12.32]
  }"#;

let mut writer = CsvWriter::new(Vec::new());
json_eater::eat_json(json.as_bytes(), &mut writer).unwrap();

let csv = String::from_utf8(writer.into_inner()).unwrap();

assert_eq!(
  csv.trim(),
  "
name, John
isAlive, true
age, -27
address/streetAddress, 21 2nd Street
numbers/0, 12.32
  ".trim(),
);
```

## Benchmarks

This project comes with a suite of benchmarks. The [`criterion` tool][criterion]
can be used to run these benchmarks and generate pretty graphs.

First, make sure the `cargo criterion` subcommand is installed.

```console
$ cargo install cargo-criterion
```

Then run it.

```console
$ cargo criterion
# Compare the effect of streaming from disk versus an in-memory buffer
input format/in-memory buffer
                        time:   [808.13 µs 810.97 µs 814.10 µs]
                        thrpt:  [641.24 MiB/s 643.71 MiB/s 645.98 MiB/s]
input format/file reader
                        time:   [3.1903 ms 3.2000 ms 3.2126 ms]
                        thrpt:  [162.50 MiB/s 163.14 MiB/s 163.63 MiB/s]

# See how much overhead is added by the thing consuming
writer overhead/baseline
                        time:   [806.04 µs 807.00 µs 808.15 µs]
                        thrpt:  [645.96 MiB/s 646.89 MiB/s 647.65 MiB/s]
Benchmarking writer overhead/csv in memory: Warming up for 3.0000 s
writer overhead/csv in memory
                        time:   [1.7513 ms 1.7526 ms 1.7543 ms]
                        thrpt:  [297.58 MiB/s 297.87 MiB/s 298.08 MiB/s]
Benchmarking writer overhead/csv on disk: Warming up for 3.0000 s
writer overhead/csv on disk
                        time:   [1.8821 ms 1.8836 ms 1.8853 ms]
                        thrpt:  [276.90 MiB/s 277.15 MiB/s 277.37 MiB/s]

# How does the input document's size affect throughput?
input size/547.39 KB    time:   [805.07 µs 807.45 µs 810.97 µs]
                        thrpt:  [643.72 MiB/s 646.52 MiB/s 648.44 MiB/s]
input size/124.94 KB    time:   [191.32 µs 191.55 µs 191.80 µs]
                        thrpt:  [621.24 MiB/s 622.04 MiB/s 622.77 MiB/s]
input size/2.47 KB      time:   [3.3742 µs 3.3817 µs 3.3902 µs]
                        thrpt:  [695.67 MiB/s 697.41 MiB/s 698.96 MiB/s]
input size/111 B        time:   [290.75 ns 290.90 ns 291.04 ns]
                        thrpt:  [363.73 MiB/s 363.90 MiB/s 364.08 MiB/s]

# Process a single file, reading it from disk and saving it to a *.csv file
Process 547.39 KB         time:   [4.1542 ms 4.1554 ms 4.1567 ms]
```

The report will be available under the `target/` folder.

```console
$ ls target/criterion/reports
index.html  Index 547.39 KB  input format  input size  writer overhead
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.md) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT.md) or
   <http://opensource.org/licenses/MIT>)

at your option.

It is recommended to always use [cargo-crev][crev] to verify the
trustworthiness of each of your dependencies, including this one.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

The intent of this crate is to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help in analysing and fixing them.

[API Docs]: https://michael-f-bryan.github.io/json-eater
[crev]: https://github.com/crev-dev/cargo-crev
[criterion]: https://bheisler.github.io/criterion.rs/book/criterion_rs.html
