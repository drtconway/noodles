// § 6.3.3.3 "Type encoding: Floats" (2024-10-09)
const NAN: u32 = 0x7fc00000;
const MISSING: u32 = 0x7f800001;
const END_OF_VECTOR: u32 = 0x7f800002;
const RESERVED_0: u32 = 0x7f800003;
const RESERVED_4: u32 = 0x7f800007;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Float {
    Value(f32),
    Missing,
    EndOfVector,
    Reserved(f32),
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        match value.to_bits() {
            NAN => Self::Value(f32::NAN),
            MISSING => Self::Missing,
            END_OF_VECTOR => Self::EndOfVector,
            RESERVED_0..=RESERVED_4 => Self::Reserved(value),
            _ => Self::Value(value),
        }
    }
}

impl From<Float> for f32 {
    fn from(value: Float) -> Self {
        match value {
            Float::Missing => f32::from_bits(MISSING),
            Float::EndOfVector => f32::from_bits(END_OF_VECTOR),
            Float::Value(n) | Float::Reserved(n) => n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f32_for_float() {
        assert!(matches!(
            Float::from(f32::from_bits(0x7fc00000)),
            Float::Value(v) if v.to_bits() == 0x7fc00000
        ));
        assert_eq!(Float::from(f32::from_bits(0x7f800001)), Float::Missing);
        assert_eq!(Float::from(f32::from_bits(0x7f800002)), Float::EndOfVector);
        assert!(matches!(
            Float::from(f32::from_bits(0x7f800003)),
            Float::Reserved(v) if v.to_bits() == 0x7f800003
        ));
        assert!(matches!(
            Float::from(f32::from_bits(0x7f800004)),
            Float::Reserved(v) if v.to_bits() == 0x7f800004
        ));
        assert!(matches!(
            Float::from(f32::from_bits(0x7f800005)),
            Float::Reserved(v) if v.to_bits() == 0x7f800005
        ));
        assert!(matches!(
            Float::from(f32::from_bits(0x7f800006)),
            Float::Reserved(v) if v.to_bits() == 0x7f800006
        ));
        assert!(matches!(
            Float::from(f32::from_bits(0x7f800007)),
            Float::Reserved(v) if v.to_bits() == 0x7f800007
        ));
        assert!(matches!(
            Float::from(f32::from_bits(0x7f800008)),
            Float::Value(v) if v.to_bits() == 0x7f800008
        ));
        assert_eq!(Float::from(0.0), Float::Value(0.0));
        assert_eq!(Float::from(f32::MAX), Float::Value(f32::MAX));
    }

    #[test]
    fn test_from_float_for_f32() {
        assert_eq!(f32::from(Float::Value(f32::NAN)).to_bits(), 0x7fc00000);
        assert_eq!(f32::from(Float::Missing).to_bits(), 0x7f800001);
        assert_eq!(f32::from(Float::EndOfVector).to_bits(), 0x7f800002);
        assert_eq!(
            f32::from(Float::Reserved(f32::from_bits(0x7f800003))).to_bits(),
            0x7f800003
        );
        assert_eq!(
            f32::from(Float::Reserved(f32::from_bits(0x7f800004))).to_bits(),
            0x7f800004
        );
        assert_eq!(
            f32::from(Float::Reserved(f32::from_bits(0x7f800005))).to_bits(),
            0x7f800005
        );
        assert_eq!(
            f32::from(Float::Reserved(f32::from_bits(0x7f800006))).to_bits(),
            0x7f800006
        );
        assert_eq!(
            f32::from(Float::Reserved(f32::from_bits(0x7f800007))).to_bits(),
            0x7f800007
        );
        assert_eq!(
            f32::from(Float::Value(f32::from_bits(0x7f800008))).to_bits(),
            0x7f800008
        );
        assert_eq!(f32::from(Float::Value(0.0)), 0.0);
        assert_eq!(f32::from(Float::Value(f32::MAX)), f32::MAX);
    }
}
