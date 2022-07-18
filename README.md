# mdbook-twiki

A backend for [mdbook](https://github.com/rust-lang/mdBook) to render to [Twiki](https://twiki.org/cgi-bin/view/TWiki/TextFormattingRules).

## Installation

If you want to use only this preprocessor, install the tool:

```sh
cargo install mdbook-twiki
```

Add an `output` table to your `book.toml`:

```toml
[output.twiki]
```

Finally, build your book as normal:

```sh
mdbook path/to/book
```

## Configuration

### Filename

By default, the output will be written to `index.twiki`.  Set `filename` to customize.

```toml
[output.twiki]
filename = "article.twiki"
```

## License

MIT. See [LICENSE](LICENSE).
