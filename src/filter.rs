mod el;
use el::*;

use std::collections::HashMap;
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

    let mut brown: Vec<Record> = Vec::new();
    let mut white_counts: HashMap<u32, usize> = HashMap::new();

    for record in white.iter() {
        if let Some(org_id) = record.organisation_id {
            let count = white_counts.entry(org_id).or_insert(0);
            *count += 1;
            if *count % 3 == 0 {
                let last_three_records: Vec<_> = white
                    .iter()
                    .rev()
                    .filter(|r| r.organisation_id == Some(org_id))
                    .take(3)
                    .cloned()
                    .collect();

                for r in last_three_records.iter().rev() {
                    brown.push(r.clone());
                }

                *count = 0;
            }
        }
    }

    fs::write("umschlaege_weiss.csv", dump(white).unwrap()).unwrap();
    fs::write("umschlaege_braun.csv", dump(brown).unwrap()).unwrap();
}
