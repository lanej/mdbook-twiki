use pulldown_cmark::{CowStr, Event, LinkType, Options, Parser, Tag};

#[derive(Default)]
pub struct Twiki {
    pub base_url: Option<String>,
}

impl Twiki {
    // https://twiki.org/cgi-bin/view/TWiki05x01/TextF&ormattingRules
    // TODO: separate markdown parsing and twiki formation
    pub fn convert(&self, content: &str, output: &mut dyn std::io::Write) -> std::io::Result<()> {
        let mut options = Options::empty();
        let mut code_block = false;
        let mut in_list = false;
        let mut emphasis = false;
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        // options.insert(Options::ENABLE_SMART_PUNCTUATION);
        // options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

        for e in Parser::new_ext(&content, options) {
            match e {
                Event::Start(s) => match s {
                    Tag::Paragraph => {}
                    Tag::Heading(level, id, _) => {
                        let mut twiki_level = String::new();
                        for _ in 0..(level as u8) {
                            twiki_level.push('+');
                        }
                        write!(output, "---{} {}", twiki_level, id.unwrap_or_default())?
                    }
                    Tag::BlockQuote => writeln!(output, "<verbatim>")?,
                    Tag::CodeBlock(_) => {
                        code_block = true;
                        writeln!(output, "<verbatim>")?
                    }
                    Tag::List(_) => {
                        in_list = true;
                    }
                    Tag::Item => {}
                    Tag::FootnoteDefinition(_) => todo!(),
                    Tag::Table(_) => todo!(),
                    Tag::TableHead => todo!(),
                    Tag::TableRow => todo!(),
                    Tag::TableCell => todo!(),
                    Tag::Strong => {
                        if emphasis {
                            write!(output, "_")?
                        } else {
                            write!(output, "*")?
                        }
                    }
                    Tag::Emphasis => {
                        emphasis = true;
                        write!(output, "_")?
                    }
                    Tag::Strikethrough => write!(output, "<strike>")?,
                    Tag::Link(t, url, _title) => match t {
                        LinkType::Inline => write!(output, "[[{}][", self.anchor_target(url),)?,
                        LinkType::Reference => todo!(),
                        LinkType::ReferenceUnknown => todo!(),
                        LinkType::Collapsed => todo!(),
                        LinkType::CollapsedUnknown => todo!(),
                        LinkType::Shortcut => todo!(),
                        LinkType::ShortcutUnknown => todo!(),
                        LinkType::Autolink => todo!(),
                        LinkType::Email => write!(output, "<a href=\"mailto:{}\">", url)?,
                    },
                    Tag::Image(_, _, _) => todo!(),
                },
                Event::End(ee) => match ee {
                    Tag::Paragraph => write!(output, "\n")?,
                    Tag::Heading(_, _, _) => write!(output, "\n")?,
                    Tag::BlockQuote => write!(output, "</verbatim>\n")?,
                    Tag::CodeBlock(_) => {
                        code_block = false;
                        write!(output, "</verbatim>\n")?
                    }
                    Tag::List(_) => {
                        in_list = false;
                    }
                    Tag::Item => write!(output, "\n")?,
                    Tag::FootnoteDefinition(_) => todo!(),
                    Tag::Table(_) => todo!(),
                    Tag::TableHead => todo!(),
                    Tag::TableRow => todo!(),
                    Tag::TableCell => todo!(),
                    Tag::Strong => {
                        if emphasis {
                            write!(output, "_")?
                        } else {
                            write!(output, "*")?
                        }
                    }
                    Tag::Emphasis => {
                        emphasis = false;
                        write!(output, "_")?
                    }
                    Tag::Strikethrough => write!(output, "</strike>")?,
                    Tag::Link(t, _url, _title) => match t {
                        LinkType::Inline => write!(output, "]]")?,
                        LinkType::Reference => todo!(),
                        LinkType::ReferenceUnknown => todo!(),
                        LinkType::Collapsed => todo!(),
                        LinkType::CollapsedUnknown => todo!(),
                        LinkType::Shortcut => todo!(),
                        LinkType::ShortcutUnknown => todo!(),
                        LinkType::Autolink => todo!(),
                        LinkType::Email => write!(output, "</a>")?,
                    },
                    Tag::Image(_, _, _) => todo!(),
                },
                Event::Text(s) => {
                    if code_block && s.trim_start().starts_with("```") {
                        // pass
                    } else if code_block {
                        write!(output, "{}", s.trim_end().strip_suffix("```").unwrap_or(&s))
                            .unwrap();
                    } else if in_list {
                        let item = s.to_string();
                        let mut prefix = String::new();
                        item.chars()
                            .take_while(|c| '*' == *c)
                            .skip(1)
                            .for_each(|_| prefix.push_str("\t"));
                        write!(
                            output,
                            "{}* {}",
                            prefix,
                            item.trim_start_matches("*").trim_start()
                        )?;
                    } else {
                        write!(output, "{}", s)?;
                    }
                }
                Event::Code(c) => match c {
                    CowStr::Boxed(s) => write!(output, "={}=", s)?,
                    CowStr::Inlined(s) => write!(output, "={}=", s)?,
                    CowStr::Borrowed(s) => write!(output, "={}=", s)?,
                },
                Event::Html(tag) => write!(output, "{}", tag)?,
                Event::FootnoteReference(_) => todo!(),
                Event::SoftBreak | Event::HardBreak | Event::TaskListMarker(_) => {
                    write!(output, "\n")?
                }
                Event::Rule => write!(output, "\n---\n")?,
            }
        }

        Ok(())
    }

    fn anchor_target(&self, url: CowStr) -> String {
        if url.starts_with("https://") || url.starts_with("http://") {
            url.to_string()
        } else {
            url.chars().skip_while(|c| '#' != *c).collect::<String>()
        }
    }
}

#[derive(Debug)]
struct StringIO {
    buffer: String,
}

impl StringIO {
    fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
}

impl Default for StringIO {
    fn default() -> Self {
        Self::new()
    }
}

impl ToString for StringIO {
    fn to_string(&self) -> String {
        self.buffer.clone()
    }
}

impl std::io::Write for StringIO {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer
            .push_str(&String::from_utf8(buf.to_vec()).unwrap());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[allow(dead_code)]
fn markdown_to_twiki_string(markdown: String) -> String {
    let mut output = StringIO::new();
    Twiki::default().convert(&markdown, &mut output).unwrap();
    return output.to_string();
}

#[test]
fn test_conversion() {
    let matrix = [
        ["`inline_code`", "=inline_code=\n"],
        [
            r#"```ruby
        def fn(i); o; end
        ```"#,
            r#"<verbatim>
        def fn(i); o; end
        </verbatim>
"#,
        ],
        ["# heading 1", "---+ heading 1\n"],
        ["## heading 2", "---++ heading 2\n"],
        ["### heading 3", "---+++ heading 3\n"],
        ["#### heading 4", "---++++ heading 4\n"],
        ["text", "text\n"],
        [
            r#"paragraph 1
            paragraph 2"#,
            "paragraph 1\nparagraph 2\n",
        ],
        ["_italic_", "_italic_\n"],
        ["**bold**", "*bold*\n"],
        ["***bolditalic***", "__bolditalic__\n"],
        [
            "* item 1\n** item 2\n*** item 3\n",
            "* item 1\n\t* item 2\n\t\t* item 3\n",
        ],
        [
            "[title](./file.md#heading-title)",
            "[[#heading-title][title]]\n",
        ],
        [
            "[title](https://www.example.org)",
            "[[https://www.example.org][title]]\n",
        ],
    ];
    matrix.iter().for_each(|[input, expected]: &[&str; 2]| {
        let actual = markdown_to_twiki_string(input.to_string());
        assert!(
            &actual == expected,
            "expected = {:?}, actual = {:?}",
            expected,
            actual
        );
    });
}
