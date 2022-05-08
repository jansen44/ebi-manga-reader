use ebi_sources::get_available_sources;
use ebi_archive::errors::Result;

mod args;

fn handle_sources() {
    println!("== Available Sources ==");
    for (identifier, source) in get_available_sources().iter() {
        println!("{identifier} -- {} -- {} -- {}/", source.title(), source.description(), source.base_url());
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let arg_matches = args::get_args();

    match arg_matches.subcommand() {
        Some(("sources", _)) => handle_sources(),
        _ => ()
    }

    Ok(())
}
