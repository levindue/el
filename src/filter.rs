mod el;
use el::*;

use std::collections::HashMap;
use std::fs;

fn main() {
    if std::env::args().len() < 2 {
        std::process::exit(-1);
    }

    let mut all_content = String::new();
    let mut file_names: Vec<String> = Vec::new();

    for arg in std::env::args().skip(1) {
        if let Ok(content) = fs::read_to_string(&arg) {
            all_content.push_str(&content);
            file_names.push(arg);
        }
    }

    let mode = determine_mode(&all_content);

    let records = match parse(&all_content, mode) {
        Ok(records) => records,
        Err(err) => {
            eprintln!("Fehler beim lesen der Eingabedatei: {}", err);
            return;
        }
    };

    let mut white: Vec<Record> = Vec::new();

    for record in records.iter() {
        if record.postversand == "Ja"
            && record.land == "Deutschland"
            && record.organisation_id.is_some()
            && record.organisation.is_some()
        {
            white.push(record.clone());
        }
    }
    white.sort_by(|a, b| {
        if a.plz != b.plz {
            a.plz.cmp(&b.plz)
        } else {
            a.organisation_id.unwrap().cmp(&b.organisation_id.unwrap())
        }
    });

    let mut brown: Vec<Record> = Vec::new();
    let mut white_counts: HashMap<u32, usize> = HashMap::new();

    // Count the occurrences of each organisation_id
    for record in white.iter() {
        if let Some(org_id) = record.organisation_id {
            let count = white_counts.entry(org_id).or_insert(0);
            *count += 1;
        }
    }

    // Process the records for each organisation_id that has more than 3 records
    for (&org_id, &count) in white_counts.iter() {
        if count > 3 {
            let to_round = (count + 2) / 3 * 3;  // round up to the nearest multiple of 3
            let last_records: Vec<_> = white
                .iter()
                .rev()
                .filter(|r| r.organisation_id == Some(org_id))
                .take(to_round)
                .cloned()
                .collect();

            for r in last_records.iter().rev() {
                brown.push(r.clone());
            }
        }
    }

    fs::write("umschlaege_weiss.csv", dump(white).unwrap()).unwrap();
    fs::write("umschlaege_braun.csv", dump(brown).unwrap()).unwrap();
}
