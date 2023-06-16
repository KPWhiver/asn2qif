use std::io::Write;
use chrono::NaiveDate;
use serde::Deserialize;
use clap::Parser;
use std::path::PathBuf;
use std::fs::File;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(with = "asn_date_format")]
    pub date: NaiveDate,
    pub account: String,
    pub payee_account: String,
    pub payee: String,
    pub address: String,
    pub postal_code: String,
    pub city: String,
    pub balance_before: String,
    pub account_currency: String,
    pub currency: String,
    pub amount: String,
    #[serde(with = "asn_date_format")]
    pub journal_date: NaiveDate,
    #[serde(with = "asn_date_format")]
    pub currency_date: NaiveDate,
    pub code: u16,
    pub kind: String,
    pub tracking_number: u64,
    pub short_description: String,
    pub long_description: String,
    pub block_number: u64,
}

mod asn_date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Deserializer};
    const FORMAT: &'static str = "%d-%m-%Y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Parser,Default,Debug)]
struct Arguments {
    csv_file: String
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    let csv_file = File::open(&args.csv_file)?;

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_file);

    let mut qif_path = PathBuf::from(args.csv_file);
    qif_path.set_extension("qif");

    let mut qif_file = File::create(qif_path)?;

    writeln!(qif_file, "!Type:Bank")?;

    for line in csv_reader.deserialize() {
        let mut transaction: Transaction = line?;
        if transaction.long_description.len() > 0 {
            transaction.long_description.remove(0);
            transaction.long_description.pop();
        }

        writeln!(qif_file, "T{}", transaction.amount)?;
        writeln!(qif_file, "D{}", transaction.date.format("%m/%d/%Y"))?;
        writeln!(qif_file, "P{}", transaction.payee)?;
        writeln!(qif_file, "M{}", transaction.long_description)?;
        writeln!(qif_file, "^")?;
    };

    Ok(())
}
