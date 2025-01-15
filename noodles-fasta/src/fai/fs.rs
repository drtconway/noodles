//! FAI filesystem operations.

use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
    path::Path,
};

use super::{
    io::{Reader, Writer},
    Index,
};

/// Reads the entire contents of a FASTA index.
///
/// This is a convenience function and is equivalent to opening the file at the given path and
/// parsing each record.
///
/// # Examples
///
/// ```no_run
/// use noodles_fasta::fai;
/// let index = fai::fs::read("reference.fa.fai")?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn read<P>(src: P) -> io::Result<Index>
where
    P: AsRef<Path>,
{
    let mut reader = File::open(src).map(BufReader::new).map(Reader::new)?;
    reader.read_index()
}

/// Writes a FASTA index to a file.
///
/// This is a convenience function and is equivalent to creating a file at the given path and
/// writing the index.
///
/// # Examples
///
/// ```no_run
/// use noodles_fasta::fai;
/// let index = fai::Index::default();
/// fai::fs::write("reference.fa.fai", &index)?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn write<P>(dst: P, index: &Index) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut writer = File::create(dst).map(BufWriter::new).map(Writer::new)?;
    writer.write_index(index)?;
    Ok(())
}
