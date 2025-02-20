use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub payment_id: String,
    pub gateway: String,
    pub settle_co: String,
    pub merchant_id: String,
    pub paid_date: String,
    pub rcv_date: String,
    pub sales_date: String,
    pub auth: String,
    pub card_no: String,
    pub sales_amt: i32,
    pub merchant_fee: i32,
    pub other_fee: i32,
    pub vat: i32,
    pub paid_amt: i32,
    pub cc_gubun: String,
    pub sales_gubun: String,
    pub maeib_gubun: String,
}