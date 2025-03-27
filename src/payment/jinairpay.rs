use serde::Deserialize;
use std::env;
use crate::payment::common::Payment;
use crate::payment::util::save_csv_file;

#[derive(Deserialize, Debug)]
pub struct JinairPaySettleData {
    pub total_count: i32,
    pub list: Vec<JinairPayTransactionItem>,
}

#[derive(Deserialize, Debug)]
pub struct JinairPayTransactionItem {
    pub approve_date: String,
    pub cancel_date: String,
    pub settle_date: String,
    pub pgcode: String,
    pub user_id: String,
    pub user_name: String,
    pub tid: String,
    pub order_no: String,
    pub amount: i32,
    pub settle_amount: i32,
    pub status_code: i32,
    pub clientid: String,
    pub pay_type: i32,
    pub catid: String,
    pub usestate: i32,
    pub cid: String,
    pub company_regnum: String,
}

pub async fn get_and_save_jinairpay_settle_data(sales_date: &str) -> anyhow::Result<()> {
    let jp_trx_list = get_jinairpay_settle_data(sales_date).await?;
    let _ = save_csv_file("JP", sales_date, jp_trx_list)?;
    Ok(())
}


async fn get_jinairpay_settle_data(sales_date: &str) -> anyhow::Result<Vec<Payment>> {
    dotenvy::dotenv()?;
    let today = chrono::Local::now();
    let today = today.format("%Y%m%d").to_string();
    let url = format!("https://pgapi.payletter.com/v1.0/payments/settle?client_id=jinair&date={}&date_settle={}", sales_date, today);

    let mut trx_list: Vec<JinairPayTransactionItem> = Vec::new();
    
    for n in 1..=2 {
        let api_key = env::var("JP_APIKEY_".to_string() + n.to_string().as_str())?;
        
        let client = reqwest::Client::new();
        let resp = client.get(&url)
            .header("Authorization", format!("PLKEY {}", api_key))
            .send()
            .await?
            .json::<JinairPaySettleData>()
            .await?;

        for trx in resp.list {
            trx_list.push(trx);
        }
    }

    let mut payment_list: Vec<Payment> = Vec::new();
    for trx in trx_list {
        let payment = convert_to_payment_jinairpay(trx);
        payment_list.push(payment);
    }
    println!("JinairPay -- date: {}, count: {}", &sales_date, payment_list.len());

    Ok(payment_list)
}

fn convert_to_payment_jinairpay(trx: JinairPayTransactionItem) -> Payment {  
    let format_date = |s: &str| -> String {
        format!("{}-{}-{}", &s[0..4], &s[4..6], &s[6..8])
    };
    Payment {
        payment_id: format!("JP_SP_{}", &trx.approve_date),
        gateway: "SP".to_string(),
        settle_co: "JP".to_string(),
        merchant_id: "IBSJPKRW".to_string(),
        paid_date: format_date(&trx.settle_date),
        rcv_date: format_date(&trx.approve_date),
        sales_date: format_date(&trx.approve_date),
        auth: trx.cid,
        card_no: "".to_string(),
        sales_amt: trx.amount,
        merchant_fee: trx.amount - trx.settle_amount,
        other_fee: 0,
        vat: 0,
        paid_amt: trx.settle_amount,
        cc_gubun: trx.pgcode,
        sales_gubun: trx.usestate.to_string(),
        maeib_gubun: "".to_string(),
    }
}