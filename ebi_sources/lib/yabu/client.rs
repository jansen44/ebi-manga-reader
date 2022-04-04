// use reqwest::header;
// use reqwest::header::HeaderMap;
// use reqwest::Client;

// use crate::errors::client::ClientResult;
// use crate::{MManga, SSource};

// const API_ACCEPT_HEADER: &str = "application/json, text/plain, */*";
// const API_REFERER_HEADER: &str = "https://mangayabu.top/lista-de-mangas/";
// const API_ACCEPT_LANGUAGE_HEADER: &str = "pt-BR,pt;q=0.8,en-US;q=0.5,en;q=0.3";

// pub struct YabuClient {
//     api_client: Client,
//     base_url: String,
// }

// impl YabuClient {
//     pub fn new(source: SSource) -> ClientResult<Self> {
//         let headers = Self::build_default_headers();
//         let api_client = Client::builder().default_headers(headers).build().unwrap();

//         Ok(Self {
//             api_client,
//             base_url: source.base_url.clone(),
//         })
//     }

//     fn build_default_headers() -> HeaderMap {
//         let mut headers = HeaderMap::new();
//         headers.insert(header::ACCEPT, API_ACCEPT_HEADER.parse().unwrap());
//         headers.insert(header::REFERER, API_REFERER_HEADER.parse().unwrap());
//         headers.insert(
//             header::ACCEPT_LANGUAGE,
//             API_ACCEPT_LANGUAGE_HEADER.parse().unwrap(),
//         );
//         headers
//     }

//     pub async fn get_manga_list(&self) -> ClientResult<Vec<MManga>> {
//         let url = format!("{}/api/show3.php", self.base_url.clone());
//         let body: MangaListResponse = self.api_client.get(url).send().await?.json().await?;

//         Ok(body.into())
//     }
// }

// #[derive(Debug, serde::Deserialize)]
// struct MangaResponse {
//     #[serde(rename = "hash")]
//     pub _hash: u32,
//     pub title: String,
//     pub genre: String,
//     #[serde(rename = "videos")]
//     pub _videos: u32,
//     pub cover: String,
//     #[serde(rename = "type")]
//     pub _manga_type: String,
//     pub slug: String,
// }

// // impl From<&MangaResponse> for MManga {
// //     fn from(manga: &MangaResponse) -> Self {
// //         let source = Yabu::source();

// //         Self {
// //             identifier: manga.slug.clone(),
// //             source_identifier: source.identifier.clone(),
// //             thumbnail: manga.cover.clone(),
// //             title: manga.title.clone(),
// //             url: format!("{}/manga/{}", source.base_url, manga.slug),
// //             genre: Some(manga.genre.clone()),
// //             description: None,
// //         }
// //     }
// // }

// #[derive(serde::Deserialize, Debug)]
// #[serde(transparent)]
// struct MangaListResponse(Vec<MangaResponse>);

// impl From<MangaListResponse> for Vec<MangaResponse> {
//     fn from(manga_list: MangaListResponse) -> Self {
//         manga_list.0
//     }
// }

// // impl From<MangaListResponse> for Vec<MManga> {
// //     fn from(manga_list: MangaListResponse) -> Self {
// //         let manga_list: Vec<MangaResponse> = manga_list.into();
        
// //         manga_list
// //             .iter()
// //             .map(|m| MManga::from(m))
// //             .collect::<Vec<MManga>>()
// //     }
// // }
