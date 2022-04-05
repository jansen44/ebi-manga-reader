use ebi_sources::opex::OpexSource;
use ebi_sources::source::Source;
use ebi_sources::yabu::YabuSource;
use ebi_sources::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut sources: Vec<Box<dyn Source>> = vec![];
    sources.push(Box::new(OpexSource::default()));
    // sources.push(Box::new(YabuSource::default()));
    println!("{:?}", sources);

    for s in sources {
        let manga = s.manga_list().await?;
        for m in manga {
            println!("{}", m);
        }
    }

    Ok(())
}
