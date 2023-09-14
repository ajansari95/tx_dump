use std::error::Error;
use std::fmt::Display;
use csv::Writer;
use prettytable::{Cell, Row, Table};
use serde::Serialize;

/// Dumps the given data to a CSV file.
pub fn dump_to_csv<T: Serialize>(data: &Vec<T>, filename: String) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(filename)?;
    for item in data {
        writer.serialize(item)?;
    }
    writer.flush()?;

    Ok(())
}

/// display_pretty Dumps the given data to a Display using table
pub fn display_pretty<T: Display>(data: &Vec<T>) {
    let mut table = Table::new();
    // Add a row per item
    for item in data.iter() {
        table.add_row(Row::new(vec![
            Cell::new(&item.to_string()),
        ]));
    }
    // Print the table to stdout
    table.printstd();
}