use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let mut input:HashMap<String, Vec<String>>= HashMap::new();
    for  line in io::stdin().lock().lines() {
        let mut parts = line.as_ref().unwrap().split(' ');
        let finger_print = parts.next().unwrap().to_string();
        let name = parts.next().unwrap().to_string(); 
        input.entry(finger_print.clone()).or_insert(Vec::new()).push(name.clone());
    }
    print_fingerprint(input);
}

fn print_fingerprint(input: HashMap<String, Vec<String>> ) {
    for key in input.values() {
        for val in key.iter() { 
        println!("{}", val);
        }
        println!();
    }
}