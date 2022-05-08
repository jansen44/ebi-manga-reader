use std::collections::HashMap;

pub mod errors;

pub mod chapter;
pub mod manga;
pub mod source;

pub mod opex;
pub mod yabu;

pub use errors::Result;

use source::Source;

pub fn get_available_sources() -> HashMap<String, Box<dyn Source>> {
    let mut map = HashMap::<String, Box<dyn Source>>::new();
    map.insert("opex".to_owned(), Box::new(opex::OpexSource::default()));
    map.insert("yabu".to_owned(), Box::new(yabu::YabuSource::default()));
    map
}
