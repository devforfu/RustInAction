use std::fs;
use std::io::stdin;

fn main() {
    let filename = get_filename();
    let mut records = read_records(&filename);
    records.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if cfg!(debug_assertions) {
        eprintln!("debug: {:#?}", records);
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Record {
    name: String,
    length: f32,
}

impl Record {
    fn new(name: &str, length: f32) -> Self {
        Self { name: name.to_string(), length }
    }
}

fn get_filename() -> String {
    println!("Enter file name: ");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("cannot read file name");
    buf.trim().to_string()
}

fn read_records(filename: &str) -> Vec<Record> {
    let result = fs::read_to_string(filename);
    let mut records: Vec<Record> = Vec::new();
    if result.is_ok() {
        let content = result.ok().unwrap();
        for (i, line) in content.split('\n').enumerate() {
            if i == 0 || line.trim().is_empty() {
                continue;
            }
            let fields: Vec<_> = line.split(',').map(|f| f.trim()).collect();
            if cfg!(debug_assertions) {
                eprintln!("debug: {:?} -> {:?}", line, fields);
            }
            let name = fields[0];
            if let Ok(length) = fields[1].parse::<f32>() {
                records.push(Record::new(name, length))
            }
        }
    }
    records
}