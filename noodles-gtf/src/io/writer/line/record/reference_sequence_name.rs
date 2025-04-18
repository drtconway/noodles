use std::io::{self, Write};

pub(super) fn write_reference_sequence_name<W>(
    writer: &mut W,
    reference_sequence_name: &str,
) -> io::Result<()>
where
    W: Write,
{
    writer.write_all(reference_sequence_name.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_reference_sequence_name() -> io::Result<()> {
        let mut buf = Vec::new();
        write_reference_sequence_name(&mut buf, "sq0")?;
        assert_eq!(buf, b"sq0");
        Ok(())
    }
}
