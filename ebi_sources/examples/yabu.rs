use ebi_sources::errors::Result;
use ebi_sources::yabu::Yabu;

#[tokio::main]
async fn main() -> Result<'static, ()> {
    let yabu = Yabu::new()?;
    let manga_list = yabu.mangas().await?;

    for manga in manga_list.iter() {
        println!("{:?}", manga);
    }

    Ok(())
}
