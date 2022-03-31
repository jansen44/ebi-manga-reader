use ebi_sources::opex::Opex;
use ebi_sources::SourceErrors;
use std::fs;

// TODO: Refactor when a Source trait exists
// TODO: Locked to Opex
pub async fn download_chapter(opex: Opex, manga_identifier: &str, chapter_number: usize) -> Result<(), SourceErrors> {
    fs::create_dir_all(format!("{source_name}/{identifier}/{chapter}",
                               source_name = "opex",
                               identifier = manga_identifier,
                               chapter = chapter_number));


    let manga = opex.manga(manga_identifier).await?.unwrap();
    let chapter = opex.chapter(&manga, chapter_number).await?.unwrap();
    let pages = opex.pages(&chapter).await?;

    println!("Download Chapter: {:?}", chapter);

    for page in pages {
        println!("{:?}", format!("{}/{}", Opex::source().base_url, page));
    }

    Ok(())
}