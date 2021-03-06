mod twiki;

extern crate mdbook;

use std::fs::{self, File};
use std::io;
use twiki::Twiki;

use mdbook::{renderer::RenderContext, BookItem};
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub ignores: Vec<String>,
    pub filename: String,
    pub page_url: Option<String>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            filename: "index.twiki".to_string(),
            ignores: vec![],
            page_url: None,
        }
    }
}

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let _ = fs::create_dir_all(&ctx.destination);

    let cfg: Config = ctx
        .config
        .get_deserialized_opt("output.twiki")
        .unwrap_or_default()
        .unwrap_or_default();

    let mut f = File::create(ctx.destination.join("index.twiki")).unwrap();
    let twiki = Twiki {
        base_url: cfg.page_url,
    };

    // TODO: generate the TOC first
    // TODO: are chapters iterated in order for the SUMMARY.md
    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            twiki.convert(&ch.content, &mut f).unwrap();
        }
    }
}
