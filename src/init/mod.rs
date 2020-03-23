pub mod from_file;
pub mod from_web_api;

use crate::configuration::Config;
use crate::init;
use kudzu::raw::SkipList;

use super::calculate_hash;

pub async fn insert_words(words: &SkipList<u64>, config: &Config) {
    let from_file = init::from_file::add_words(&config.init.path);

    // None means hash was inserted - but here we skip it
    match from_file {
        Ok(from_file) => {
            println!("Got {} words from {}", from_file.len(), &config.init.path);

            for word in from_file.iter() {
                words.insert(calculate_hash(word));
            }
        }
        Err(e) => eprintln!("Couldn't read file: {}", e),
    }

    let from_uri = init::from_web_api::add_words(&config.init.web_api).await;

    match from_uri {
        Ok(from_uri) => {
            println!("Got {} words from {}", from_uri.len(), &config.init.web_api);

            // None means hash was inserted
            for word in from_uri.iter() {
                words.insert(calculate_hash(word));
            }
        }
        Err(e) => eprintln!("Faild to insert words from web API because {}", e),
    }
}
