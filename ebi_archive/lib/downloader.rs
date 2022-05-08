use crate::errors::download::DownloadError;
use crate::Result;
use ebi_sources::get_available_sources;
use futures::future::join_all;
use reqwest;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
use tokio::task::JoinHandle;

fn default_target_base_path() -> String {
    let home = std::env::var("HOME").unwrap(); // TODO: handle this error and "Windows" later
    format!("{home}/ebi/manga")
}

fn create_directories(
    source_name: &str,
    manga_identifier: &str,
    chapter_number: usize,
    destination: Option<String>,
) -> std::io::Result<String> {
    // TODO: handle already existing directories better

    let destination = match destination {
        Some(destination) => destination,
        None => default_target_base_path(),
    };
    let destination = format!("{destination}/{source_name}/{manga_identifier}/{chapter_number}");

    fs::create_dir_all(&destination)?;
    Ok(destination)
}

async fn download_page(page_url: &str, destination: &str) -> Result<(), DownloadError> {
    let page_url = page_url.to_owned();
    let destination = destination.to_owned();

    let resp = reqwest::get(page_url).await?;
    let mut out = File::create(destination).expect("failed to create file"); // TODO: better error handling
    let mut content = Cursor::new(resp.bytes().await?);

    // TODO: Handle "Too many open files"
    io::copy(&mut content, &mut out).expect("failed to copy content"); // TODO: better error handling
    Ok(())
}

fn download_page_job(page: String, destination: String) -> JoinHandle<Result<(), DownloadError>> {
    tokio::spawn(async move {
        let downloaded = download_page(page.as_str(), destination.as_str()).await;
        match downloaded {
            Err(err) => Err(DownloadError::GenericError(format!(
                "ERROR downloading {}: {}",
                page, err
            ))),
            _ => Ok(()),
        }
    })
}

async fn download_single_chapter<'a>(
    source_identifier: &str,
    manga_identifier: &str,
    chapter: &'a Box<dyn ebi_sources::chapter::Chapter>,
    destination: Option<String>,
) -> Result<String> {
    let pages = chapter.page_url_list().await?;

    let target_dir = create_directories(
        source_identifier,
        manga_identifier,
        chapter.chapter(),
        destination,
    )?;

    let mut tasks: Vec<JoinHandle<Result<(), DownloadError>>> = vec![];

    for (index, page) in pages.iter().enumerate() {
        let file_extension = *page
            .split("/")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>()
            .last()
            .unwrap();

        let destination = format!("{target_dir}/{index}.{file_extension}");

        tasks.push(download_page_job(page.clone(), destination.clone()));
    }

    join_all(tasks).await;

    Ok(target_dir)
}

const CHAPTER_BATCH_SIZE: usize = 3;

pub async fn download_all_chapters(
    source: &str,
    manga_identifier: &str,
    destination: Option<String>,
) -> Result<()> {
    let sources = get_available_sources();
    let source = sources.get(source).unwrap(); // TODO: better error handling

    let manga = source.get_manga(manga_identifier).await?.unwrap();
    let mut chapters = manga.chapter_list().await?;

    while chapters.len() > 0 {
        let mut tasks: Vec<JoinHandle<Result<String, crate::errors::ArchiveError>>> =
            Vec::with_capacity(CHAPTER_BATCH_SIZE); // maybe thread better??
        let mut downloaded: String = String::new();

        for i in 0..CHAPTER_BATCH_SIZE {
            if i >= chapters.len() {
                break;
            }

            let chapter = chapters.swap_remove(i);
            downloaded = format!("{},{}", chapter.chapter(), downloaded);

            let destination = match destination {
                Some(ref s) => Some(s.clone()),
                None => None,
            };
            let source_identifier = source.identifier();
            let manga_identifier = manga.identifier();

            tasks.push(tokio::spawn(async move {
                download_single_chapter(
                    source_identifier.as_str(),
                    manga_identifier.as_str(),
                    &chapter,
                    destination,
                )
                .await
            }));
        }
        let results = join_all(tasks).await;
        for result in results {
            match result {
                Ok(result) => match result {
                    Ok(result) => println!("Chapter downloaded to \"{result}\""),
                    Err(err) => println!("{}", err),
                },
                Err(err) => println!("error: Join error: {}", err),
            }
        }
        println!("Batch with chapters [{downloaded}] done.");
    }

    Ok(())
}

pub async fn download_chapter(
    source: &str,
    manga_identifier: &str,
    chapter_number: usize,
    destination: Option<String>,
) -> Result<()> {
    let sources = get_available_sources();
    let source = sources.get(source).unwrap(); // TODO: better error handling

    let manga = source.get_manga(manga_identifier).await?.unwrap();

    let chapter = manga.chapter(chapter_number).await?.unwrap();

    let destination = download_single_chapter(
        source.identifier().as_str(),
        manga.identifier().as_str(),
        &chapter,
        destination,
    )
    .await?;
    
    println!("Done. Downloaded to \"{destination}\", have a good read!");
    Ok(())
}
