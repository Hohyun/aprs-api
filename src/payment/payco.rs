use std::env;
use csv::ReaderBuilder;
use crate::payment::common::Payment;
use crate::payment::util::save_csv_file;

pub async fn get_and_save_payco_settle_data(sales_date: &str) -> anyhow::Result<()> {
    let trx_list = get_payco_settle_data(sales_date).await?;
    let _ = save_csv_file("PC", sales_date, trx_list)?;
    Ok(())
}

async fn get_payco_settle_data(sales_date: &str) -> anyhow::Result<Vec<Payment>> {
    dotenvy::dotenv()?;
    let service_code = "SB_PAY_D";
    let customer_id = env::var("PC_CUSTOMER_ID")?;
    let api_key = env::var("PC_APIKEY")?;
    let url = format!("https://api-partner-bill.payco.com/pgTradeCheck/download/pay?serviceCode={}&mrcCode={}&ymd={}&token={}&version=1.0", service_code, customer_id, sales_date, api_key);

    let mut payment_list: Vec<Payment> = Vec::new();
    //get data from url
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;

    // println!("Payco --  date: {},  {:?}", sales_date, &resp);

    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(resp.as_bytes());
    
    for result in reader.records() {
        let record = result?;
        payment_list.push(convert_to_payment_payco(record));
    }

    Ok(payment_list)
}


fn convert_to_payment_payco (record: csv::StringRecord) -> Payment {

    let format_date = |s: &str| -> String {
        format!("{}-{}-{}", &s[0..4], &s[4..6], &s[6..8])
    };
    
    Payment {
        payment_id: format!("PC_SP_{}", record.get(0).unwrap()),
        gateway: "SP".to_string(),
        settle_co: "PC".to_string(),
        merchant_id: "IBSPCKRW".to_string(),
        paid_date: format_date(record.get(0).unwrap()),
        rcv_date: format_date(record.get(3).unwrap()),
        sales_date: format_date(record.get(3).unwrap()),
        auth: record.get(10).unwrap().to_string(),
        card_no: "".to_string(),
        sales_amt: record.get(16).unwrap().parse::<f64>().unwrap() as i32,
        merchant_fee: record.get(17).unwrap().parse::<f64>().unwrap() as i32,
        other_fee: 0,
        vat: record.get(20).unwrap().parse::<f64>().unwrap() as i32,
        paid_amt: record.get(21).unwrap().parse::<f64>().unwrap() as i32,
        cc_gubun: "".to_string(),
        sales_gubun: "".to_string(),
        maeib_gubun: "OK".to_string(),
    }
}