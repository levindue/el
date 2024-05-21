use serde::*;
use std::error::Error;
use csv::ReaderBuilder;

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

pub enum Mode {
    Comma,
    Semi,
}

pub fn determine_mode(content: &str) -> Mode {
    for c in content.chars() {
        match c {
            ',' => return Mode::Comma,
            ';' => return Mode::Semi,
            _ => continue,
        }
    }

    Mode::Comma
}

pub fn parse(data: &str, mode: Mode) -> Result<Vec<Record>, Box<dyn Error>> {
    let delimiter = match mode {
        Mode::Comma => b',',
        Mode::Semi => b';',
    };

    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(data.as_bytes());

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
