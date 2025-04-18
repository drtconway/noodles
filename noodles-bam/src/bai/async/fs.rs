//! Async BAI filesystem operations.

use std::path::Path;

use tokio::{
    fs::File,
    io::{self, BufReader, BufWriter},
};

use super::io::{Reader, Writer};
use crate::bai::Index;

/// Reads the entire contents of a BAM index.
///
/// This is a convenience function and is equivalent to opening the file at the given path, reading
/// the header, and reading the index.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> tokio::io::Result<()> {
/// use noodles_bam::bai;
/// let index = bai::r#async::fs::read("sample.bam.bai").await?;
/// # Ok(())
/// # }
/// ```
pub async fn read<P>(src: P) -> io::Result<Index>
where
    P: AsRef<Path>,
{
    let mut reader = File::open(src).await.map(BufReader::new).map(Reader::new)?;
    reader.read_index().await
}

/// Writes a BAM index to a file.
///
/// This is a convenience function and is equivalent to creating a file at the given path, writing
/// the header, and writing the index.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> tokio::io::Result<()> {
/// use noodles_bam::bai;
/// use noodles_csi as csi;
///
/// let index = bai::Index::default();
/// bai::r#async::fs::write("sample.bam.bai", &index).await?;
/// # Ok(())
/// # }
/// ```
pub async fn write<P>(dst: P, index: &Index) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut writer = File::create(dst)
        .await
        .map(BufWriter::new)
        .map(Writer::new)?;

    writer.write_index(index).await?;

    writer.shutdown().await?;

    Ok(())
}
