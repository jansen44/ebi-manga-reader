use std::env;

use ebi_archive::downloader::download_chapter;
use ebi_archive::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let chapter = &args[1].parse::<usize>().unwrap();
        download_chapter("opex", "main", chapter.to_owned(), None).await?;

        Ok(())
    } else {
        panic!("chapter argument required")
    }
}
