use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "BUILD_ENVIRONMENT")]
    build_environment: String,
    #[serde(rename = "ENABLE_PASTE_ATTACHMENTS")]
    allow_attachments: bool,
    #[serde(rename = "MAX_ATTACHMENT_SIZE")]
    max_attachment_size: usize,
    #[serde(rename = "LANGUAGES")]
    syntax_highlight: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        let build_environment = {
            if cfg!(debug_assertions) {
                "dev".to_owned()
            } else {
                "prod".to_owned()
            }
        };
        Config {
            build_environment,
            allow_attachments: true,
            max_attachment_size: 1,
            syntax_highlight: vec!["text".to_owned()]
        }
    }
}
