use std::{
    fs::File,
    io::{BufReader, BufWriter, Read},
};

use json_eater::CsvWriter;

fn main() {
    let filename = std::env::args().nth(1);

    let reader = match filename {
        Some(filename) => {
            let f = File::open(filename).unwrap();
            BufReader::new(Box::new(f) as Box<dyn Read>)
        },
        None => BufReader::new(Box::new(std::io::stdin()) as Box<dyn Read>),
    };

    let writer = BufWriter::new(std::io::stdout());
    json_eater::eat_json_read(reader, CsvWriter::new(std::io::stdout()))
        .unwrap();
}
