use ebi_sources::opex::OpexSource;
use ebi_sources::source::Source;
use ebi_sources::yabu::YabuSource;
use ebi_sources::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut sources: Vec<Box<dyn Source>> = vec![];
    sources.push(Box::new(OpexSource::default()));
    // sources.push(Box::new(YabuSource::default()));
    
    println!("=== Sources ===\n");
    println!("{:?}", sources);

    println!("\n\n=== Manga List ===\n");
    for s in sources.iter() {
        let manga = s.manga_list().await?;
        for m in manga {
            println!("{}", m);
        }
    }
    
    println!("\n\n=== Latest Manga List ===\n");
    for s in sources.iter() {
        let manga = s.latest_manga().await?;
        for m in manga {
            println!("{}", m);
        }
    }

    println!("\n\n=== Popular Manga List ===\n");
    for s in sources.iter() {
        let manga = s.popular_manga().await?;
        for m in manga {
            println!("{}", m);
        }
    }

    println!("\n\n=== Hot Manga List ===\n");
    for s in sources.iter() {
        let manga = s.hot_manga().await?;
        for m in manga {
            println!("{}", m);
        }
    }

    println!("\n\n=== Search Manga ===\n");
    let manga = sources[0].search_manga("").await?;
    for m in manga {
        println!("{}", m);
    }

    println!("\n\n=== Get Manga ===\n");
    let manga = sources[0].get_manga("main").await?.unwrap();
    println!("{}", manga);

    Ok(())
}
