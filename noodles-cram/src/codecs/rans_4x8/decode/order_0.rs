use std::io::{self, Read};

use byteorder::ReadBytesExt;

use super::{rans_advance_step, rans_get_cumulative_freq, rans_renorm, read_states};
use crate::{codecs::rans_4x8::ALPHABET_SIZE, io::reader::num::read_itf8_as};

type Frequencies = [u16; ALPHABET_SIZE]; // F
type CumulativeFrequencies = Frequencies; // C
type CumulativeFrequenciesSymbolsTable = [u8; 4096];

pub fn decode<R>(reader: &mut R, dst: &mut [u8]) -> io::Result<()>
where
    R: Read,
{
    let freqs = read_frequencies_0(reader)?;
    let cumulative_frequencies = build_cumulative_frequencies(&freqs);

    let cumulative_freqs_symbols_table =
        build_cumulative_freqs_symbols_table_0(&cumulative_frequencies);

    let mut states = read_states(reader)?;

    for chunk in dst.chunks_mut(states.len()) {
        for (d, state) in chunk.iter_mut().zip(states.iter_mut()) {
            let f = rans_get_cumulative_freq(*state);
            let s = cumulative_freqs_symbols_table[f as usize];

            *d = s;

            *state = rans_advance_step(
                *state,
                freqs[usize::from(s)],
                cumulative_frequencies[usize::from(s)],
            );

            *state = rans_renorm(reader, *state)?;
        }
    }

    Ok(())
}

pub fn read_frequencies_0<R>(reader: &mut R) -> io::Result<Frequencies>
where
    R: Read,
{
    const NUL: u8 = 0x00;

    let mut frequencies = [0; ALPHABET_SIZE];

    let mut sym = reader.read_u8()?;
    let mut prev_sym = sym;

    loop {
        let f = read_itf8_as(reader)?;
        frequencies[usize::from(sym)] = f;

        sym = reader.read_u8()?;

        if sym == NUL {
            break;
        }

        if sym - 1 == prev_sym {
            let len = reader.read_u8()?;

            for _ in 0..len {
                let f = read_itf8_as(reader)?;
                frequencies[usize::from(sym)] = f;
                sym += 1;
            }
        }

        prev_sym = sym;
    }

    Ok(frequencies)
}

pub fn build_cumulative_frequencies(frequencies: &Frequencies) -> CumulativeFrequencies {
    let mut cumulative_frequencies = [0; ALPHABET_SIZE];

    let mut f = cumulative_frequencies[0];

    for (next_f, g) in cumulative_frequencies[1..].iter_mut().zip(frequencies) {
        *next_f = f + g;
        f = *next_f;
    }

    cumulative_frequencies
}

pub fn build_cumulative_freqs_symbols_table_0(
    cumulative_freqs: &CumulativeFrequencies,
) -> CumulativeFrequenciesSymbolsTable {
    let mut table = [0; 4096];
    let mut sym = 0;

    for (f, g) in (0u16..).zip(&mut table) {
        while sym < u8::MAX && f >= cumulative_freqs[usize::from(sym + 1)] {
            sym += 1;
        }

        *g = sym;
    }

    table
}
