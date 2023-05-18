use crate::model::faction::FactionResult;

pub(crate) struct ApiClient {
    url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub(crate) fn new<S>(server_url: S) -> Self
    where
        S: ToString,
    {
        let client = reqwest::Client::new();

        Self {
            url: server_url.to_string(),
            client,
        }
    }

    pub(crate) async fn get_factions(&self) -> anyhow::Result<FactionResult> {
        Ok(self
            .client
            .get(format!("{}/v2/factions", self.url))
            .send()
            .await?
            .json::<FactionResult>()
            .await?)
    }
}
