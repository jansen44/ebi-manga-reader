use anyhow::Result;

use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::Client;

use crate::errors::EbiError;
use crate::sources::manga::Manga;

use super::manga::{YabuManga, YabuMangaBuilder};
use super::YABU_BASE_URL;

const API_ACCEPT_HEADER: &str = "application/json, text/plain, */*";
const API_REFERER_HEADER: &str = "https://mangayabu.top/lista-de-mangas/";
const API_ACCEPT_LANGUAGE_HEADER: &str = "pt-BR,pt;q=0.8,en-US;q=0.5,en;q=0.3";
const API_MANGA_LIST_PATH: &str = "/api/show3.php";

const HTML_ACCEPT_HEADER: &str =
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8";
const HTML_ACCEPT_LANGUAGE_HEADER: &str = "pt-BR,pt;q=0.8,en-US;q=0.5,en;q=0.3";

pub async fn yabu_manga_list() -> Result<Vec<Manga>> {
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

pub async fn yabu_chapter_page_list(url: &str) -> Result<Vec<String>> {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT, API_ACCEPT_HEADER.parse().unwrap());
    headers.insert(header::REFERER, API_REFERER_HEADER.parse().unwrap());
    headers.insert(
        header::ACCEPT_LANGUAGE,
        API_ACCEPT_LANGUAGE_HEADER.parse().unwrap(),
    );

    let client = Client::builder().default_headers(headers).build().unwrap();
    let body: serde_json::Value = client.get(url).send().await?.json().await?;

    let raw_chapter_list = body.get("Miko").ok_or(EbiError::ClientInvalidRequestBody(
        "Field not find in JSON response: \"Miko\"",
    ))?;

    let chapter_list = raw_chapter_list.clone();
    let chapter_list = chapter_list.as_array().ok_or(EbiError::ClientInvalidRequestBody(
        "Invalid JSON response data type: expected \"array\"",
    ))?;

    let chapter_list: Vec<String> = chapter_list
        .into_iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect();

    Ok(chapter_list)
}

pub async fn yabu_homepage_html() -> Result<String> {
    let url = YABU_BASE_URL.to_owned();

    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCEPT_LANGUAGE,
        HTML_ACCEPT_LANGUAGE_HEADER.parse().unwrap(),
    );
    headers.insert(header::ACCEPT, HTML_ACCEPT_HEADER.parse().unwrap());

    let client = Client::builder().default_headers(headers).build().unwrap();
    let body = client.get(url).send().await?.text().await?;

    Ok(body)
}

pub async fn yabu_html(url: &str) -> Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCEPT_LANGUAGE,
        HTML_ACCEPT_LANGUAGE_HEADER.parse().unwrap(),
    );
    headers.insert(header::ACCEPT, HTML_ACCEPT_HEADER.parse().unwrap());

    let client = Client::builder().default_headers(headers).build().unwrap();
    let body = client.get(url).send().await?.text().await?;

    Ok(body)
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

impl From<MangaListResponse> for Vec<Manga> {
    fn from(manga_list: MangaListResponse) -> Self {
        let manga_list: Vec<MangaResponse> = manga_list.into();

        manga_list
            .iter()
            .map(|m| YabuManga::from(m).into())
            .collect::<Vec<Manga>>()
    }
}
