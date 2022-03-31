use ebi_sources::errors::Result;
use ebi_sources::yabu::Yabu;

#[tokio::main]
async fn main() -> Result<'static, ()> {
    let yabu = Yabu::new()?;
    let manga_list = yabu.mangas().await?;

    println!("{:?}", manga_list);

    Ok(())
}
