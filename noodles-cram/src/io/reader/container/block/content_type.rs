use std::io;

use bytes::Buf;

use crate::container::block::ContentType;

pub(super) fn get_content_type<B>(src: &mut B) -> io::Result<ContentType>
where
    B: Buf,
{
    let n = src
        .try_get_u8()
        .map_err(|e| io::Error::new(io::ErrorKind::UnexpectedEof, e))?;

    match n {
        0 => Ok(ContentType::FileHeader),
        1 => Ok(ContentType::CompressionHeader),
        2 => Ok(ContentType::SliceHeader),
        3 => Ok(ContentType::Reserved),
        4 => Ok(ContentType::ExternalData),
        5 => Ok(ContentType::CoreData),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid content type",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_content_type() -> io::Result<()> {
        fn t(mut src: &[u8], expected: ContentType) -> io::Result<()> {
            let actual = get_content_type(&mut src)?;
            assert_eq!(actual, expected);
            Ok(())
        }

        t(&[0x00], ContentType::FileHeader)?;
        t(&[0x01], ContentType::CompressionHeader)?;
        t(&[0x02], ContentType::SliceHeader)?;
        t(&[0x03], ContentType::Reserved)?;
        t(&[0x04], ContentType::ExternalData)?;
        t(&[0x05], ContentType::CoreData)?;

        let mut src = &[][..];
        assert!(matches!(
            get_content_type(&mut src),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof
        ));

        let mut src = &[0x06][..];
        assert!(matches!(
            get_content_type(&mut src),
            Err(e) if e.kind() == io::ErrorKind::InvalidData
        ));

        Ok(())
    }
}
