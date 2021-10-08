use datafusion::arrow::record_batch::RecordBatch;
use datafusion::arrow::util::pretty::print_batches;
use datafusion::prelude::*;
use std::error::Error;
use std::fs::read_dir;
use structopt::StructOpt;

mod models;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Select available action from
    /// fetch: fetch FPL API as a .csv file;
    /// query: query csv file(s);
    /// check: check available file(s) on /data directory
    #[structopt(short)]
    mode: String,

    /// SQL query string
    #[structopt(required_if("mode", "query"))]
    query: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    if args.mode == "fetch" {
        let resp: models::General = match fetch().await {
            Ok(r) => r,
            Err(e) => panic!("error: {:?}", e),
        };
        match writer(resp, "elements") {
            Ok(()) => println!("Succesfully fetched API request"),
            Err(e) => panic!("error: {:?}", e),
        }
    } else if args.mode == "query" {
        let mut ctx = ExecutionContext::new();
        ctx.register_csv("elements", "./data/elements.csv", CsvReadOptions::new())?;
        let query = args.query.as_deref();
        let df = ctx.sql(query.unwrap())?;
        let results: Vec<RecordBatch> = df.collect().await?;
        print_batches(&results)?;
    } else if args.mode == "check" {
        checker();
    }
    Ok(())
}

fn checker() {
    let paths = read_dir("./data/").unwrap();
    for path in paths {
        println!("{:?}", path.unwrap().file_name());
    }
}

fn writer(resp: models::General, filename: &str) -> Result<(), Box<dyn Error>> {
    let full_path = format!("./data/{}.csv", &filename);
    let mut writer = csv::Writer::from_path(full_path)?;
    let element: Vec<models::Element> = resp.elements;
    for row in element {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

async fn fetch() -> Result<models::General, reqwest::Error> {
    let request_url = "https://fantasy.premierleague.com/api/bootstrap-static/".to_string();
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let response = client.get(request_url).send().await?;
    println!("status: {}", response.status());
    let payload = response.json::<models::General>().await?;
    Ok(payload)
}
