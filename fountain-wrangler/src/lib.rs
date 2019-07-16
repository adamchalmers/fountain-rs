mod utils;

use cfg_if::cfg_if;

use fountain;
use wasm_bindgen::prelude::*;
cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn parse(screenplay: &str) -> String {
    println!("Parsing the following Fountain doc:\n{}", screenplay);

    // Write to String buffer.
    match fountain::parse_document::<(&str, _)>(&screenplay) {
        Err(e) => format!("<h1>Error</h1><p>{:?}</p>", e),
        Ok(("", parsed)) => parsed.as_html(),
        Ok((unparsed, parsed)) => format!(
            "\
            <h1>Unparsed</h1>
            <p>'{}'</p>
            {}",
            unparsed,
            parsed.as_html()
        ),
    }
}
