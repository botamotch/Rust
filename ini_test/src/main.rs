extern crate ini;
use ini::Ini;

fn main() {
    let conf = Ini::load_from_file("conf.ini").unwrap();

    let section = conf.section(Some("User".to_owned())).unwrap();
    let tommy = section.get("given_name").unwrap();
    let green = section.get("family_name").unwrap();

    println!("{:?} {:?}", tommy, green);

    let section = conf.section(Some("400G eval board".to_owned())).unwrap();
    let ip_address = section.get("ip_address").unwrap();

    println!("{:?}", ip_address);

    // iterating
    for (sec, prop) in &conf {
        println!("Section: {:?}", sec);
        for (key, value) in prop {
            println!("{:?}:{:?}", key, value);
        }
    }
}
