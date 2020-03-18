use hyper::Uri;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub init: Init,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Init {
    pub path: String,
    #[serde(with = "http_serde::uri")]
    pub web_api: Uri,
}

pub(crate) fn parse_configuration<P>(path: P) -> Config
where
    P: AsRef<std::path::Path>,
{
    // if anything fails, defaults are created. it could be more granular though
    let defaults = Config {
        server: Server {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
        },
        init: Init {
            path: "./assets/init_words.txt".to_owned(),
            web_api: "https://random-word-api.herokuapp.com/word?number=10"
                .parse()
                .expect("unvalid uri given for web_api"),
        },
    };

    // since we are not returning errors, both error types are mapped to ()
    let config = std::fs::read_to_string(path)
        .map_err(|e| {
            eprintln!("Couldn't parse config file because {}.\nUsing defaults.", e);
        })
        .and_then(|s| {
            toml::from_str(&s).map_err(|e| {
                eprintln!("Couldn't parse config file because {}.\nUsing defaults.", e);
            })
        })
        .unwrap_or(defaults);

    config
}
