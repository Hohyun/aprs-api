use clap::Parser;
use chrono::prelude::*;

pub mod payment;



#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Settle date (YYYYMMDD)
    #[clap(short, long, default_value_t = day_before_today(3))]
    date: String,

    /// Settle Company Code: JP, NK, NP, PC, TP, SP, ALL
    #[clap(short, long, default_value_t = String::from("ALL"))]
    paycode: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let args = Args::parse();

    println!("{:?}", args);
    
    match args.paycode.as_str() {
        "TP" => {
            let _ = crate::payment::tosspay::get_and_save_tosspay_settle_data(&args.date).await?;
        },
        "NK" => {
            let _ = crate::payment::kakaopay::get_and_save_kakaopay_settle_data(&args.date).await?;
        },
        "NP" => {
            let _ = crate::payment::naverpay::get_and_save_naverpay_settle_data(&args.date).await?;
        },
        "PC" => {
            let _ = crate::payment::payco::get_and_save_payco_settle_data(&args.date).await?;
        },
        "JP" => {
            let _ = crate::payment::jinairpay::get_and_save_jinairpay_settle_data(&args.date).await?;
        },
        "ALL" => {
            // I would like to run all payment settlement data in parallel
            let tp = crate::payment::tosspay::get_and_save_tosspay_settle_data(&args.date);
            let nk = crate::payment::kakaopay::get_and_save_kakaopay_settle_data(&args.date);
            let np = crate::payment::naverpay::get_and_save_naverpay_settle_data(&args.date);
            let pc = crate::payment::payco::get_and_save_payco_settle_data(&args.date);
            let jp = crate::payment::jinairpay::get_and_save_jinairpay_settle_data(&args.date);
            // wait for all futures to complete
            let _ = tokio::try_join!(tp, nk, np, pc, jp)?;
        },
        _ => {
            println!("Not supported yet: {}", args.paycode);
        }
    }

    Ok(())
}

fn day_before_today(delta: i64) -> String {
    let today = Local::now();
    let four_days_ago = today - chrono::Duration::days(delta);
    four_days_ago.format("%Y%m%d").to_string()
}