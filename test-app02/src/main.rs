use pulldown_cmark::{Options, Parser, Event};

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

- [ ] task 1
- [x] task 2
- [ ] task 3
";

  let mut options = Options::empty();
  options.insert(Options::ENABLE_TASKLISTS);
  let parser = Parser::new_ext(markdown_input, options);
  let parser = parser.map(|event| match event {
    Event::SoftBreak => Event::HardBreak,
    _ => event,
  });

  // Write to a new String buffer.
  let mut html_output = String::new();
  pulldown_cmark::html::push_html(&mut html_output, parser);
  // assert_eq!(&html_output, "<p>hello world</p>\n");
  println!("{}", html_output);
}
