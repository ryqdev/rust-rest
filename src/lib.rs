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

    async fn get(
        &self,
        path: &str,
        body: &[(String, String)]
    ) -> anyhow::Result<Response> {
        self.send(Method::GET, path, body).await
    }

    async fn post(
        &self,
        path: &str,
        body: &[(String, String)],
    ) -> anyhow::Result<Response> {
        self.send(Method::POST, path, body).await
    }

    async fn send(
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
        let response = builder.send().await?;
        Ok(response)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use tokio;

    #[tokio::test]
    async fn test_real_get() {
        let rest_client = Rest::new("127.0.0.1:8080").unwrap();
        let body = vec![];

        let response = rest_client.get("test-get", &body).await;

        match response {
            Ok(resp) => {
                assert_eq!(resp.status(), StatusCode::OK);
                println!("{}", resp.text().await.unwrap());
            },
            Err(e) => {
                panic!("GET request failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_real_post() {
        let rest_client = Rest::new("http://www.foo.bar").unwrap();
        let body = vec![("param1".to_string(), "value1".to_string())];

        // Assuming "www.foo.bar/test-post" is a valid POST endpoint
        let response = rest_client.post("test-post", &body).await;

        match response {
            Ok(resp) => {
                assert_eq!(resp.status(), StatusCode::CREATED);
                // Uncomment below to inspect response body (may vary based on server behavior)
                // println!("{}", resp.text().await.unwrap());
            },
            Err(e) => {
                panic!("POST request failed: {:?}", e);
            }
        }
    }
}