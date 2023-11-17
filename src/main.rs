use std::fs;
use std::io::BufReader;
use std::io::BufRead;

type NodeId = usize;
struct File {
    fname : String, /* file name */
    lines : Vec<String>, /* lines in the file */
}

struct DomTree {
    nodes : Vec<Node>,
}

struct Node {
    value : String,
    childs : Vec<NodeId>,
}

impl DomTree {
    fn new() -> Self {
        return DomTree {nodes : Vec::new()}
    }
    
    fn insert(&mut self, val : String, parent : Option<NodeId>) {
        if let Some(p_id) = parent {
            self.nodes.push(Node::create(val));
            let id : NodeId = self.nodes.len();
            self.nodes[p_id].childs.push(id);
        }
        else {
            self.nodes.push(Node::create(val));
        }
    }

}

impl Node {
    fn create(val : String) -> Self {
        return Node {
            value : val,
            childs : Vec::new(),
        }
    }   
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
    let mut dom : DomTree = DomTree::new();
    dom.insert("<html>".to_string(), None);
    dom.insert("<head>".to_string(), Some(0));
    println!("{} -> {}", &dom.nodes[0].value, &dom.nodes[0].childs[0])
    /* let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: ./html-parser [file]");
        std::process::exit(1);
    }
    if let Ok(file) = fs::File::open(&args[1]) {
        let mut sfile : File = File::new(&args[1]);
        sfile.read(file);
    }
    else {
        println!("./html-parser: failed to open '{}'", &args[1]);
        std::process::exit(1);
    } */
}
