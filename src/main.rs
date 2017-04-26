extern crate reqwest;
extern crate regex;

use std::io::Read;

use regex::Regex;

// Regex for links (?:<a)(?:.*?)(?:href="(.*?)")(?:.*?)(?:>)
// Regex for imgs (?:<img)(?:.*?)(?:src="(.*?)")(?:.*?)(?:>)

fn main() {
    let a = Regex::new(r#"(?:<a)(?:.*?)(?:href="(.*?)")(?:.*?)(?:>)"#).unwrap();
    let img = Regex::new(r#"(?:<img)(?:.*?)(?:src="(.*?)")(?:.*?)(?:>)"#).unwrap();
    let mut resp = reqwest::get("https://www.rust-lang.org/en-US/").unwrap();
    assert!(resp.status().is_success());
    let mut conent = String::new();
    resp.read_to_string(&mut conent).unwrap();
    print!("Conent:\n\n{}\n--------\n\n", conent);
    for cap in a.captures_iter(&conent) {
        println!("{}", &cap[1]);
    }
    print!("\n--------\n\n");
    for cap in img.captures_iter(&conent) {
        println!("{}", &cap[1]);
    }
}
