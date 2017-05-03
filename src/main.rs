extern crate reqwest;
extern crate regex;

use std::io::Read;
use std::collections::HashSet;
use std::collections::VecDeque;

use regex::Regex;

// Regex for links (?:<a)(?:.*?)(?:href="(.*?)")(?:.*?)(?:>)
// Regex for imgs (?:<img)(?:.*?)(?:src="(.*?)")(?:.*?)(?:>)
// Regex for URL (?:((?:http|https):(?://))((?:(?:.*?)(?:/))|(?:.*?)))?((?:.*?))(?:(?:(?:#|\?)(?:.*?))?$)

fn main() {
    let a = Regex::new(r#"(?:<a)(?:.*?)(?:href="(.*?)")(?:.*?)(?:>)"#).unwrap();
    let img = Regex::new(r#"(?:<img)(?:.*?)(?:src="(.*?)")(?:.*?)(?:>)"#).unwrap();
    let purl = Regex::new(r#"(?:((?:http|https):(?://))((?:(?:.*?)(?:/))|(?:.*?)))?((?:.*?))(?:(?:(?:#|\?)(?:.*?))?$)"#).unwrap();
    let mut to_visit = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut images = HashSet::new();

    to_visit.push_back("https://zethratech.com/".to_owned());

    while to_visit.len() > 0 {
        let addr = match to_visit.pop_front() {
            Some(s) => s,
            None => break,
        };
        println!("Visiting: {}", addr);
        // println!("\tList: {:?}", visited);
        // println!("\tContains: {}", visited.contains(&addr));
        // println!("\tQueue: {:?}", to_visit);
        visited.insert(addr.clone());
        let parts = match purl.captures(&addr) {
            Some(p) => p,
            None => continue,
        };
        let mut origin = String::new();
        origin.push_str(parts.get(1).unwrap().as_str());
        origin.push_str(parts.get(2).unwrap().as_str());
        let mut resp = match reqwest::get(&addr) {
            Ok(resp) => resp,
            Err(e) => {
                println!("Error visiting `{}`: {}", addr, e);
                continue;
            }
        };
        if !resp.status().is_success() {
            println!("Failed to access: {}", addr);
            continue;
        }
        let mut conent = String::new();
        if resp.read_to_string(&mut conent).is_err() {
            continue;
        }
        for cap in a.captures_iter(&conent) {
            let mut nadd = String::new();
            let parts = match purl.captures(&cap[1]) {
                Some(p) => p,
                None => continue,
            };
            if parts.get(1).is_none() {
                nadd.push_str(&origin);
                nadd.push_str(parts.get(3).unwrap().as_str());
            } else if parts.get(2).is_none() {
                nadd.push_str(parts.get(1).unwrap().as_str());
                nadd.push_str(parts.get(3).unwrap().as_str());
            } else {
                nadd.push_str(parts.get(1).unwrap().as_str());
                nadd.push_str(parts.get(2).unwrap().as_str());
                nadd.push_str(parts.get(3).unwrap().as_str());
            }
            if !visited.contains(&nadd) {
                // println!("\tSkipping: {}", nadd);
                to_visit.push_back(nadd);
            }
        }
        for cap in img.captures_iter(&conent) {
            let mut nadd = String::new();
            let parts = match purl.captures(&cap[1]) {
                Some(p) => p,
                None => continue,
            };
            if parts.get(1).is_none() || parts.get(2).is_none() {
                nadd.push_str(&origin);
            } else {
                nadd.push_str(parts.get(1).unwrap().as_str());
                nadd.push_str(parts.get(2).unwrap().as_str());
            }
            nadd.push_str(parts.get(3).unwrap().as_str());
            images.insert(nadd);
        }
    }

    println!("Done:");
    for i in images {
        println!("{}", i);
    }
}
