use serde_json::json;
use elasticsearch::{
    Elasticsearch, Error, BulkParts,
    http::{request::JsonBody, transport::Transport}
};

use std::env;
use super::ElasticSearchExporter;
use super::Exporter;
use super::DmarcReport;

impl ElasticSearchExporter {
    pub const EXPORTER_NAME: &'static str = "elasticsearch";

    const ENVVAR_ENDPOINT: &'static str = "DMARC_RECEIVER_ELK_ENDPOINT";
    const ENVVAR_INDEX: &'static str = "DMARC_RECEIVER_ELK_INDEX";

    pub fn new() -> Result<ElasticSearchExporter, String> {

        let endpoint = env::var(ElasticSearchExporter::ENVVAR_ENDPOINT)
            .map_err(|_| format!("Missing ElasticSearch endpoint {}", ElasticSearchExporter::ENVVAR_ENDPOINT))?;
        let index = env::var(ElasticSearchExporter::ENVVAR_INDEX)
            .map_err(|_| format!("Missing ElasticSearch index {}", ElasticSearchExporter::ENVVAR_INDEX))?;

        let client = create_elk_client(endpoint.as_str())
            .map_err(|_| "Could not configure ElasticSearch client")?;

        Ok(ElasticSearchExporter {
            client,
            index
        })
    }
}

#[async_trait::async_trait]
impl Exporter for ElasticSearchExporter {
    fn name(&self) -> &'static str { return ElasticSearchExporter::EXPORTER_NAME; }

    async fn export(&self, reports: &Vec<DmarcReport>) -> Result<(), String> {

        let mut body: Vec<JsonBody<_>> = Vec::with_capacity(reports.len()*2);

        for report in reports {
            body.push(json!({"index": {}}).into());
            body.push(json!(report).into());
        }

        let response = self.client
            .bulk(BulkParts::Index(self.index.as_str()))
            .body(body)
            .send()
            .await
            .map_err(|e| format!("Encountered ELK error:\n{:?}", e))?;
    
        println!("{:?}", response);

        match response.status_code().is_success() {
            true => Ok(()),
            false => Err(String::from("Could not export to ElasticSearch"))
        }
    }
}

fn create_elk_client(endpoint: &str) -> Result<Elasticsearch, Error> {
    let transport = Transport::single_node(endpoint)?;
    Ok(Elasticsearch::new(transport))
}
