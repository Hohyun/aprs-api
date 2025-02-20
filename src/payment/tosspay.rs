use std::collections::HashMap;
use serde::Deserialize;
use std::env;
use crate::payment::common::Payment;
use crate::payment::util::save_csv_file;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TossPaySettleData {
    pub transaction_list: Vec<TossPayTransactionItem>,
    pub next_cursor: String,
    pub total_amount: i32,
    pub total_fee: i32,
    pub total_vat: i32,
    pub total_fee_vat_sum: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TossPayTransactionItem {
    pub transaction_type: String,
    pub transaction_id: String,
    pub pay_token: String,
    pub product_desc: String,
    pub order_no: String,
    pub pay_method: String,
    pub amount: i32,
    pub fee: i32,
    pub vat: i32,
    pub fee_vat_sum: i32,
    pub settle_date: String,
    pub due_date: String,
}


pub async fn get_and_save_tosspay_settle_data(sales_date: &str) -> anyhow::Result<()> {
    let tp_trx_list = get_tosspay_settle_data(sales_date).await?;
    let _ = save_csv_file("TP", sales_date, tp_trx_list)?;
    Ok(())
}

async fn get_tosspay_settle_data(sales_date: &str) -> anyhow::Result<Vec<Payment>> {
    dotenvy::dotenv()?;
    let url = "https://pay.toss.im/api/v2/settlement-details";
    let mut trx_list: Vec<TossPayTransactionItem> = Vec::new();
    
    for n in 1..=3 {
        let mut map = HashMap::new();
        let api_key = env::var("TP_APIKEY_".to_string() + n.to_string().as_str())?;
        map.insert("apiKey", api_key.as_str());
        map.insert("dateType", "SETTLE");
        map.insert("baseDate", sales_date);
        
        let client = reqwest::Client::new();
        let data = client.post(url)
        .json(&map)
        .send()
        .await?
        .json::<TossPaySettleData>()
        .await?;

        for trx in data.transaction_list {
            trx_list.push(trx);
        }
    }

    let mut payment_list: Vec<Payment> = Vec::new();
    for trx in trx_list {
        let payment = convert_to_payment_tosspay(trx);
        payment_list.push(payment);
    }
    println!("Tosspay -- date: {}, count: {}", &sales_date, payment_list.len());

    Ok(payment_list)
}

fn convert_to_payment_tosspay(trx: TossPayTransactionItem) -> Payment {
    let v: Vec<&str> = trx.order_no.split("_").collect();  
    let format_date = |s: &str| -> String {
        format!("{}-{}-{}", &s[0..4], &s[4..6], &s[6..8])
    };
    Payment {
        payment_id: format!("TP_SP_{}", &trx.settle_date),
        gateway: "SP".to_string(),
        settle_co: "TP".to_string(),
        merchant_id: "IBSTPKRW".to_string(),
        paid_date: format_date(&trx.due_date),
        rcv_date: format_date(&trx.settle_date),
        sales_date: format_date(&trx.settle_date),
        auth: v[1].to_string(),
        card_no: "".to_string(),
        sales_amt: trx.amount,
        merchant_fee: trx.fee * -1,
        other_fee: 0,
        vat: trx.vat * -1,
        paid_amt: trx.amount + trx.fee + trx.vat,
        cc_gubun: trx.pay_method,
        sales_gubun: "".to_string(),
        maeib_gubun: "OK".to_string(),
    }
}