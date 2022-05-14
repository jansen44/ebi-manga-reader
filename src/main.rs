use anyhow::Result;

mod archive;
mod args;
mod errors;
mod sources;

use archive::downloader;

fn handle_sources() {
    println!("== Available Sources ==");
    for (identifier, source) in sources::get_available_sources().iter() {
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
    let target_dir = match arg_matches.value_of("target_dir") {
        Some(dir) => Some(dir.to_owned()),
        None => None,
    };

    let sources = sources::get_available_sources();
    let source = match sources.get(source) {
        Some(source) => source,
        None => return Err(errors::EbiError::InvalidSourceIdentifier.into()),
    };

    if arg_matches.is_present("all") {
        return downloader::download_all_chapters(source.identifier().as_str(), manga_identifier, target_dir)
            .await;
    }

    let chapter: usize = arg_matches.value_of("chapter").unwrap().parse().unwrap();

    downloader::download_chapter(
        source.identifier().as_str(),
        manga_identifier,
        chapter,
        target_dir,
    )
    .await
}

#[tokio::main]
async fn main() -> Result<()> {
    let arg_matches = args::get_args();

    match arg_matches.subcommand() {
        Some(("sources", _)) => handle_sources(),
        Some(("down", subargs)) => {
            let result = handle_down(subargs).await;
            if let Err(err) = result {
                return Err(err);
            }
        }
        _ => (), // unreachable due to clap validations
    }

    Ok(())
}
