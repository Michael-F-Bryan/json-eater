use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use fetch_data::FetchData;
use json_eater::{eat_json, eat_json_read};
use once_cell::sync::Lazy;
use tempfile::tempfile;

static SAMPLE_DATA: Lazy<FetchData> = Lazy::new(|| {
    FetchData::new(
        include_str!("sample-data.txt"),
        "https://www.hl7.org/fhir/",
        "JSON_EATER_DATA_DIR",
        "ai",
        "hotg",
        "json-eater",
    )
});

pub fn csv_writing_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("visitor overhead");
    let path = SAMPLE_DATA.fetch_file("patient-example.json").unwrap();
    let data = std::fs::read_to_string(&path).unwrap();

    group.throughput(Throughput::Bytes(data.len() as _));

    group.bench_function("noop", |b| {
        b.iter(|| {
            let mut counter = Counter::default();
            eat_json(data.as_bytes(), &mut counter).unwrap();

            counter
        });
    });

    group.bench_function("save as csv", |b| {
        b.iter(|| {
            let mut f = tempfile().unwrap();

            eat_json(data.as_bytes(), CsvWriter(BufWriter::new(&mut f)))
                .unwrap();

            f.flush().unwrap();
        });
    });

    group.finish();
}

pub fn input_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("input format");
    let path = SAMPLE_DATA.fetch_file("patient-example.json").unwrap();
    let data = std::fs::read_to_string(&path).unwrap();

    group.throughput(Throughput::Bytes(data.len() as _));

    group.bench_function("in-memory buffer", |b| {
        b.iter(|| {
            let mut counter = Counter::default();
            eat_json(data.as_bytes(), &mut counter).unwrap();

            counter
        });
    });

    group.bench_function("file reader", |b| {
        b.iter(|| {
            let mut counter = Counter::default();
            let f = File::open(&path).unwrap();
            eat_json_read(BufReader::new(f), &mut counter).unwrap();

            counter
        });
    });

    group.finish();
}

#[derive(Debug, Default)]
struct Counter(usize);

impl json_eater::Visitor for Counter {
    fn visit_any(
        &mut self,
        _path: &json_eater::Path,
        _value: json_eater::Value<'_>,
    ) {
        self.0 += 1;
    }
}

struct CsvWriter<W>(W);

impl<W: Write> json_eater::Visitor for CsvWriter<W> {
    fn visit_any(
        &mut self,
        path: &json_eater::Path,
        value: json_eater::Value<'_>,
    ) {
        let _ = writeln!(self.0, "{}, {}", path, value);
    }
}

criterion_group!(benches, input_format, csv_writing_overhead);
criterion_main!(benches);
