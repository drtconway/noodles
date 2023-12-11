use std::io;

/// Alignment record flags.
pub trait Flags {
    /// Converts raw flags to a `u16`.
    fn try_to_u16(&self) -> io::Result<u16>;
}

impl TryFrom<&dyn Flags> for u16 {
    type Error = io::Error;

    fn try_from(raw_flags: &dyn Flags) -> Result<Self, Self::Error> {
        raw_flags.try_to_u16()
    }
}

impl TryFrom<&dyn Flags> for crate::record::Flags {
    type Error = io::Error;

    fn try_from(raw_flags: &dyn Flags) -> Result<Self, Self::Error> {
        u16::try_from(raw_flags)
            .map(Self::from)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_ref_dyn_flags_for_crate_record_flags() -> io::Result<()> {
        struct T(u16);

        impl Flags for T {
            fn try_to_u16(&self) -> io::Result<u16> {
                Ok(self.0)
            }
        }

        let flags = crate::record::Flags::UNMAPPED;
        let raw_flags: &dyn Flags = &T(u16::from(flags));
        assert_eq!(crate::record::Flags::try_from(raw_flags)?, flags);

        Ok(())
    }
}
