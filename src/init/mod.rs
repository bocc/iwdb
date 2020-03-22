pub mod from_file;
pub mod from_web_api;

use crate::configuration::Config;
use crate::init;
use std::collections::HashSet;

pub async fn insert_words(words: &mut HashSet<String>, config: &Config) {
    let res = init::from_file::add_words(words, &config.init.path);

    match res {
        Ok(n) => println!("Inserted {} words from {}", n, &config.init.path),
        Err(e) => eprintln!("Faild to insert words from file because {}", e),
    }

    let res = init::from_web_api::add_words(words, &config.init.web_api).await;

    match res {
        Ok(n) => println!("Inserted {} words from {}", n, &config.init.web_api),
        Err(e) => eprintln!("Faild to insert words from web API because {}", e),
    }
}
