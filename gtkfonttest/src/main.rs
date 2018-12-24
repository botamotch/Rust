extern crate pango;
extern crate pangocairo;
use pango::prelude::*;
use pango::FontDescription;

extern crate gtk;
use gtk::prelude::*;

fn main() {
    // let drawing_area = gtk::DrawingArea::new();
    // let pango_context = drawing_area.create_pango_context().unwrap();
    // pango_context.set_font_description(&FontDescription::from_string("Noto Sans Mono CJK JP 12"));

    print_font_info(&FontDescription::from_string("Noto Sans Mono CJK JP 12"));
    print_font_info(&FontDescription::from_string("DejaVu Sans Mono 12"));
}

fn print_font_info(s: &FontDescription) {
    println!("========================================");
    println!("  Size     : {}", s.get_size());
    match s.get_family() {
        Option::Some(val) => println!("  Family   : {}", val),
        Option::None => println!("  Family not found"),
    }
    match s.to_filename() {
        Option::Some(val) => println!("  Filename : {}", val),
        Option::None => println!("  Filename not found"),
    }
    println!("========================================");
}
