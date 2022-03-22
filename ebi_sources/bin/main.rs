use ebi_sources::opex::Opex;
use ebi_sources::SourceErrors;

#[tokio::main]
async fn main() -> Result<(), SourceErrors> {
    let opex = Opex::new()?;

    println!("{:?}", Opex::source());
    println!("{:?}", opex.mangas().await);

    Ok(())
}
