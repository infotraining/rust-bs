use std::io;
use std::io::prelude::*;
use std::error::Error;
use std::path::PathBuf;

fn grep<R>(reader: &mut R, target: &str) -> io::Result<()> 
    where R: BufRead 
{
    for line_read in reader.lines() {
        let line = line_read?;
        if line.contains(target) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn grep_main() -> Result<(), Box<dyn Error>> {
    let mut args= std::env::args().skip(1);

    let target = match args.next() {
        Some(arg) => arg,
        None => return Err("Usage: grep PATTERN FILE...".into()),
    };
    
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        let stdin = io::stdin();
        let mut stdin_handle = stdin.lock();
        grep(&mut stdin_handle, &target)?;
    } else {
        for file in files {
            let f = std::fs::File::open(file)?;
            let mut reader = io::BufReader::new(f);
            grep(&mut reader, &target)?;
        }
    }

    Ok(())
}

fn main() {
    let result = grep_main();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
