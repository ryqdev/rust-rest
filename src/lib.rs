use reqwest::{Method, Response};

mod rest;


struct Rest {
    endpoint: String,
}

impl Rest {
    fn new(
        endpoint: &str
    ) -> anyhow::Result<Self> {
        Ok(Self{
            endpoint: endpoint.to_string(),
        })
    }

    fn get(
        &self,
        path: &str,
        body: &[(String, String)]
    ) -> anyhow::Result<Response> {
        self.send(Method::GET, path, body)
    }

    fn post(
        &self,
        path: &str,
        body: &[(String, String)],
    ) -> anyhow::Result<Response> {
        self.send(Method::POST, path, body)
    }

    fn send(
        &self,
        method: Method,
        path: &str,
        body: &[(String, String)]

    ) -> anyhow::Result<Response> {
        let builder = reqwest::Client::new()
            .request(
                method,
                format!("{}/{}", self.endpoint, path),
            )
            .header("header key", "header value")
            .query(body);
        let response = builder.send();
        Ok(response)
    }
}