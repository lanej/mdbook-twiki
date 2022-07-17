use pulldown_cmark::{Event, Options, Parser, Tag};

// https://twiki.org/cgi-bin/view/TWiki05x01/TextFormattingRules
pub fn to_twiki(content: &str, output: &mut dyn std::io::Write) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    Parser::new_ext(&content, options).for_each(|e| match e {
        Event::Start(s) => match s {
            Tag::Paragraph => {}
            Tag::Heading(level, id, _) => {
                let mut twiki_level = String::new();
                for _ in 0..(level as u8) {
                    twiki_level.push('+');
                }
                write!(output, "---{} {}", twiki_level, id.unwrap_or_default()).unwrap()
            }
            Tag::BlockQuote => todo!(),
            Tag::CodeBlock(c) => {
                if let pulldown_cmark::CodeBlockKind::Fenced(fc) = c {
                    writeln!(output, "<code>{}</code>", fc).unwrap()
                }
            }
            Tag::List(_) => todo!(),
            Tag::Item => todo!(),
            Tag::FootnoteDefinition(_) => todo!(),
            Tag::Table(_) => todo!(),
            Tag::TableHead => todo!(),
            Tag::TableRow => todo!(),
            Tag::TableCell => todo!(),
            Tag::Emphasis => todo!(),
            Tag::Strong => {}
            Tag::Strikethrough => todo!(),
            Tag::Link(_, _, _) => todo!(),
            Tag::Image(_, _, _) => todo!(),
        },
        Event::End(ee) => match ee {
            Tag::Paragraph => write!(output, "\n").unwrap(),
            Tag::Heading(_, _, _) => {}
            Tag::BlockQuote => todo!(),
            Tag::CodeBlock(_) => write!(output, "\n").unwrap(),
            Tag::List(_) => todo!(),
            Tag::Item => todo!(),
            Tag::FootnoteDefinition(_) => todo!(),
            Tag::Table(_) => todo!(),
            Tag::TableHead => todo!(),
            Tag::TableRow => todo!(),
            Tag::TableCell => todo!(),
            Tag::Emphasis => todo!(),
            Tag::Strong => {
                write!(output, "*").unwrap();
            }
            Tag::Strikethrough => todo!(),
            Tag::Link(_, _, _) => todo!(),
            Tag::Image(_, _, _) => todo!(),
        },
        Event::Text(s) => {
            write!(output, "{}", s).unwrap();
        }
        Event::Code(c) => match c {
            pulldown_cmark::CowStr::Boxed(_) => todo!(),
            pulldown_cmark::CowStr::Inlined(s) => write!(output, "={}=", s).unwrap(),
            pulldown_cmark::CowStr::Borrowed(s) => write!(output, "={}=", s).unwrap(),
        },
        Event::Html(_) => todo!(),
        Event::FootnoteReference(_) => todo!(),
        Event::SoftBreak => todo!(),
        Event::HardBreak => todo!(),
        Event::Rule => todo!(),
        Event::TaskListMarker(_) => todo!(),
    });
}

#[derive(Debug)]
struct StringIO {
    buffer: String,
}

impl StringIO {
    fn new() -> Self {
        Self { buffer: String::new() }
    }
}

impl ToString for StringIO {
    fn to_string(&self) -> String {
        self.buffer.clone()
    }
}

impl std::io::Write for StringIO {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.push_str(&String::from_utf8(buf.to_vec()).unwrap());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

#[test]
fn test_inline_code() {
    let mut output = StringIO::new();
    let expected = "=code=\n";
    to_twiki("`code`", &mut output);
    assert!(output.to_string() == expected, "expected = {:?}, actual = {:?}", expected, output.to_string());
}
