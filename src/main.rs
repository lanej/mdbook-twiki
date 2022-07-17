mod twiki;

extern crate mdbook;

use std::fs::{self, File};
use std::io;
use twiki::to_twiki;

use mdbook::{renderer::RenderContext, BookItem};
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WordcountConfig {
    pub ignores: Vec<String>,
    pub filename: String,
}

// let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    let _ = fs::create_dir_all(&ctx.destination);
    let mut f = File::create(ctx.destination.join("index.twiki")).unwrap();

    let cfg: WordcountConfig = ctx
        .config
        .get_deserialized_opt("output.wordcount")
        .unwrap_or_default()
        .unwrap_or_default();

    // TODO: generate the TOC first
    // TODO: are chapters iterated in order for the SUMMARY.md
    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            to_twiki(&ch.content, &mut f).unwrap();
        }
    }
}
