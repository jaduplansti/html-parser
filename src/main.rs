use std::fs;
use std::io::BufReader;
use std::io::BufRead;

struct File {
    fname : String, /* file name */
    lines : Vec<String>, /* lines in the file */
}

impl File {
    fn new(name : &String) -> Self {
        return File {
            fname : name.clone(),
            lines : Vec::new(),
        }
    }

    fn read(&mut self, file : fs::File) {
        let reader : BufReader<fs::File> = BufReader::new(file);
        for result in reader.lines() {
            if let Ok(line) = result {
                self.lines.push(line);
            }
        }
    }

}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: ./html-parser [file]");
        std::process::exit(1);
    }
    if let Ok(file) = fs::File::open(&args[1]) {
        let mut sfile : File = File::new(&args[1]);
        sfile.read(file);
        for line in sfile.lines {
            println!("{}", line);
        }
    }
    else {
        println!("./html-parser: failed to open '{}'", &args[1]);
        std::process::exit(1);
    }
}
