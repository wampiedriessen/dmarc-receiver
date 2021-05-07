mod webhook;
mod datadog;
mod elasticsearch;

use super::DmarcReport;

pub fn create_exporter(exporter_name: String) -> Result<Box<dyn super::Exporter>, String> {
    Ok(match exporter_name.as_str() {
        ElasticSearchExporter::EXPORTER_NAME => Box::new(ElasticSearchExporter::new()?),
        // WebhookExporter::EXPORTER_NAME => Box::new(WebhookExporter::new()?),
        // DatadogExporter::EXPORTER_NAME => Box::new(DatadogExporter::new()?),
        _ => return Err(String::from("Unkown Exporter"))
    })
}

#[async_trait::async_trait]
pub trait Exporter {
    fn name(&self) -> &'static str;
    async fn export(&self, reports: &Vec<DmarcReport>) -> Result<(), String>;
}

struct ElasticSearchExporter {
    client: ::elasticsearch::Elasticsearch,
    index: String,
}

// struct WebhookExporter {
//     webhook_url: String,
//     webclient: reqwest::Client,
// }

// struct DatadogExporter {
//     datadog_site: String,
//     api_key: String,
//     webclient: reqwest::Client,
// }
