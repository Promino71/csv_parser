use csv::{ReaderBuilder, StringRecord};
use std::{
    fs::File,
    io::{Read, Write},
};

/// *Returns*
/// [Result<Output filename, Error text>]
pub fn sort_csv(
    filename: String,
    is_reverse: bool,
    delimiter: char,
    pos: usize,
    output: String,
) -> Result<String, String> {
    if !filename.ends_with(".csv") {
        return Err(String::from("File not ending on .csv"));
    }

    // File reading
    let mut csv_data = String::new();

    let f = File::open(filename);
    if let Err(e) = f {
        return Err(e.to_string());
    }

    f.unwrap()
        .read_to_string(&mut csv_data)
        .expect("Failed to read file!");

    let first_row = csv_data.split('\n').collect::<Vec<_>>()[0];

    // CSV Parsing
    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter.to_string().as_bytes()[0])
        .from_reader(csv_data.as_bytes());

    let mut all_records: Vec<StringRecord> = Vec::new();

    for record in rdr.records() {
        let s_record = record.unwrap();
        if s_record.len() != 0 && !s_record.is_empty() {
            all_records.push(s_record);
        }
    }

    let recs = parse_and_sort(all_records, pos, is_reverse);
    save_sorted(recs, first_row, delimiter, &output)
}

fn save_sorted(
    items: Vec<StringRecord>,
    first_row: &str,
    delimiter: char,
    output_filename: &str,
) -> Result<String, String> {
    let f = File::create(output_filename);
    if let Err(e) = f {
        return Err(e.to_string());
    }

    let mut text = String::new();

    if first_row.ends_with('\n') {
        text.push_str(first_row);
    } else {
        text.push_str(&format!("{}\n", first_row));
    }

    for item in items {
        let last = item.into_iter().last().unwrap_or("");
        for i in item.into_iter() {
            if i == last {
                text.push_str(&format!("{i}"));
            } else {
                text.push_str(&format!("{i}{delimiter}"));
            }
        }
        text.push('\n');
    }

    if let Err(e) = f.unwrap().write_all(text.as_bytes()) {
        return Err(e.to_string());
    }

    Ok(format!("Successfully wrote to {output_filename}"))
}

fn parse_and_sort(items: Vec<StringRecord>, pos: usize, is_reverse: bool) -> Vec<StringRecord> {
    let mut items = items;
    items.sort_by(|sr1, sr2| {
        let range1 = sr1.range(pos).expect("Pos invalid!");
        let item1 = sr1.as_slice()[range1].to_string();

        let range2 = sr2.range(pos).expect("Pos invalid!");
        let item2 = sr2.as_slice()[range2].to_string();

        if is_reverse {
            item2.partial_cmp(&item1).unwrap()
        } else {
            item1.partial_cmp(&item2).unwrap()
        }
    });

    items
}
