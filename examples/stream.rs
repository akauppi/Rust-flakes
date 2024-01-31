/*
* examples/stream.rs
*
* Reading a file, streamed.
*
* References:
*   - Rust by Example > Std misc > File I/O > 'read_lines'
*       -> https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
*/
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//keep use rayon::prelude::*;
use std::env;

/*
* Usage:
*   $ cargo run --example stream -- {file.txt}
*/
fn main() {
    let args: Vec<_> = env::args().skip(1)
        .collect();
    // Rust note: '.collect()' (creating a 'Vec') is needed because matching needs a slice,
    //      and slice cannot be created from an iterator.

    let p: &Path = match &args[..] {
        [s] => Path::new(s),
        //
        _ => {
            eprintln!("Usage:\
                \n  cargo run --example stream -- file.txt\
            \n");
            std::process::exit(-1);
        }
    };

    if let Ok(lines) = read_lines(p) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}