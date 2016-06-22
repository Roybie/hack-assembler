pub mod symbol;
pub mod c_instr;

use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;

use symbol::*;
use c_instr::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("uhoh"),
    };

    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Err(_) => panic!("couldn't read file"),
        Ok(_) => (),
    }

    let mut preparse: Vec<String> = Vec::new();

    let lines = string.lines().filter(|line| {
        !line.starts_with('/') && line.len() > 0
    }).map(|line| {
        line.split_whitespace().next().unwrap()
    }).collect::<Vec<&str>>();

    let mut symbols = SymbolTable::new();
    let mut i = 0;
    for line in lines.clone() {
        if line.starts_with('(') {
            //label, add to symbols
            symbols.add_symbol(line.trim_matches(|c| c=='(' || c == ')'), i);
        } else {
            i = i + 1;
        }
    }

    for line in lines {
        if line.starts_with('(') {
        } else {
            if line.starts_with('@') {
                let addr;
                match line.trim_matches('@').parse::<usize>() {
                    Ok(n) => {
                        addr = n;
                    },
                    Err(_) => {
                        addr = symbols.get_symbol(line.trim_matches('@'));
                    },
                }
                let new = format!("{:01$b}", addr, 16);
                preparse.push(new.to_owned());
            } else {
                preparse.push(get_instr(line.to_owned()));
            }
        }
    }

    //println!("{}", string);
    //println!("{:?}", symbols.symbol);
    //println!("{:?}", preparse);
    let fname = format!("{}.hack", path.file_stem().unwrap().to_owned().into_string().unwrap());
    let wpath = Path::new(&fname);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut wfile = match File::create(&wpath) {
        Err(_) => panic!("couldn't create  file!"),
        Ok(file) => file,
    };

    for line in preparse {
        write!(wfile, "{}\n", line).unwrap();
    }
}
