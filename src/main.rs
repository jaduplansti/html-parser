use std::fs;
use std::io::BufReader;
use std::io::BufRead;

struct File {
    fname : String, /* file name */
    file : fs::File, /* file */
    lines : Vec<String>, /* lines in the file */
}

impl File {
    fn new(name : &String, f : fs::File) -> Self {
        return File {
            fname : name.clone(),
            file : f,
            lines : Vec::new(),
        }
    }

    fn read(&mut self) {

        self.lines = reade

}

fn main() {
    println!("Hello, world!");
}
