use std::fs::File;
use std::io::Read;

use pulldown_cmark::{Event, Options, Parser};
// use pulldown_cmark::Tag;

fn main() {
  let filename = "input.md";

  test(&filename);
}

fn test(filename: &str) {
  let mut file = match File::open(&filename) {
    Err(e) => panic!("couldn't open {}: {}", filename, e),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", filename, why),
    Ok(_) => println!("\n>>> file read success, {}", filename),
  }

  let mut options = Options::empty();
  options.insert(Options::ENABLE_TASKLISTS);
  // options.insert(Options::ENABLE_SMART_PUNCTUATION);
  // options.insert(Options::ENABLE_TABLES);
  // options.insert(Options::ENABLE_FOOTNOTES);
  // options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
  // let parser = Parser::new_ext(markdown_input, options);
  // let parser = parser.map(|event| match event {
  //   Event::SoftBreak => Event::HardBreak,
  //   Event::Start(Tag::Link(link_type, url, title)) => Event::Start(Tag::Link(
  //     link_type,
  //     url.replace("http://", "https://").into(),
  //     title,
  //   )),
  //   Event::Text(text) => Event::Text(
  //     text
  //       .replace("Peter", "John")
  //       .replace("https://zenn.dev", "<https://zenn.dev>")
  //       .into(),
  //   ),
  //   Event::Start(Tag::Paragraph) => event,
  //   _ => event,
  // });

  println!("\n>>> parser map event print");
  let parser = Parser::new(&s).map(|event| {
    match &event {
      Event::Start(tag) => {
        println!(" Start: {:?}", tag);
        event
      }
      Event::End(tag) => {
        println!(" End: {:?}", tag);
        event
      }
      Event::Html(s) => {
        println!("  Html: {:?}", s);
        event
      }
      // Event::InlineHtml(s) => println!("InlineHtml: {:?}", s),
      Event::Text(s) => {
        println!("  Text: {:?}", s);
        match s.to_string() == "https://zenn.dev/" {
          true => Event::Rule,
          false => event,
        }
      }
      Event::Code(s) => {
        println!("  Code: {:?}", s);
        event
      }
      Event::FootnoteReference(s) => {
        println!("  FootnoteReference: {:?}", s);
        event
      }
      Event::TaskListMarker(b) => {
        println!("  TaskListMarker: {:?}", b);
        event
      }
      Event::SoftBreak => {
        println!("  SoftBreak");
        event
        // Event::Rule
      }
      Event::HardBreak => {
        println!("  HardBreak");
        event
      }
      Event::Rule => {
        println!("  Rule");
        event
      }
    }
    // event
  });

  // Write to a new String buffer.
  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, parser);
  // assert_eq!(&html_output, "<p>hello world</p>\n");
  println!("\n>>> html output");
  println!("{}", html_output);
}
