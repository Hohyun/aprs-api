use std::env;
use serde::Deserialize;
use crate::payment::common::Payment;
use crate::payment::util::save_csv_file;
use reqwest::header;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayResponse {
    pub code: String,
    pub message: String,
    pub body: NaverPaySettleData,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NaverPaySettleData {
    pub list: Vec<NaverPayTransactionItem>,
    pub total_count: i32,
    pub response_count: i32,
    pub total_page_count: Option<i32>,
    pub current_page: i32,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NaverPayTransactionItem {
    pub settle_basis_date: String,
    pub settle_expect_date: String,
    pub order_id: String,
    pub product_order_type: String,
    pub settle_type: String,
    pub pay_settle_amount: i32,
    pub total_commission_amount: i32,
    pub free_installment_commission_amount: i32,
    pub selling_interlock_commission_amount: i32,
    pub settle_expect_amount: i32,
}



pub async fn get_and_save_naverpay_settle_data(sales_date: &str) -> anyhow::Result<()> {
    let trx_list = get_naverpay_settle_data(sales_date).await?;
    let _ = save_csv_file("NP", sales_date, trx_list)?;
    Ok(())
}

async fn get_naverpay_settle_data(sales_date: &str) -> anyhow::Result<Vec<Payment>> {
    dotenvy::dotenv()?;

    let mut payment_list: Vec<Payment> = Vec::new();

    for n in 1..=5 {
        let api_key = env::var("NP_APIKEY_".to_string() + n.to_string().as_str())?;
    
        let mut trx_list: Vec<NaverPayTransactionItem> = Vec::new();

        let mut headers = header::HeaderMap::new();
        headers.insert("X-Naver-API-Key", header::HeaderValue::from_str(api_key.as_str()).unwrap());
        headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        
        let mut page_no = 1;
        let mut total_page = 1;

        while page_no <= total_page {
            
            let url = format!("https://apis.naver.com/naverpaysettle-payment/naverpaysettle/v1/settlements/by-case?periodType=SETTLE_BASIS_DATE&startDate={}&endDate={}&pageNumber={}", sales_date, sales_date, page_no);

            let resp = client.get(&url)
                .send()
                .await?
                .json::<NaverPayResponse>()
                .await?;
        
            let resp_body = &resp.body;
            // println!("Naverpay -- APIKEY{}, date: {},  page_no: {}, count: {}", n, sales_date, page_no, &resp_body.list.len());

            for trx in &resp_body.list {
                trx_list.push(trx.clone());
            }
            
            total_page = resp_body.total_page_count.unwrap_or(total_page);
            page_no += 1;

        }

        for trx in trx_list {
            let payment = convert_to_payment_naverpay(trx);
            payment_list.push(payment);
        }
    }
    println!("Naverpay --  date: {},  total count: {}", sales_date, payment_list.len());
    
    Ok(payment_list)
}


fn convert_to_payment_naverpay (trx: NaverPayTransactionItem) -> Payment {

    let format_date = |s: &String| -> String {
        format!("{}-{}-{}", &s[0..4], &s[4..6], &s[6..8])
    };
    
    Payment {
        payment_id: format!("NP_SP_{}", &trx.settle_basis_date),
        gateway: "SP".to_string(),
        settle_co: "NP".to_string(),
        merchant_id: "IBSNPKRW".to_string(),
        paid_date: format_date(&trx.settle_expect_date),
        rcv_date: format_date(&trx.settle_basis_date),
        sales_date: format_date(&trx.settle_basis_date),
        auth: trx.order_id,
        card_no: "".to_string(),
        sales_amt: trx.pay_settle_amount,
        merchant_fee: trx.total_commission_amount,
        other_fee: trx.free_installment_commission_amount + trx.selling_interlock_commission_amount,
        vat: 0,
        paid_amt: trx.settle_expect_amount,
        cc_gubun: trx.product_order_type,
        sales_gubun: trx.settle_type,
        maeib_gubun: "OK".to_string(),
    }
}