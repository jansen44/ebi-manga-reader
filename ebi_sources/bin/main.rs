use ebi_sources::opex::Opex;
use ebi_sources::SourceErrors;

#[tokio::main]
async fn main() -> Result<(), SourceErrors> {
    let opex = Opex::new()?;
    let mangas = opex.mangas().await?;
    let manga = opex.manga(3).await?.unwrap();
    let chapters = opex.chapters(&manga).await?;
    let chapter = opex.chapter(&manga, 7).await?.unwrap();

    println!("Source: {:?}", Opex::source());

    for m in mangas {
        println!("{:?}", m);
    }
    println!("Manga: {:?}", manga);

    for chapter in chapters {
        println!("{:?}", chapter);
    }

    println!("Chapter 7: {:?}", chapter);

    Ok(())
}
