use std::collections::HashMap;

pub mod chapter;
pub mod manga;
pub mod source;

pub mod opex;
pub mod yabu;

use source::Source;

pub fn get_available_sources() -> HashMap<String, Source> {
    // TODO: Insert only sources selected by crate features
    let mut map = HashMap::<String, Source>::new();
    map.insert("opex".to_owned(), opex::OpexSource::default().into());
    map.insert("yabu".to_owned(), yabu::YabuSource::default().into());
    map
}
