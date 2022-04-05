// use crate::errors::Result;
// use crate::Source as SourceTrait;
// use crate::{BoxedMangaList, OptionalBoxedManga};
// use crate::{SourceData, SourceInfo};


// impl SourceInfo for Source {
//     fn identifier(&self) -> String {
//         self.identifier.clone()
//     }

//     fn title(&self) -> String {
//         self.title.clone()
//     }

//     fn description(&self) -> String {
//         self.description.clone()
//     }

//     fn base_url(&self) -> String {
//         self.base_url.clone()
//     }
// }

// #[async_trait::async_trait]
// impl<'t> SourceData<'t> for Source {
//     async fn manga_list(&self) -> Result<'t, BoxedMangaList<'t>> {
//         todo!()
//     }

//     async fn latest_manga(&self) -> Result<'t, BoxedMangaList<'t>> {
//         todo!()
//     }

//     async fn popular_manga(&self) -> Result<'t, BoxedMangaList<'t>> {
//         todo!()
//     }

//     async fn hot_manga(&self) -> Result<'t, BoxedMangaList<'t>> {
//         todo!()
//     }

//     async fn search_manga(&self, _manga_title: &str) -> Result<'t, BoxedMangaList<'t>> {
//         todo!()
//     }

//     async fn get_manga(&self, _manga_identifier: &str) -> Result<'t, OptionalBoxedManga<'t>> {
//         todo!()
//     }
// }

// impl<'t> SourceTrait<'t> for Source {}
