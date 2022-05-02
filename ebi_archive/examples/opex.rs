use ebi_archive::downloader::download_chapter;
use ebi_archive::Result;
use ebi_sources::opex::OpexSource;

#[tokio::main]
async fn main() -> Result<()> {
    let opex = Box::new(OpexSource::default());

    download_chapter(opex, "main", 7).await?;

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
