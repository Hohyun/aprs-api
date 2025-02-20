use clap::Parser;

pub mod payment;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Settle date (YYYYMMDD)
    #[clap(short, long)]
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
        _ => {
            println!("Not supported yet: {}", args.paycode);
        }
    }

    Ok(())
}