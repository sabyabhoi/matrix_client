use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let cognusboi = matrix_client::User {
        username: String::from("cognusboi"),
        password: String::from("foo"),
    };

    cognusboi.register(&client).await?;

    Ok(())
}
