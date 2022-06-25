// TODO: Maybe ChunkError or a better name like that?
#[derive(Debug)]
pub enum Error {
    InvalidChunkType,
    ChunkTooShort,
    ChunkLengthInvalid,
    BadChunkCrc,
}
