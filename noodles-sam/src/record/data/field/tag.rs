//! SAM record data field tag.

mod other;
mod standard;

use std::{borrow::Borrow, error, fmt, str::FromStr};

use self::{other::Other, standard::Standard};

/// The smallest template-independent mapping quality in the template (`AM`).
pub const MIN_MAPPING_QUALITY: Tag = Tag::Standard(Standard::MinMappingQuality);

/// Alignment score generated by aligner (`AS`).
pub const ALIGNMENT_SCORE: Tag = Tag::Standard(Standard::AlignmentScore);

/// Barcode sequence identifying the sample (`BC`).
pub const SAMPLE_BARCODE_SEQUENCE: Tag = Tag::Standard(Standard::SampleBarcodeSequence);

/// Offset to base alignment quality (BAQ) (`BQ`).
pub const BASE_ALIGNMENT_QUALITY_OFFSETS: Tag =
    Tag::Standard(Standard::BaseAlignmentQualityOffsets);

/// Phred quality of the unique molecular barcode bases in the `OX` tag (`BZ`).
pub const ORIGINAL_UMI_QUALITY_SCORES: Tag = Tag::Standard(Standard::OriginalUmiQualityScores);

/// Cell identifier (`CB`).
pub const CELL_BARCODE_ID: Tag = Tag::Standard(Standard::CellBarcodeId);

/// Reference name of the next hit (`CC`).
pub const NEXT_HIT_REFERENCE_SEQUENCE_NAME: Tag =
    Tag::Standard(Standard::NextHitReferenceSequenceName);

/// BAM only: `CIGAR` in BAM's binary encoding if (and only if) it consists of > 65535 operators
/// (`CG`).
pub const CIGAR: Tag = Tag::Standard(Standard::Cigar);

/// Edit distance between the color sequence and the color reference (see also `NM`) (`CM`).
pub const COLOR_EDIT_DISTANCE: Tag = Tag::Standard(Standard::ColorEditDistance);

/// Free-text comments (`CO`).
pub const COMMENT: Tag = Tag::Standard(Standard::Comment);

/// Leftmost coordinate of the next hit (`CP`).
pub const NEXT_HIT_POSITION: Tag = Tag::Standard(Standard::NextHitPosition);

/// Color read base qualities (`CQ`).
pub const COLOR_QUALITY_SCORES: Tag = Tag::Standard(Standard::ColarQualityScores);

/// Cellular barcode sequence bases (uncorrected) (`CR`).
pub const CELL_BARCODE_SEQUENCE: Tag = Tag::Standard(Standard::CellBarcodeSequence);

/// Color read sequence (`CS`).
pub const COLOR_SEQUENCE: Tag = Tag::Standard(Standard::ColorSequence);

/// Complete read annotation tag, used for consensus annotation dummy features (`CT`).
pub const COMPLETE_READ_ANNOTATIONS: Tag = Tag::Standard(Standard::CompleteReadAnnotations);

/// Phred quality of the cellular barcode sequence in the `CR` tag (`CY`).
pub const CELL_BARCODE_QUALITY_SCORES: Tag = Tag::Standard(Standard::CellBarcodeQualityScores);

/// The 2nd most likely base calls (`E2`).
pub const NEXT_HIT_SEQUENCE: Tag = Tag::Standard(Standard::NextHitSequence);

/// The index of segment in the template (`FI`).
pub const SEGMENT_INDEX: Tag = Tag::Standard(Standard::SegmentIndex);

/// Segment suffix (`FS`).
pub const SEGMENT_SUFFIX: Tag = Tag::Standard(Standard::SegmentSuffix);

/// Flow signal intensities (`FZ`).
pub const ALTERNATIVE_SEQUENCE: Tag = Tag::Standard(Standard::AlternativeSequence);

/// Reserved for backwards compatibility reasons (`GC`).
pub const RESERVED_GC: Tag = Tag::Standard(Standard::ReservedGc);

/// Reserved for backwards compatibility reasons (`GQ`).
pub const RESERVED_GQ: Tag = Tag::Standard(Standard::ReservedGq);

/// Reserved for backwards compatibility reasons (`GS`).
pub const RESERVED_GS: Tag = Tag::Standard(Standard::ReservedGs);

/// Number of perfect hits (`H0`).
pub const PERFECT_HIT_COUNT: Tag = Tag::Standard(Standard::PerfectHitCount);

/// Number of 1-difference hits (see also `NM`) (`H1`).
pub const ONE_DIFFERENCE_HIT_COUNT: Tag = Tag::Standard(Standard::OneDifferenceHitCount);

/// Number of 2-difference hits (`H2`).
pub const TWO_DIFFERENCE_HIT_COUNT: Tag = Tag::Standard(Standard::TwoDifferenceHitCount);

/// Query hit index (`HI`).
pub const HIT_INDEX: Tag = Tag::Standard(Standard::HitIndex);

/// Query hit total count (`IH`).
pub const TOTAL_HIT_COUNT: Tag = Tag::Standard(Standard::TotalHitCount);

/// Library (`LB`).
pub const LIBRARY: Tag = Tag::Standard(Standard::Library);

/// CIGAR string for mate/next segment (`MC`).
pub const MATE_CIGAR: Tag = Tag::Standard(Standard::MateCigar);

/// String encoding mismatched and deleted reference bases (`MD`).
pub const MISMATCHED_POSITIONS: Tag = Tag::Standard(Standard::MismatchedPositions);

/// Reserved for backwards compatibility reasons (`MF`).
pub const RESERVED_MF: Tag = Tag::Standard(Standard::ReservedMf);

/// Molecular identifier (`MI`).
///
/// A string that uniquely identifies the molecule from which the record was derived.
pub const UMI_ID: Tag = Tag::Standard(Standard::UmiId);

/// Base modification probabilities (`ML`).
pub const BASE_MODIFICATION_PROBABILITIES: Tag =
    Tag::Standard(Standard::BaseModificationProbabilities);

/// Base modifications / methylation (`MM`).
pub const BASE_MODIFICATIONS: Tag = Tag::Standard(Standard::BaseModifications);

/// Length of sequence at the time `MM` and `ML` were produced (`MN`).
pub const BASE_MODIFICATION_SEQUENCE_LENGTH: Tag =
    Tag::Standard(Standard::BaseModificationSequenceLength);

/// Mapping quality of the mate/next segment (`MQ`).
pub const MATE_MAPPING_QUALITY: Tag = Tag::Standard(Standard::MateMappingQuality);

/// Number of reported alignments that contain the query in the current record (`NH`).
pub const ALIGNMENT_HIT_COUNT: Tag = Tag::Standard(Standard::AlignmentHitCount);

/// Edit distance to the reference (`NM`).
pub const EDIT_DISTANCE: Tag = Tag::Standard(Standard::EditDistance);

/// Original alignment (`OA`).
pub const ORIGINAL_ALIGNMENT: Tag = Tag::Standard(Standard::OriginalAlignment);

/// Original CIGAR (deprecated; use `OA` instead) (`OC`).
pub const ORIGINAL_CIGAR: Tag = Tag::Standard(Standard::OriginalCigar);

/// Original mapping position (deprecated; use `OA` instead) (`OP`).
pub const ORIGINAL_POSITION: Tag = Tag::Standard(Standard::OriginalPosition);

/// Original base quality (`OQ`).
pub const ORIGINAL_QUALITY_SCORES: Tag = Tag::Standard(Standard::OriginalQualityScores);

/// Original unique molecular barcode bases (`OX`).
pub const ORIGINAL_UMI_BARCODE_SEQUENCE: Tag = Tag::Standard(Standard::OriginalUmiBarcodeSequence);

/// Program (`PG`).
pub const PROGRAM: Tag = Tag::Standard(Standard::Program);

/// Phred likelihood of the template (`PQ`).
pub const TEMPLATE_LIKELIHOOD: Tag = Tag::Standard(Standard::TemplateLikelihood);

/// Read annotations for parse of the padded read sequence (`PT`).
pub const PADDED_READ_ANNOTATIONS: Tag = Tag::Standard(Standard::PaddedReadAnnotations);

/// Platform unit (`PU`).
pub const PLATFORM_UNIT: Tag = Tag::Standard(Standard::PlatformUnit);

/// Phred quality of the mate/next segment sequence in the `R2` tag (`Q2`).
pub const MATE_QUALITY_SCORES: Tag = Tag::Standard(Standard::MateQualityScores);

/// Phred quality of the sample barcode sequence in the `BC` tag (`QT`).
pub const SAMPLE_BARCODE_QUALITY_SCORES: Tag = Tag::Standard(Standard::SampleBarcodeQualityScores);

/// Quality score of the unique molecular identifier in the `RX` tag (`QX`).
pub const UMI_QUALITY_SCORES: Tag = Tag::Standard(Standard::UmiQualityScores);

/// Sequence of the mate/next segment in the template (`R2`).
pub const MATE_SEQUENCE: Tag = Tag::Standard(Standard::MateSequence);

/// Read group (`RG`).
pub const READ_GROUP: Tag = Tag::Standard(Standard::ReadGroup);

/// Reserved for backwards compatibility reasons (`RT`).
pub const RESERVED_RT: Tag = Tag::Standard(Standard::ReservedRt);

/// Sequence bases of the (possibly corrected) unique molecular identifier (`RX`).
pub const UMI_SEQUENCE: Tag = Tag::Standard(Standard::UmiSequence);

/// Reserved for backwards compatibility reasons (`S2`).
pub const RESERVED_S2: Tag = Tag::Standard(Standard::ReservedS2);

/// Other canonical alignments in a chimeric alignment (`SA`).
pub const OTHER_ALIGNMENTS: Tag = Tag::Standard(Standard::OtherAlignments);

/// Template-independent mapping quality (`SM`).
pub const TEMPLATE_MAPPING_QUALITY: Tag = Tag::Standard(Standard::TemplateMappingQuality);

/// Reserved for backwards compatibility reasons (`SQ`).
pub const RESERVED_SQ: Tag = Tag::Standard(Standard::ReservedSq);

/// The number of segments in the template (`TC`).
pub const SEGMENT_COUNT: Tag = Tag::Standard(Standard::SegmentCount);

/// Transcript strand (`TS`).
pub const TRANSCRIPT_STRAND: Tag = Tag::Standard(Standard::TranscriptStrand);

/// Phred probability of the 2nd call being wrong conditional on the best being wrong (`U2`).
pub const NEXT_HIT_QUALITY_SCORES: Tag = Tag::Standard(Standard::NextHitQualityScores);

/// Phred likelihood of the segment, conditional on the mapping being correct (`UQ`).
pub const SEGMENT_LIKELIHOOD: Tag = Tag::Standard(Standard::SegmentLikelihood);

const LENGTH: usize = 2;

/// A SAM record data field tag.
///
/// Standard tags are defined in "Sequence Alignment/Map Optional Fields Specification"
/// (2020-05-29).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Tag {
    /// A standard tag.
    Standard(Standard),
    /// A non-standard tag.
    Other(Other),
}

impl AsRef<[u8; LENGTH]> for Tag {
    fn as_ref(&self) -> &[u8; LENGTH] {
        match self {
            Self::Standard(tag) => tag.as_ref(),
            Self::Other(tag) => tag.as_ref(),
        }
    }
}

impl Borrow<[u8; LENGTH]> for Tag {
    fn borrow(&self) -> &[u8; LENGTH] {
        self.as_ref()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [b0, b1] = self.as_ref();
        char::from(*b0).fmt(f)?;
        char::from(*b1).fmt(f)?;
        Ok(())
    }
}

/// An error returned when a raw SAM record data field tag fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The input is empty.
    Empty,
    /// The length is invalid.
    ///
    /// The tag length must be 2 characters.
    InvalidLength(usize),
    /// A character is invalid.
    InvalidCharacter(char),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::InvalidLength(len) => {
                write!(f, "invalid length: expected {LENGTH}, got {len}")
            }
            Self::InvalidCharacter(c) => write!(f, "invalid character: {c}"),
        }
    }
}

// § 1.5 The alignment section: optional fields (2021-01-07)
impl TryFrom<[u8; LENGTH]> for Tag {
    type Error = ParseError;

    fn try_from(b: [u8; LENGTH]) -> Result<Self, Self::Error> {
        match Standard::try_from(b) {
            Ok(tag) => Ok(Self::Standard(tag)),
            Err(_) => {
                if !b[0].is_ascii_alphabetic() {
                    Err(ParseError::InvalidCharacter(char::from(b[0])))
                } else if !b[1].is_ascii_alphanumeric() {
                    Err(ParseError::InvalidCharacter(char::from(b[1])))
                } else {
                    Ok(Self::Other(Other(b)))
                }
            }
        }
    }
}

impl TryFrom<&[u8]> for Tag {
    type Error = ParseError;

    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        if b.is_empty() {
            Err(ParseError::Empty)
        } else if b.len() == LENGTH {
            Self::try_from([b[0], b[1]])
        } else {
            Err(ParseError::InvalidLength(b.len()))
        }
    }
}

impl FromStr for Tag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Tag::try_from(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(MIN_MAPPING_QUALITY.to_string(), "AM");
        assert_eq!(ALIGNMENT_SCORE.to_string(), "AS");
        assert_eq!(SAMPLE_BARCODE_SEQUENCE.to_string(), "BC");
        assert_eq!(BASE_ALIGNMENT_QUALITY_OFFSETS.to_string(), "BQ");
        assert_eq!(ORIGINAL_UMI_QUALITY_SCORES.to_string(), "BZ");
        assert_eq!(CELL_BARCODE_ID.to_string(), "CB");
        assert_eq!(NEXT_HIT_REFERENCE_SEQUENCE_NAME.to_string(), "CC");
        assert_eq!(CIGAR.to_string(), "CG");
        assert_eq!(COLOR_EDIT_DISTANCE.to_string(), "CM");
        assert_eq!(COMMENT.to_string(), "CO");
        assert_eq!(NEXT_HIT_POSITION.to_string(), "CP");
        assert_eq!(COLOR_QUALITY_SCORES.to_string(), "CQ");
        assert_eq!(CELL_BARCODE_SEQUENCE.to_string(), "CR");
        assert_eq!(COLOR_SEQUENCE.to_string(), "CS");
        assert_eq!(COMPLETE_READ_ANNOTATIONS.to_string(), "CT");
        assert_eq!(CELL_BARCODE_QUALITY_SCORES.to_string(), "CY");
        assert_eq!(NEXT_HIT_SEQUENCE.to_string(), "E2");
        assert_eq!(SEGMENT_INDEX.to_string(), "FI");
        assert_eq!(SEGMENT_SUFFIX.to_string(), "FS");
        assert_eq!(ALTERNATIVE_SEQUENCE.to_string(), "FZ");
        assert_eq!(RESERVED_GC.to_string(), "GC");
        assert_eq!(RESERVED_GQ.to_string(), "GQ");
        assert_eq!(RESERVED_GS.to_string(), "GS");
        assert_eq!(PERFECT_HIT_COUNT.to_string(), "HO");
        assert_eq!(ONE_DIFFERENCE_HIT_COUNT.to_string(), "H1");
        assert_eq!(TWO_DIFFERENCE_HIT_COUNT.to_string(), "H2");
        assert_eq!(HIT_INDEX.to_string(), "HI");
        assert_eq!(TOTAL_HIT_COUNT.to_string(), "IH");
        assert_eq!(LIBRARY.to_string(), "LB");
        assert_eq!(MATE_CIGAR.to_string(), "MC");
        assert_eq!(MISMATCHED_POSITIONS.to_string(), "MD");
        assert_eq!(RESERVED_MF.to_string(), "MF");
        assert_eq!(UMI_ID.to_string(), "MI");
        assert_eq!(BASE_MODIFICATION_PROBABILITIES.to_string(), "ML");
        assert_eq!(BASE_MODIFICATIONS.to_string(), "MM");
        assert_eq!(MATE_MAPPING_QUALITY.to_string(), "MQ");
        assert_eq!(ALIGNMENT_HIT_COUNT.to_string(), "NH");
        assert_eq!(EDIT_DISTANCE.to_string(), "NM");
        assert_eq!(ORIGINAL_ALIGNMENT.to_string(), "OA");
        assert_eq!(ORIGINAL_CIGAR.to_string(), "OC");
        assert_eq!(ORIGINAL_POSITION.to_string(), "OP");
        assert_eq!(ORIGINAL_QUALITY_SCORES.to_string(), "OQ");
        assert_eq!(ORIGINAL_UMI_BARCODE_SEQUENCE.to_string(), "OX");
        assert_eq!(PROGRAM.to_string(), "PG");
        assert_eq!(TEMPLATE_LIKELIHOOD.to_string(), "PQ");
        assert_eq!(PADDED_READ_ANNOTATIONS.to_string(), "PT");
        assert_eq!(PLATFORM_UNIT.to_string(), "PU");
        assert_eq!(MATE_QUALITY_SCORES.to_string(), "Q2");
        assert_eq!(SAMPLE_BARCODE_QUALITY_SCORES.to_string(), "QT");
        assert_eq!(UMI_QUALITY_SCORES.to_string(), "QX");
        assert_eq!(MATE_SEQUENCE.to_string(), "R2");
        assert_eq!(READ_GROUP.to_string(), "RG");
        assert_eq!(RESERVED_RT.to_string(), "RT");
        assert_eq!(UMI_SEQUENCE.to_string(), "RX");
        assert_eq!(RESERVED_S2.to_string(), "S2");
        assert_eq!(OTHER_ALIGNMENTS.to_string(), "SA");
        assert_eq!(TEMPLATE_MAPPING_QUALITY.to_string(), "SM");
        assert_eq!(RESERVED_SQ.to_string(), "SQ");
        assert_eq!(SEGMENT_COUNT.to_string(), "TC");
        assert_eq!(TRANSCRIPT_STRAND.to_string(), "TS");
        assert_eq!(NEXT_HIT_QUALITY_SCORES.to_string(), "U2");
        assert_eq!(SEGMENT_LIKELIHOOD.to_string(), "UQ");
        assert_eq!(Tag::Other(Other([b'Z', b'N'])).to_string(), "ZN");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("AM".parse(), Ok(MIN_MAPPING_QUALITY));
        assert_eq!("AS".parse(), Ok(ALIGNMENT_SCORE));
        assert_eq!("BC".parse(), Ok(SAMPLE_BARCODE_SEQUENCE));
        assert_eq!("BQ".parse(), Ok(BASE_ALIGNMENT_QUALITY_OFFSETS));
        assert_eq!("BZ".parse(), Ok(ORIGINAL_UMI_QUALITY_SCORES));
        assert_eq!("CB".parse(), Ok(CELL_BARCODE_ID));
        assert_eq!("CC".parse(), Ok(NEXT_HIT_REFERENCE_SEQUENCE_NAME));
        assert_eq!("CG".parse(), Ok(CIGAR));
        assert_eq!("CM".parse(), Ok(COLOR_EDIT_DISTANCE));
        assert_eq!("CO".parse(), Ok(COMMENT));
        assert_eq!("CP".parse(), Ok(NEXT_HIT_POSITION));
        assert_eq!("CQ".parse(), Ok(COLOR_QUALITY_SCORES));
        assert_eq!("CR".parse(), Ok(CELL_BARCODE_SEQUENCE));
        assert_eq!("CS".parse(), Ok(COLOR_SEQUENCE));
        assert_eq!("CT".parse(), Ok(COMPLETE_READ_ANNOTATIONS));
        assert_eq!("CY".parse(), Ok(CELL_BARCODE_QUALITY_SCORES));
        assert_eq!("E2".parse(), Ok(NEXT_HIT_SEQUENCE));
        assert_eq!("FI".parse(), Ok(SEGMENT_INDEX));
        assert_eq!("FS".parse(), Ok(SEGMENT_SUFFIX));
        assert_eq!("FZ".parse(), Ok(ALTERNATIVE_SEQUENCE));
        assert_eq!("GC".parse(), Ok(RESERVED_GC));
        assert_eq!("GQ".parse(), Ok(RESERVED_GQ));
        assert_eq!("GS".parse(), Ok(RESERVED_GS));
        assert_eq!("HO".parse(), Ok(PERFECT_HIT_COUNT));
        assert_eq!("H1".parse(), Ok(ONE_DIFFERENCE_HIT_COUNT));
        assert_eq!("H2".parse(), Ok(TWO_DIFFERENCE_HIT_COUNT));
        assert_eq!("HI".parse(), Ok(HIT_INDEX));
        assert_eq!("IH".parse(), Ok(TOTAL_HIT_COUNT));
        assert_eq!("LB".parse(), Ok(LIBRARY));
        assert_eq!("MC".parse(), Ok(MATE_CIGAR));
        assert_eq!("MD".parse(), Ok(MISMATCHED_POSITIONS));
        assert_eq!("MF".parse(), Ok(RESERVED_MF));
        assert_eq!("MI".parse(), Ok(UMI_ID));
        assert_eq!("ML".parse(), Ok(BASE_MODIFICATION_PROBABILITIES));
        assert_eq!("MM".parse(), Ok(BASE_MODIFICATIONS));
        assert_eq!("MN".parse(), Ok(BASE_MODIFICATION_SEQUENCE_LENGTH));
        assert_eq!("MQ".parse(), Ok(MATE_MAPPING_QUALITY));
        assert_eq!("NH".parse(), Ok(ALIGNMENT_HIT_COUNT));
        assert_eq!("NM".parse(), Ok(EDIT_DISTANCE));
        assert_eq!("OA".parse(), Ok(ORIGINAL_ALIGNMENT));
        assert_eq!("OC".parse(), Ok(ORIGINAL_CIGAR));
        assert_eq!("OP".parse(), Ok(ORIGINAL_POSITION));
        assert_eq!("OQ".parse(), Ok(ORIGINAL_QUALITY_SCORES));
        assert_eq!("OX".parse(), Ok(ORIGINAL_UMI_BARCODE_SEQUENCE));
        assert_eq!("PG".parse(), Ok(PROGRAM));
        assert_eq!("PQ".parse(), Ok(TEMPLATE_LIKELIHOOD));
        assert_eq!("PT".parse(), Ok(PADDED_READ_ANNOTATIONS));
        assert_eq!("PU".parse(), Ok(PLATFORM_UNIT));
        assert_eq!("Q2".parse(), Ok(MATE_QUALITY_SCORES));
        assert_eq!("QT".parse(), Ok(SAMPLE_BARCODE_QUALITY_SCORES));
        assert_eq!("QX".parse(), Ok(UMI_QUALITY_SCORES));
        assert_eq!("R2".parse(), Ok(MATE_SEQUENCE));
        assert_eq!("RG".parse(), Ok(READ_GROUP));
        assert_eq!("RT".parse(), Ok(RESERVED_RT));
        assert_eq!("RX".parse(), Ok(UMI_SEQUENCE));
        assert_eq!("S2".parse(), Ok(RESERVED_S2));
        assert_eq!("SA".parse(), Ok(OTHER_ALIGNMENTS));
        assert_eq!("SM".parse(), Ok(TEMPLATE_MAPPING_QUALITY));
        assert_eq!("SQ".parse(), Ok(RESERVED_SQ));
        assert_eq!("TC".parse(), Ok(SEGMENT_COUNT));
        assert_eq!("TS".parse(), Ok(TRANSCRIPT_STRAND));
        assert_eq!("U2".parse(), Ok(NEXT_HIT_QUALITY_SCORES));
        assert_eq!("UQ".parse(), Ok(SEGMENT_LIKELIHOOD));
        assert_eq!("ZN".parse(), Ok(Tag::Other(Other([b'Z', b'N']))));

        assert_eq!("".parse::<Tag>(), Err(ParseError::Empty));
        assert_eq!("R".parse::<Tag>(), Err(ParseError::InvalidLength(1)));
        assert_eq!("RGP".parse::<Tag>(), Err(ParseError::InvalidLength(3)));
        assert_eq!("1G".parse::<Tag>(), Err(ParseError::InvalidCharacter('1')));
        assert_eq!("R_".parse::<Tag>(), Err(ParseError::InvalidCharacter('_')));
    }
}
