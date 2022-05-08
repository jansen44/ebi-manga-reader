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

    io::copy(&mut content, &mut out).expect("failed to copy content"); // TODO: better error handling
    Ok(())
}

pub async fn download_chapter(
    source: &str,
    manga_identifier: &str,
    chapter_number: usize,
    destination: Option<String>,
) -> Result<()> {
    print!("Validating \"{source}\" source...");
    let sources = get_available_sources();
    let source = sources.get(source).unwrap(); // TODO: better error handling
    println!(" Done!");

    print!("Loading necessary manga info...");
    let manga = source.get_manga(manga_identifier).await?.unwrap();
    println!(" Done!");

    print!("Loading necessary chapter info...");
    let chapter = manga.chapter(chapter_number.to_owned()).await?.unwrap();
    let pages = chapter.page_url_list().await?;
    println!(" Done!");

    print!("Creating necessary directories...");
    let target_dir = create_directories(
        &source.identifier(),
        manga_identifier,
        chapter_number,
        destination,
    )?;
    println!(" Done!");

    let mut tasks: Vec<JoinHandle<Result<(), DownloadError>>> = vec![];

    println!(
        "== Starting download of \"{}\" chapter {} - \"{}\"",
        manga.title(),
        chapter_number,
        chapter.title()
    );
    for page in pages {
        // TODO: more consistent filename generation (maybe just index?)
        let url_parts = page.split("/").collect::<Vec<&str>>();
        let file_name = (*url_parts.last().unwrap()).to_owned();

        let destination = format!("{target_dir}/{page}", page = file_name.clone());

        tasks.push(tokio::spawn(async move {
            let downloaded = download_page(page.as_str(), &destination).await;
            match downloaded {
                Err(err) => Err(DownloadError::GenericError(format!(
                    "ERROR downloading {}: {}",
                    page, err
                ))),
                _ => {
                    println!("Downloaded: {file_name}");
                    Ok(())
                }
            }
        }));
    }

    join_all(tasks).await;
    println!("== Everything done! ==");
    println!("You can find it here: {target_dir}");
    println!("Have a good reading!");

    Ok(())
}
