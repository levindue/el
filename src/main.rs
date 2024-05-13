use std::error::Error;
use std::fs;
use serde::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    #[serde(rename = "Kontakt ID")]
    pub kontakt_id: u32,
    #[serde(rename = "Anrede")]
    pub anrede: String,
    #[serde(rename = "Titel")]
    pub titel: String,
    #[serde(rename = "Vorname")]
    pub vorname: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Email des Kontakts")]
    pub email: String,
    #[serde(rename = "Adresszusatz (Kontakt)")]
    pub adresszusatz: String,
    #[serde(rename = "Organisations ID")]
    pub organisation_id: Option<u32>,
    #[serde(rename = "Organisation")]
    pub organisation: Option<String>,
    #[serde(rename = "Straße mit Hausnummer")]
    pub strasse: String,
    #[serde(rename = "PLZ")]
    pub plz: String,
    #[serde(rename = "Ort")]
    pub ort: String,
    #[serde(rename = "Land")]
    pub land: String,
    #[serde(rename = "Postversand ja/nein")]
    pub postversand: String,
    #[serde(rename = "E-Mail-Werbung ja/nein")]
    pub email_werbung: String,
}

pub fn parse(data: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    let mut records: Vec<Record> = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn dump(records: Vec<Record>) -> Result<String, Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(vec![]);

    for record in records {
        wtr.serialize(record)?;
    }

    let csv_data = String::from_utf8(wtr.into_inner()?.to_vec())?;

    Ok(csv_data)
}

#[derive(Deserialize)]
struct Config {
    filter: Filter,
    sortierung: Sortierung,
    duplikate: Duplikate,
}

#[derive(Deserialize)]
struct Filter {
    land: String,
    postversand: bool,
}

#[derive(Deserialize)]
struct Sortierung {
    schlüssel: Vec<String>,
    reihenfolge: String, // aufsteigend/absteigend
}

#[derive(Deserialize)]
struct Duplikate {
    schlüssel: Vec<String>, 
}

fn main() {
    let mut all_content = String::new();

    for arg in std::env::args().skip(1) {
        if let Ok(content) = fs::read_to_string(&arg) {
            all_content.push_str(&content);
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
