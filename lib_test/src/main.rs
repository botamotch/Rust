mod tests;

fn main() {
    let mut data: Vec<String> = Vec::new();
    data.push("hello world".to_string());
    data.push("goodbye honey".to_string());

    println!("> Hello, world! {:?}", data);
    tests::print_hello();
    tests::write_to_file("output.txt".to_string(), data).unwrap();
}
