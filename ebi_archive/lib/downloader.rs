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

pub async fn download_chapter(
    source: Box<dyn Source>,
    manga_identifier: &str,
    chapter_number: usize,
) -> Result<()> {
    let _ = fs::create_dir_all(format!(
        "{source_name}/{identifier}/{chapter}",
        source_name = source.identifier(),
        identifier = manga_identifier,
        chapter = chapter_number
    ))?;

    let manga = source.get_manga(manga_identifier).await?.unwrap();
    let chapter = manga.chapter(chapter_number).await?.unwrap();
    let pages = chapter.page_url_list().await?;

    println!("Download Chapter: {:?}", chapter);

    let mut tasks: Vec<JoinHandle<Result<(), DownloadError>>> = vec![];

    for page in pages {
        let url_parts = page.split("/").collect::<Vec<&str>>();
        let file_name = url_parts.last().unwrap();
        println!("{:?}", format!("{}::::{}", page, file_name));
        let destination = format!(
            "{source_name}/{identifier}/{chapter}/{page}",
            source_name = source.identifier(),
            identifier = manga_identifier,
            chapter = chapter_number,
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

async fn download_page(page: &str, destination: &str) -> Result<(), DownloadError> {
    let resp = reqwest::get(page.to_owned()).await?;

    let mut out = File::create(destination.to_owned()).expect("failed to create file");
    let mut content = Cursor::new(resp.bytes().await?);
    io::copy(&mut content, &mut out).expect("failed to copy content");
    Ok(())
}
