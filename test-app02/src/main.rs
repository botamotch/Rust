use pulldown_cmark::{Event, Options, Parser, Tag};

fn main() {
  println!(">>> Hello, pulldown-cmark!");
  test();
}

fn test() {
  // Create parser with example Markdown text.
  let markdown_input = "
# markdown

hello world
goodbye honey

this is a ~~complicated~~ *very simple* example.

[Zenn by https](https://zenn.dev/)

[Zenn by http](http://zenn.dev/){:target=\"_blank\"}

auto link by angle bracket <>
<https://zenn.dev/>

https://zenn.dev/

- [ ] task 1
- [x] task 2
- [ ] task 3

Peter
Peter

Peter Peter

Peters

Jhon
";

  let mut options = Options::empty();
  options.insert(Options::ENABLE_TASKLISTS);
  // options.insert(Options::ENABLE_SMART_PUNCTUATION);
  // options.insert(Options::ENABLE_TABLES);
  // options.insert(Options::ENABLE_FOOTNOTES);
  // options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
  let parser = Parser::new_ext(markdown_input, options);
  let parser = parser.map(|event| match event {
    Event::SoftBreak => Event::HardBreak,
    Event::Start(Tag::Link(link_type, url, title)) => Event::Start(Tag::Link(
      link_type,
      url.replace("http://", "https://").into(),
      title,
    )),
    Event::Text(text) => Event::Text(
      text
        .replace("Peter", "John")
        .replace("https://zenn.dev", "<https://zenn.dev>")
        .into(),
    ),
    Event::Start(Tag::Paragraph) => event,
    _ => event,
  });

  // Write to a new String buffer.
  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, parser);
  // assert_eq!(&html_output, "<p>hello world</p>\n");
  println!("{}", html_output);
}
