use crate::errors::download::DownloadError;
use crate::Result;
use ebi_sources::source::Source;
use futures::future::join_all;
use reqwest;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
use tokio::task::JoinHandle;

const DEFAULT_DIR: &str = "ebi/manga";

pub async fn download_chapter(
    source: Box<dyn Source>,
    manga_identifier: &str,
    chapter_number: usize,
) -> Result<()> {
    let base_path = create_directories(&source.identifier(), manga_identifier, chapter_number)?;

    let manga = source.get_manga(manga_identifier).await?.unwrap();
    let chapter = manga.chapter(chapter_number.to_owned()).await?.unwrap();
    let pages = chapter.page_url_list().await?;

    println!("Download Chapter: {:?}", chapter);

    let mut tasks: Vec<JoinHandle<Result<(), DownloadError>>> = vec![];

    for page in pages {
        let url_parts = page.split("/").collect::<Vec<&str>>();
        let file_name = url_parts.last().unwrap();
        println!("{:?}", format!("{}::::{}", page, file_name));
        let destination = format!(
            "{base_path}/{page}",
            page = file_name
        );
        tasks.push(tokio::spawn(async move {
            let downloaded = download_page(page.as_str(), &destination).await;
            match downloaded {
                Err(err) => Err(DownloadError::GenericError(format!(
                    "ERROR downloading {}: {}",
                    page, err
                ))),
                _ => Ok(()),
            }
        }));
    }

    join_all(tasks).await;

    Ok(())
}

fn create_directories(
    source_name: &str,
    manga_identifier: &str,
    chapter_number: usize,
) -> std::io::Result<String> {
    let home = std::env::var("HOME").unwrap(); // TODO: handle this error and "Windows" later
    let base_path = format!(
        "{home}/{DEFAULT_DIR}/{source_name}/{identifier}/{chapter}",
        source_name = source_name,
        identifier = manga_identifier,
        chapter = chapter_number
    );
    fs::create_dir_all(&base_path)?;
    Ok(base_path)
}

async fn download_page(page: &str, destination: &str) -> Result<(), DownloadError> {
    let resp = reqwest::get(page.to_owned()).await?;

    let mut out = File::create(destination.to_owned()).expect("failed to create file");
    let mut content = Cursor::new(resp.bytes().await?);
    io::copy(&mut content, &mut out).expect("failed to copy content");
    Ok(())
}
