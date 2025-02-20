use std::path::Path;
use crate::payment::common::Payment;

pub fn save_csv_file(settleco: &str, sales_date: &str, payments: Vec<Payment>) -> anyhow::Result<()> {
    let filename = format!("./data/payment/extract/work/_{}_SP_{}.csv", settleco, sales_date);
    let path = Path::new(filename.as_str());
    let mut wtr = csv::Writer::from_path(path)?;
    for p in payments {
        wtr.serialize(p)?;
    }
    wtr.flush()?;
    Ok(())
}
