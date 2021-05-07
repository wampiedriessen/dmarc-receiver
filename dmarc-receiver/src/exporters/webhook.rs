// use reqwest;
// use std::env;
// use super::WebhookExporter;
// use super::Exporter;
// use super::DmarcReport;

// impl WebhookExporter {
//     pub const EXPORTER_NAME: &'static str = "webhook";

//     pub fn new() -> Result<WebhookExporter, &'static str> {
//         let webhook_url = match env::var("DMARC_RECEIVER_WEBHOOK_URL") {
//             Ok(x) => x,
//             Err(_) => return Err("Missing Webhook Url Envvar"),
//         };

//         Ok(WebhookExporter {
//             webhook_url: webhook_url.to_string(),
//             webclient: reqwest::Client::new(),
//         })
//     }
// }

// #[async_trait::async_trait]
// impl Exporter for WebhookExporter {
//     fn name(&self) -> &'static str { return WebhookExporter::EXPORTER_NAME; }

//     async fn export(&self, report: DmarcReport) -> Result<(), &'static str> { 
//         self.webclient.post(&self.webhook_url)
//             .body(format!("Domain: {},\n# Emails: {}", report.domain, report.num_mails))
//             .send()
//             .await
//              .map_err(|_| "Error contacting webhook URL")?;

//         Ok(())
//     }
// }

// // #[cfg(test)]
// // mod test {

// //     #[test]
// //     fn posts_with_reqwest() {
// //         super::my_implementation();
// //     }
// // }