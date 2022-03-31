use ebi_sources::{errors::SourceErrors, yabu::Yabu};

#[tokio::main]
async fn main() -> Result<(), SourceErrors> {
    let yabu = Yabu::new()?;
    let manga_list = yabu.mangas().await?;

    println!("{:?}", manga_list);

    Ok(())
}