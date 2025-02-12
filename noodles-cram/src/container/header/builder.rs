use super::Header;
use crate::container::ReferenceSequenceContext;

#[derive(Debug, Default)]
pub struct Builder {
    reference_sequence_context: ReferenceSequenceContext,
    record_count: usize,
    record_counter: u64,
    base_count: u64,
    block_count: usize,
    landmarks: Vec<usize>,
}

impl Builder {
    pub fn set_reference_sequence_context(
        mut self,
        reference_sequence_context: ReferenceSequenceContext,
    ) -> Self {
        self.reference_sequence_context = reference_sequence_context;
        self
    }

    pub fn set_record_count(mut self, record_count: usize) -> Self {
        self.record_count = record_count;
        self
    }

    pub fn set_record_counter(mut self, record_counter: u64) -> Self {
        self.record_counter = record_counter;
        self
    }

    pub fn set_base_count(mut self, base_count: u64) -> Self {
        self.base_count = base_count;
        self
    }

    pub fn set_block_count(mut self, block_count: usize) -> Self {
        self.block_count = block_count;
        self
    }

    pub fn set_landmarks(mut self, landmarks: Vec<usize>) -> Self {
        self.landmarks = landmarks;
        self
    }

    pub fn build(self) -> Header {
        Header {
            reference_sequence_context: self.reference_sequence_context,
            record_count: self.record_count,
            record_counter: self.record_counter,
            base_count: self.base_count,
            block_count: self.block_count,
            landmarks: self.landmarks,
        }
    }
}
