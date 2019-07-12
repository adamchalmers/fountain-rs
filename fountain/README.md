# fountain-rs
A library for parsing [Fountain](http://fountain.io) markup. Fountain is used for screen- or stageplays.

## Usage
This library parses Fountain markup and can render it into print-friendly HTML. From there, some other program can print it or convert it into a PDF. We use [Nom 5](https://crates.io/crates/nom) for parsing.

## Progress
Eventually I would like `fountain-rs` to be fully compliant with the Fountain spec. Only a subset of the spec has currently been implemented. So far metadata (e.g. Title and Author fields) are implemented, as well as these Fountain elements:
 - Action
 - Character
 - Dialogue
 - Scene
 - Parenthetical

This project's goal is to replace Amazon's recently-deprecated Storywriter as an easy way to write Fountain docs in a browser. Features will get added when I need them for my own personal use. If `fountain-rs` doesn't support your particular use-case, please open an issue.

## License
This is licensed under the MIT License or the Unlicense, whichever is most permissive and legally recognized in your jurisdiction. 
