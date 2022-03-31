use ebi_sources::opex::Opex;
use ebi_sources::SourceErrors;
use ebi_archive::download_chapter;

#[tokio::main]
async fn main() -> Result<(), SourceErrors> {
    let opex = Opex::new()?;

    download_chapter(opex,"main",7).await?;

    // // let mangas = opex.mangas().await?;
    // let manga = opex.manga("main").await?.unwrap();
    // // let chapters = opex.chapters(&manga).await?;
    // let chapter = opex.chapter(&manga, 7).await?.unwrap();
    // let pages = opex.pages(&chapter).await?;
    //
    // println!("Source: {:?}", Opex::source());
    //
    // // for m in mangas {
    // //     println!("{:?}", m);
    // // }
    // println!("Manga: {:?}", manga);
    //
    // for chapter in chapters {
    //
    // }
    //
    // println!("Download Chapter 7: {:?}", chapter);
    //
    // for page in pages {
    //     println!("{:?}", page);
    // }

    Ok(())
}