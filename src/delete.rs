mod el;
use el::*;

use serde::*;
use csv::ReaderBuilder;

use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Brevo {
    #[serde(rename = "EMAIL")]
    pub email: String,
    #[serde(rename = "ADDED_TIME")]
    pub added_time: String,
    #[serde(rename = "MODIFIED_TIME")]
    pub modified_time: String,
}

pub fn parse_brevo(data: &str, mode: Mode) -> Result<Vec<String>, Box<dyn Error>> {
        let delimiter = match mode {
        Mode::Comma => b',',
        Mode::Semi => b';',
    };

    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(data.as_bytes());

    let mut emails: Vec<String> = Vec::new();

    for result in rdr.deserialize() {
        let brevo_record: Brevo = result?;
        emails.push(brevo_record.email);
    }

    Ok(emails)
}

pub fn filter_records(records: Vec<Record>, emails_to_remove: Vec<String>) -> Vec<Record> {
    records
        .iter()
        .cloned()
        .filter(|record| !emails_to_remove.contains(&record.email))
        .collect()
}

fn main() {
    let file_names: Vec<String> = std::env::args().skip(1).collect();

    if file_names.len() != 2 {
        panic!("Expected exactly two file names as arguments");
    }

    let mut file1 = fs::File::open(&file_names[0]).unwrap();
    let mut file2 = fs::File::open(&file_names[1]).unwrap();
    let mut first_line1 = String::new();
    let mut first_line2 = String::new();

    std::io::BufRead::read_line(&mut std::io::BufReader::new(&mut file1), &mut first_line1).unwrap();
    std::io::BufRead::read_line(&mut std::io::BufReader::new(&mut file2), &mut first_line2).unwrap();

    let (brevo_file, record_file) = if first_line1.len() < first_line2.len() {
        (&file_names[0], &file_names[1])
    } else {
        (&file_names[1], &file_names[0])
    };

    let brevo_data = fs::read_to_string(brevo_file).unwrap();
    let brevo_emails = parse_brevo(&brevo_data, Mode::Semi).unwrap();

    let record_data = fs::read_to_string(record_file).unwrap();
    let mut records = parse(&record_data).unwrap();

    records = filter_records(records, brevo_emails);

    fs::write("out.csv", dump(records).unwrap()).expect("Failed to write to file");
}
