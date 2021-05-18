mod exporters;
mod parser;

use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use exporters::Exporter;
use serde::Serialize;
use std::env;

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

// public struct format supplied to exporters
#[derive(Serialize, Debug)]
pub struct DmarcReport {
    pub org_name: String,
    pub date_range_begin: DateTime<Utc>,
    pub date_range_end: DateTime<Utc>,
    pub policy_domain: String,
    pub policy_adkim: Option<parser::dmarc_definition::AlignmentType>,
    pub policy_aspf: Option<parser::dmarc_definition::AlignmentType>,
    pub policy_p: parser::dmarc_definition::DispositionType,
    pub policy_sp: Option<parser::dmarc_definition::DispositionType>,
    pub policy_pct: u8,
    pub source_ip: std::net::IpAddr,
    pub count: u32,
    pub evaluated_disposition: parser::dmarc_definition::DispositionType,
    pub evaluated_dkim: Option<parser::dmarc_definition::DMARCResultType>,
    pub evaluated_spf: Option<parser::dmarc_definition::DMARCResultType>,
    pub header_from: String,
    pub auth_results: parser::dmarc_definition::AuthResultType
}

// mapping from formal format to export format
fn map_to_export_format(dmarc: &Vec<parser::dmarc_definition::feedback>) -> Vec<DmarcReport> {
    let mut reports = Vec::with_capacity(dmarc.len());

    for feedback in dmarc {
        let begin = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(feedback.report_metadata.date_range.begin as i64, 0), Utc);
        let end = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(feedback.report_metadata.date_range.end as i64, 0), Utc);
        for record in &feedback.record {

            reports.push(DmarcReport {
                org_name: feedback.report_metadata.org_name.clone(),
                date_range_begin: begin.clone(),
                date_range_end: end.clone(),

                policy_domain: feedback.policy_published.domain.clone(),
                policy_adkim: feedback.policy_published.adkim.clone(),
                policy_aspf: feedback.policy_published.aspf.clone(),
                policy_p: feedback.policy_published.p.clone(),
                policy_sp: feedback.policy_published.sp.clone(),
                policy_pct: feedback.policy_published.pct,

                source_ip: record.row.source_ip,
                count: record.row.count,
                evaluated_disposition: record.row.policy_evaluated.disposition.clone(),
                evaluated_dkim: record.row.policy_evaluated.dkim.clone(),
                evaluated_spf: record.row.policy_evaluated.spf.clone(),

                header_from: record.identifiers.header_from.clone(),

                auth_results: record.auth_results.clone()
            })
        }
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
