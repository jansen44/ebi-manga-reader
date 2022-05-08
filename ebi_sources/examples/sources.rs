use ebi_sources::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let sources = ebi_sources::get_available_sources();
    
    println!("=== Sources ===\n");
    println!("{:?}", sources);

    println!("\n\n=== Manga List ===\n");
    for (_, s) in sources.iter() {
        let manga = s.manga_list().await?;
        for m in manga {
            println!("{}", m);
        }
    }
    
    println!("\n\n=== Latest Manga List ===\n");
    for (_, s) in sources.iter() {
        let manga = s.latest_manga().await?;
        for m in manga {
            println!("{}", m);
        }
    }

    println!("\n\n=== Popular Manga List ===\n");
    for (_, s) in sources.iter() {
        let manga = s.popular_manga().await?;
        for m in manga {
            println!("{}", m);
        }
    }

    println!("\n\n=== Hot Manga List ===\n");
    for (_, s) in sources.iter() {
        let manga = s.hot_manga().await?;
        for m in manga {
            println!("{}", m);
        }
    }

    println!("\n\n=== Search Manga ===\n");
    let manga = sources.get("yabu").unwrap().search_manga("vin").await?;
    for m in manga {
        println!("{}", m);
    }

    println!("\n\n=== Get Manga ===\n");
    let manga = sources.get("yabu").unwrap().get_manga("vinland-saga").await?.unwrap();
    println!("{}", manga);

    println!("\n\n=== Manga Chapter List ===\n");
    let chapters = manga.chapter_list().await?;
    for c in chapters.iter() {
        println!("{}", c);
    }

    println!("\n\n=== Manga Chapter ===\n");
    let chapter_num = 300;
    let chapter = manga.chapter(chapter_num).await?;
    match chapter {
        Some(c) => println!("{c}"),
        None => println!("Chapter {chapter_num} not found!"),
    }

    let chapter_num = 120;
    let chapter = manga.chapter(chapter_num).await?;
    match chapter {
        Some(ref c) => println!("{c}"),
        None => println!("Chapter {chapter_num} not found!"),
    }
    let chapter = chapter.unwrap();

    let page_list = chapter.page_url_list().await?;
    for page in page_list {
        println!("{}", page);
    }

    Ok(())
}
