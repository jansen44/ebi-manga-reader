use ebi_archive::downloader::download_chapter;
use ebi_archive::errors::{ArchiveError, Result};
use ebi_sources::errors::SourceError;
use ebi_sources::get_available_sources;

mod args;

fn handle_sources() {
    println!("== Available Sources ==");
    for (identifier, source) in get_available_sources().iter() {
        println!(
            "{identifier} -- {} -- {} -- {}/",
            source.title(),
            source.description(),
            source.base_url()
        );
    }
}

async fn handle_down(arg_matches: &clap::ArgMatches) -> Result<()> {
    let source = arg_matches.value_of("source").unwrap();
    let manga_identifier = arg_matches.value_of("identifier").unwrap();
    let chapter: usize = arg_matches.value_of("chapter").unwrap().parse().unwrap();

    let sources = get_available_sources();
    let source = match sources.get(source) {
        Some(source) => source,
        None => return Err(ArchiveError::from(SourceError::InvalidSourceIdentifier)),
    };

    download_chapter(
        source.identifier().as_str(),
        manga_identifier,
        chapter,
        None,
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let arg_matches = args::get_args();

    match arg_matches.subcommand() {
        Some(("sources", _)) => handle_sources(),
        Some(("down", subargs)) => handle_down(subargs).await?,
        _ => (), // unreachable due to clap validations
    }

    Ok(())
}
