use std::vec;

use futures::{Stream, stream};
use noodles_bgzf as bgzf;
use noodles_core::region::Interval;
use noodles_csi::binning_index::index::reference_sequence::bin::Chunk;
use tokio::io::{self, AsyncRead, AsyncSeek};

use super::Reader;
use crate::{Record, io::reader::query::intersects};

enum State {
    Seek,
    Read(bgzf::VirtualPosition),
    Done,
}

struct Context<'a, R>
where
    R: AsyncRead + AsyncSeek,
{
    reader: &'a mut Reader<bgzf::r#async::io::Reader<R>>,
    chunks: vec::IntoIter<Chunk>,

    reference_sequence_id: usize,
    interval: Interval,

    state: State,
}

pub fn query<R>(
    reader: &mut Reader<bgzf::r#async::io::Reader<R>>,
    chunks: Vec<Chunk>,
    reference_sequence_id: usize,
    interval: Interval,
) -> impl Stream<Item = io::Result<Record>> + '_
where
    R: AsyncRead + AsyncSeek + Unpin,
{
    let ctx = Context {
        reader,
        chunks: chunks.into_iter(),

        reference_sequence_id,
        interval,

        state: State::Seek,
    };

    Box::pin(stream::try_unfold(ctx, |mut ctx| async {
        loop {
            match ctx.state {
                State::Seek => {
                    ctx.state = match ctx.chunks.next() {
                        Some(chunk) => {
                            ctx.reader.get_mut().seek(chunk.start()).await?;
                            State::Read(chunk.end())
                        }
                        None => State::Done,
                    };
                }
                State::Read(chunk_end) => match next_record(ctx.reader).await? {
                    Some(record) => {
                        if ctx.reader.get_ref().virtual_position() >= chunk_end {
                            ctx.state = State::Seek;
                        }

                        if intersects(&record, ctx.reference_sequence_id, ctx.interval)? {
                            return Ok(Some((record, ctx)));
                        }
                    }
                    None => ctx.state = State::Seek,
                },
                State::Done => return Ok(None),
            }
        }
    }))
}

async fn next_record<R>(
    reader: &mut Reader<bgzf::r#async::io::Reader<R>>,
) -> io::Result<Option<Record>>
where
    R: AsyncRead + AsyncSeek + Unpin,
{
    let mut record = Record::default();

    reader.read_record(&mut record).await.map(|n| match n {
        0 => None,
        _ => Some(record),
    })
}
