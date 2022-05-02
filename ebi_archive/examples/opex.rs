use ebi_archive::downloader::download_chapter;
use ebi_archive::Result;
use ebi_sources::opex::OpexSource;

#[tokio::main]
async fn main() -> Result<()> {
    let opex = Box::new(OpexSource::default());

    download_chapter(opex, "main", 7).await?;


    Ok(())
}
