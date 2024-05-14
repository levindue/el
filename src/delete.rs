mod el;
use el::*;

use std::fs;

fn main() {
    let mut all_content = String::new();
    let mut file_names: Vec<String> = Vec::new();

    for arg in std::env::args().skip(1) {
        if let Ok(content) = fs::read_to_string(&arg) {
            all_content.push_str(&content);
            file_names.push(arg);
        }
    }

    let records = match parse(&all_content) {
        Ok(records) => records,
        Err(err) => {
            eprintln!("Error parsing input files: {}", err);
            return;
        }
    };

    let mut unique_records: Vec<Record> = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();
    for record in records {
        if seen_ids.insert(record.kontakt_id) {
            unique_records.push(record);
        }
    }

    let csv_data = match dump(unique_records) {
        Ok(csv_data) => csv_data,
        Err(err) => {
            eprintln!("Error dumping records to CSV: {}", err);
            return;
        }
    };

    if let Err(err) = fs::write("out.csv", csv_data) {
        eprintln!("Error writing to file: {}", err);
    } else {
        println!("Output written to out.csv");
    }
}
