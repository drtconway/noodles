//! Counts the number of records in a GTF file.
//!
//! The result matches the output of `grep --count --invert-match "^#" <src>`.

use std::{
    env,
    fs::File,
    io::{self, BufReader},
};

use noodles_gtf as gtf;

fn main() -> io::Result<()> {
    let src = env::args().nth(1).expect("missing src");

    let mut reader = File::open(src)
        .map(BufReader::new)
        .map(gtf::io::Reader::new)?;

    let mut line = gtf::Line::default();
    let mut n = 0;

    while reader.read_line(&mut line)? != 0 {
        if line.as_record().is_some() {
            n += 1;
        }
    }

    println!("{n}");

    Ok(())
}
