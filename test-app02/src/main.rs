use std::fs::File;
use std::io::Read;

use pulldown_cmark::LinkType;
use pulldown_cmark::{Event, Options, Parser, Tag};

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
  let mut is_paragraph_to_link = false;
  let mut width = 0;
  let parser = Parser::new_ext(&s, options).map(|event| {
    match &event {
      Event::Start(tag) => {
        println!("{:width$}Start: {:?}", "", tag);
        width += 2;
        event
      }
      Event::End(tag) => {
        width -= 2;
        println!("{:width$}End: {:?}", "", tag);
        event
      }
      Event::Html(s) => {
        println!("{:width$}Html: {:?}", "", s);
        event
      }
      // Event::InlineHtml(s) => println!("InlineHtml: {:?}", s),
      Event::Text(s) => {
        println!("{:width$}Text: {:?}", "", s);
        match s.to_string() == "https://zenn.dev/" {
          true => {
            is_paragraph_to_link = true;
            Event::Start(Tag::Link(
              LinkType::Autolink,
              "https://zenn.dev/".into(),
              "".into(),
            ))
          }
          false => event,
          // _ => event,
        }
      }
      Event::Code(s) => {
        println!("{:width$}Code: {:?}", "", s);
        event
      }
      Event::FootnoteReference(s) => {
        println!("{:width$}FootnoteReference: {:?}", "", s);
        event
      }
      Event::TaskListMarker(b) => {
        println!("{:width$}TaskListMarker: {:?}", "", b);
        event
      }
      Event::SoftBreak => {
        println!("{:width$}SoftBreak", "");
        event
        // Event::Rule
      }
      Event::HardBreak => {
        println!("{:width$}HardBreak", "");
        event
      }
      Event::Rule => {
        println!("{:width$}Rule", "");
        event
      }
    }
    // event
  });

  // Write to a new String buffer.
  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, parser);
  // assert_eq!(&html_output, "<p>hello world</p>\n");
  let html_output = html_output.replace("<a ", "<a target=\"_blank\" rel=\"nofollow\" ");
  println!("\n>>> html output");
  println!("{}", html_output);
}
