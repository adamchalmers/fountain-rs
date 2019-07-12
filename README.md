# fountain-rs
A library for parsing [Fountain](http://fountain.io) markdown. Fountain is used for screen- or stageplays.

## Usage
Clone this repo, then run
```bash
$ cargo run -- MY_FOUNTAIN_DOC.fountain
```
`fountain-rs` will output HTML to stdout. You can redirect that to a file (to open in a browser) or to a PDF-creating program. My current workflow is to write the output to a file, open the file in Chrome, then print the page into a PDF. There's probably some nifty CLI util that can read HTML from stdin and output a PDF. If you can suggest one, I'll put it here.

## Progress
Eventually I would like `fountain-rs` to be fully compliant with the Fountain spec. Only a subset of the spec has currently been implemented. So far metadata (e.g. Title and Author fields) are implemented, as well as these Fountain elements:
 - Action
 - Character
 - Dialogue
 - Scene
 - Parenthetical

This project's goal is to replace Amazon's recently-deprecated Storywriter as an easy way to write Fountain docs in a browser. Features will get added when I need them for my own personal use. If `fountain-rs` doesn't support your particular use-case, please open an issue.
