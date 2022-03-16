use std::error::Error;

use reqwest::Response;

const CLIENT: &'static str = "http://localhost:8008/_matrix/client";

pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn register(&self, client: &reqwest::Client) -> Result<Response, Box<dyn Error>> {
        let body = format!(
            "{{
            \"username\": \"{}\",
            \"password\": \"{}\",
            \"auth\": {{
                \"type\": \"m.login.dummy\"
            }}
        }}",
            self.username, self.password
        );
        Ok(client
            .post(CLIENT.to_owned() + "/r0/register")
            .body(body)
            .send()
            .await?)
    }
    pub async fn login(&self, client: &reqwest::Client) -> Result<Response, Box<dyn Error>> {
        let body = format!(
            "{{
            \"username\": \"{}\",
            \"password\": \"{}\",
            \"type\": \"m.login.password\"
        }}",
            self.username, self.password
        );
        Ok(client
            .post(CLIENT.to_owned() + "/r0/login")
            .body(body)
            .send()
            .await?)
    }
}

pub async fn get_version() -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(CLIENT.to_owned() + "/versions")
        .await?
        .text()
        .await?)
}
