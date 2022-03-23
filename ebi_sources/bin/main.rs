use ebi_sources::opex::Opex;
use ebi_sources::SourceErrors;

#[tokio::main]
async fn main() -> Result<(), SourceErrors> {
    let opex = Opex::new()?;
    let main = opex.mangas().await?;
    let main = &main[0];

    println!("{:?}", Opex::source());
    println!("{:?}", opex.mangas().await?);
    
    let chapters = opex.chapters(main).await?;
    for chapter in chapters {
        println!("{:?}", chapter);
    }

    Ok(())
}
