# fountain-rs
A library for parsing [Fountain](http://fountain.io) markup. Fountain is used for screen- or stageplays.

[![Fountain's crates.io badge](https://img.shields.io/crates/v/fountain.svg)](https://crates.io/crates/fountain)
[![Fountain's docs.rs badge](https://docs.rs/fountain/badge.svg)](https://docs.rs/fountain)

## Usage
This repo contains a binary and a library. Use `cargo build` to build the binary, then run it like so:
```bash
$ fountain MY_FOUNTAIN_DOC.fountain
```
The binary will output HTML to stdout. You can redirect that to a file (to open in a browser) or to a PDF-creating program. My current workflow is to write the output to a file, open the file in Chrome, then print the page into a PDF. There's probably some nifty CLI util that can read HTML from stdin and output a PDF. If you can suggest one, I'll put it here.

## Progress
Eventually I would like `fountain-rs` to be fully compliant with the Fountain spec. Only a subset of the spec has currently been implemented. So far these Fountain elements are implemented:
 - Action
 - Character
 - Dialogue
 - Scene
 - Parenthetical
 - Title page

This project's goal is to replace Amazon's recently-deprecated Storywriter as an easy way to write Fountain docs in a browser. Features will get added when I need them for my own personal use. If `fountain-rs` doesn't support your particular use-case, please open an issue.

## License
This is licensed under the MIT License or the Unlicense, whichever is most permissive and legally recognized in your jurisdiction.
