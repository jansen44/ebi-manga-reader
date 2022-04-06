use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Client;

use crate::errors::client::ClientResult;
use crate::manga::Manga;

use super::manga::{YabuManga, YabuMangaBuilder};
use super::YABU_BASE_URL;

const API_ACCEPT_HEADER: &str = "application/json, text/plain, */*";
const API_REFERER_HEADER: &str = "https://mangayabu.top/lista-de-mangas/";
const API_ACCEPT_LANGUAGE_HEADER: &str = "pt-BR,pt;q=0.8,en-US;q=0.5,en;q=0.3";

const API_MANGA_LIST_PATH: &str = "/api/show3.php";

pub async fn yabu_manga_list() -> ClientResult<Vec<Box<dyn Manga>>> {
    let url = format!("{}{}", YABU_BASE_URL, API_MANGA_LIST_PATH);

    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT, API_ACCEPT_HEADER.parse().unwrap());
    headers.insert(header::REFERER, API_REFERER_HEADER.parse().unwrap());
    headers.insert(
        header::ACCEPT_LANGUAGE,
        API_ACCEPT_LANGUAGE_HEADER.parse().unwrap(),
    );

    let client = Client::builder().default_headers(headers).build().unwrap();
    let body: MangaListResponse = client.get(url).send().await?.json().await?;

    Ok(body.into())
}

#[derive(Debug, serde::Deserialize)]
struct MangaResponse {
    #[serde(rename = "hash")]
    pub _hash: u32,
    pub title: String,
    pub genre: String,
    #[serde(rename = "videos")]
    pub _videos: u32,
    pub cover: String,
    #[serde(rename = "type")]
    pub _manga_type: String,
    pub slug: String,
}

impl From<&MangaResponse> for YabuManga {
    fn from(manga: &MangaResponse) -> Self {
        YabuMangaBuilder::new()
            .with_identifier(manga.slug.as_str())
            .with_cover(manga.cover.as_str())
            .with_title(manga.title.as_str())
            .with_genre(manga.genre.as_str())
            .build()
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(transparent)]
struct MangaListResponse(Vec<MangaResponse>);

impl From<MangaListResponse> for Vec<MangaResponse> {
    fn from(manga_list: MangaListResponse) -> Self {
        manga_list.0
    }
}

impl From<MangaListResponse> for Vec<Box<dyn Manga>> {
    fn from(manga_list: MangaListResponse) -> Self {
        let manga_list: Vec<MangaResponse> = manga_list.into();

        manga_list
            .iter()
            .map(|m| Box::new(YabuManga::from(m)) as Box<dyn Manga>)
            .collect::<Vec<Box<dyn Manga>>>()
    }
}
