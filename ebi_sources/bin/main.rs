use ebi_sources::client::ClientErrors;
use ebi_sources::opex::client::OpexClient;

#[tokio::main]
async fn main() -> Result<(), ClientErrors> {
    let client = OpexClient::new()?;

    match client.get_manga_page().await {
        Err(err) => println!("Something went Wrong! {err}"),
        Ok(body) => println!("{body}"),
    };

    Ok(())
}
