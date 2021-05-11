mod exporters;
use exporters::Exporter;
mod parser;

use std::env;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let config = Config::new();

    match config {
        Ok(config) => {
            if let Err(e) = run(config).await {
                println!("Unexpected failure during execution");
                println!("{:?}", e);
                std::process::exit(1);
            }
        }
        Err(s) => 
        {
            println!("Application could not boot:\n{}", s);
            std::process::exit(1);
        }
    }
}

async fn run(config: Config) -> Result<(), String> {
    println!("Running exporter '{}'", config.exporter.name());
    println!("==========");

    let full_reports: Vec<parser::dmarc_definition::feedback>;
    let stdin = std::io::stdin();
    {
        let mut stdin = stdin.lock();

        full_reports = parser::parse(&mut stdin)?;
    }

    let export_reports = map_to_export_format(&full_reports);

    config.exporter.export(&export_reports).await?;

    Ok(())
}

#[derive(Serialize, Debug)]
pub struct DmarcReport {
    pub domain: String,
    pub num_mails: u32,
}

fn map_to_export_format(dmarc: &Vec<parser::dmarc_definition::feedback>) -> Vec<DmarcReport> {
    let mut reports = Vec::with_capacity(dmarc.len());

    for feedback in dmarc {
        reports.push(DmarcReport {
            domain: feedback.policy_published.domain.clone(),
            num_mails: feedback.record.iter().map(|x| x.row.count).sum()
        })
    }

    reports
}

struct Config {
    pub exporter: Box<dyn Exporter>,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let exporter_name = env::var("DMARC_RECEIVER_EXPORTER").map_err(|_| "Missing Exporter Name Envvar")?;

        let exporter = exporters::create_exporter(exporter_name)?;

        Ok(Config {
            exporter: exporter
        })
    }
}
