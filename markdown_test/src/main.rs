use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use pulldown_cmark::{Parser, Options, html};

fn read_file(filename: String) -> Result<String, io::Error> {
    let file = File::open(filename);
    let mut content = String::new();
    match file {
        Ok(mut f) => {
            f.read_to_string(&mut content);
            Ok(content)
        },
        Err(err) => Err(err),
    }
}

fn main() {
    let markdown_input = match read_file("chapter1.md".to_string()) {
        Ok(content) => content,
        Err(reason) => panic!("{}", reason)
    };

    //     let markdown_input = "
    // Hello world, this is a *very simple* example.
    // Goodby, Honey.
    //
    // This is test message.
    // ";

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    // let expected_html = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    // assert_eq!(expected_html, &html_output);
    println!("{}", html_output);
}
