use crate::prelude::*;
use azure_core::{headers::Headers, Method, Url};
use serde_json::{Map, Value};

operation! {
    GetRandomBytes,
    client: KeyClient,
    hsm_name: String,
    count: u8,
}

impl GetRandomBytesBuilder {
    pub fn into_future(self) -> GetRandomBytes {
        Box::pin(async move {
            // POST {HSMBaseUrl}//rng?api-version=7.4
            let vault_url = format!("https://{}.managedhsm.azure.net/", self.hsm_name);
            let mut uri = Url::parse(&vault_url)?;
            let path = "rng".to_string();

            uri.set_path(&path);

            let mut request_body = Map::new();
            request_body.insert("count".to_owned(), Value::from(self.count));

            let headers = Headers::new();
            let mut request = KeyvaultClient::finalize_request(
                uri,
                Method::Post,
                headers,
                Some(Value::Object(request_body).to_string().into()),
            );

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}

type GetRandomBytesResponse = GetRandomBytesResult;
