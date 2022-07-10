use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use criterion::{
    criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion,
    Throughput,
};
use fetch_data::FetchData;
use humansize::{file_size_opts::DECIMAL, FileSize};
use json_eater::{eat_json, eat_json_read, CsvWriter};
use once_cell::sync::Lazy;
use tempfile::tempfile;

static SAMPLE_DATA: Lazy<FetchData> = Lazy::new(|| {
    FetchData::new(
        include_str!("sample-data.txt"),
        "http://docs.smarthealthit.org/dstu2-examples/examples/",
        "JSON_EATER_DATA_DIR",
        "ai",
        "hotg",
        "json-eater",
    )
});

/// Time how long it takes to consume a JSON file and save it as a CSV of
/// key/value pairs. This is the typical use case for anyone using a
/// command-line tool.
pub fn file_to_file(c: &mut Criterion) {
    let path = SAMPLE_DATA
        .fetch_file("diagnosticreport-examples-lab-text.canonical.json")
        .unwrap();
    let length = path.metadata().unwrap().len();
    let name = format!("Index {}", length.file_size(DECIMAL).unwrap());

    c.bench_function(&name, |b| {
        b.iter_batched(
            || {
                (
                    BufReader::new(File::open(&path).unwrap()),
                    BufWriter::new(tempfile().unwrap()),
                )
            },
            |(input, mut dest)| {
                eat_json_read(input, CsvWriter::new(&mut dest)).unwrap();
                dest.flush().unwrap();
            },
            BatchSize::SmallInput,
        );
    });
}

/// How much overhead does writing to a CSV take compared with doing nothing.
pub fn writing_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("writer overhead");
    let path = SAMPLE_DATA
        .fetch_file("diagnosticreport-examples-lab-text.canonical.json")
        .unwrap();
    let data = std::fs::read_to_string(&path).unwrap();

    group.throughput(Throughput::Bytes(data.len() as _));

    group.bench_function("baseline", |b| {
        b.iter(|| {
            let mut counter = Noop::default();
            eat_json(data.as_bytes(), &mut counter).unwrap();

            counter
        });
    });

    group.bench_function("csv in memory", |b| {
        b.iter(|| {
            let mut buffer = Vec::new();
            eat_json(data.as_bytes(), CsvWriter::new(&mut buffer)).unwrap();
        });
    });

    group.bench_function("csv on disk", |b| {
        b.iter_batched(
            || BufWriter::new(tempfile().unwrap()),
            |mut f| {
                eat_json(data.as_bytes(), CsvWriter::new(&mut f)).unwrap();
                f.flush().unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Determine the overhead of reading a file versus an in-memory array.
pub fn input_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("input format");
    let path = SAMPLE_DATA
        .fetch_file("diagnosticreport-examples-lab-text.canonical.json")
        .unwrap();
    let data = std::fs::read_to_string(&path).unwrap();

    group.throughput(Throughput::Bytes(data.len() as _));

    group.bench_function("in-memory buffer", |b| {
        b.iter(|| {
            let mut counter = Noop::default();
            eat_json(data.as_bytes(), &mut counter).unwrap();

            counter
        });
    });

    group.bench_function("file reader", |b| {
        b.iter_batched(
            || BufReader::new(File::open(&path).unwrap()),
            |f| {
                let mut counter = Noop::default();
                eat_json_read(f, &mut counter).unwrap();
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// How does processing time change as the file size increases?
pub fn input_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("input size");
    let files = [
        "diagnosticreport-examples-lab-text.canonical.json",
        "conceptmap-example-specimen-type.canonical.json",
        "allergyintolerance-example.canonical.json",
        "account-example.canonical.json",
    ];

    for filename in files {
        let path = SAMPLE_DATA.fetch_file(filename).unwrap();
        let data = std::fs::read(&path).unwrap();
        group.throughput(Throughput::Bytes(data.len() as _));

        group.bench_with_input(
            BenchmarkId::from_parameter(data.len().file_size(DECIMAL).unwrap()),
            data.as_slice(),
            |b, data| {
                b.iter(|| {
                    let mut counter = Noop::default();
                    eat_json(data, &mut counter).unwrap();

                    counter
                });
            },
        );
    }

    group.finish();
}

#[derive(Debug, Default)]
struct Noop;

impl json_eater::Visitor<'_> for Noop {
    fn visit_any(
        &mut self,
        _path: &json_eater::Path,
        _value: json_eater::Value<'_>,
    ) {
    }
}

criterion_group!(
    benches,
    input_format,
    writing_overhead,
    input_size,
    file_to_file
);
criterion_main!(benches);
