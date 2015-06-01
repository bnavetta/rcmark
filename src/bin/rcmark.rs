extern crate rcmark;

use std::io::{self, Read};

/// A *very* basic command-line tool.
/// CommonMark is read from standard input
/// and written as HTML to standard output.
fn main() {
    let mut inbuf = String::new();

    match io::stdin().read_to_string(&mut inbuf) {
        Ok(_) => (),
        Err(e) => panic!("Unable to read input: {}", e),
    };

    let doc = rcmark::parse_document(&inbuf, rcmark::DEFAULT);

    println!("{}", rcmark::render_html(&doc, rcmark::DEFAULT));
}
