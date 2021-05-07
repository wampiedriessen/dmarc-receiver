// use reqwest;
// use std::env;
// use super::DatadogExporter;
// use super::Exporter;
// use super::DmarcReport;

// impl DatadogExporter {
//     pub const EXPORTER_NAME: &'static str = "datadog";

//     pub fn new() -> Result<DatadogExporter, &'static str> {
//         let datadog_site = env::var("DMARC_RECEIVER_DATADOG_SITE").unwrap_or(String::from("datadoghq.com"));
//         let api_key = env::var("DMARC_RECEIVER_DATADOG_API_KEY").map_err(|_| "No Datadog API key provided")?;

//         Ok(DatadogExporter {
//             datadog_site,
//             api_key,
//             webclient: reqwest::Client::new(),
//         })
//     }
// }

// #[async_trait::async_trait]
// impl Exporter for DatadogExporter {
//     fn name(&self) -> &'static str { return DatadogExporter::EXPORTER_NAME; }

//     async fn export(&self, report: DmarcReport) -> Result<(), &'static str> {
//         Ok(())
//     }
// }