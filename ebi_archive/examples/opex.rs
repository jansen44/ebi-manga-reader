use std::env;

use ebi_archive::downloader::download_chapter;
use ebi_archive::Result;
use ebi_sources::opex::OpexSource;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() > 1 {
        let chapter = &args[1].parse::<usize>().unwrap();

        let opex = Box::new(OpexSource::default());

        download_chapter(opex, "main", chapter.to_owned()).await?;
        Ok(())
    } else {
        panic!("chapter argument required")
    }
}
