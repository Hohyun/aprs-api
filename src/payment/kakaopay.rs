use std::env;
use serde::Deserialize;
use crate::payment::common::Payment;
use crate::payment::util::save_csv_file;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct KakaoSettleFileInfo {
    url: String,
    expires_at: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct KakaoPaySettleData {
    #[serde(rename = "type")]
    type_field: String,
    bucket_id: String,
    target_date: String,
    file_created_at: String,
    partner: String,
    statistics: KakaoStatistics,
    data: Vec<KakaoTransactionItem>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct KakaoStatistics {
    payment: Stat,
    cancel: Stat,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Stat {
    count: i32,
    amount: i32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct KakaoTransactionItem {
    cid: String,
    tid: String,
    aid: String,
    approved_at: String,
    partner_order_id: String,
    payment_method_type: String,
    payment_action_type: String,
    currency: String,
    amount: i32,
    point_amount: i32,
    discount_amount: i32,
    green_deposit: i32,
    fee: i32,
    fee_vat: i32,
    amount_payable: i32,
    deposit_date: String,
    payment_detail_action_type: String,
    interest_free_fee: i32,
    interest_free_fee_vat: i32,
}

pub async fn get_and_save_kakaopay_settle_data(sales_date: &str) -> anyhow::Result<()> {
    let trx_list = get_kakaopay_settle_data(sales_date).await?;
    let _ = save_csv_file("NK", sales_date, trx_list)?;
    Ok(())
}

async fn get_kakaopay_settle_data(sales_date: &str) -> anyhow::Result<Vec<Payment>> {
    let url = get_kakaopay_settle_file_url(sales_date).await?;
    // download file from url
    let resp = reqwest::get(url)
        .await?
        .json::<KakaoPaySettleData>()
        .await?; 

    println!("Kakaopay --  date: {},  {:?}", resp.target_date, resp.statistics);

    let mut payment_list: Vec<Payment> = Vec::new();
    for trx in resp.data {
        let payment = convert_to_payment_kakaopay(sales_date, trx);
        payment_list.push(payment);
    }

    Ok(payment_list)
}

async fn get_kakaopay_settle_file_url(sales_date: &str) -> anyhow::Result<String> {
    dotenvy::dotenv()?;
    let pg_key = env::var("NK_PG_KEY")?;
    let bucket_id = env::var("NK_BUCKET_ID")?;
    let url = format!("https://biz-dapi.kakaopay.com/files/v1/settlements/history?target_date={}&bucket_id={}", sales_date, bucket_id);

    let client = reqwest::Client::new();
    let resp = client.get(url)
        .header("Authorization", format!("PG_BIZAPI_KEY {}", pg_key))
        .send()
        .await?
        .json::<KakaoSettleFileInfo>()
        .await
        .unwrap_or(KakaoSettleFileInfo { url: "".to_string(), expires_at: "".to_string() });

    println!("{:?}", resp);

    Ok(resp.url)
}

fn convert_to_payment_kakaopay (sales_date: &str, trx: KakaoTransactionItem) -> Payment {
    let format_date = |s: &str| -> String {
        format!("{}-{}-{}", &s[0..4], &s[4..6], &s[6..8])
    };
    Payment {
        payment_id: format!("NK_SP_{}", &sales_date),
        gateway: "SP".to_string(),
        settle_co: "NK".to_string(),
        merchant_id: "IBSNKKRW".to_string(),
        paid_date: trx.deposit_date,
        rcv_date: format_date(sales_date),
        sales_date: format_date(sales_date),
        auth: trx.tid,
        card_no: "".to_string(),
        sales_amt: trx.amount,
        merchant_fee: trx.fee,
        other_fee: trx.interest_free_fee,
        vat: trx.fee_vat,
        paid_amt: trx.amount_payable,
        cc_gubun: trx.payment_method_type,
        sales_gubun: trx.payment_action_type,
        maeib_gubun: "OK".to_string(),
    }
}