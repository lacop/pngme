use thiserror::Error;

// TODO: Maybe ChunkError or a better name like that?
#[derive(Debug, Error)]
pub enum Error {
    #[error("Chunk type is not valid")]
    InvalidChunkType,
    #[error("Chunk seems to be too short")]
    ChunkTooShort,
    #[error("Chunk length is not valid")]
    ChunkLengthInvalid,
    #[error("Bad CRC")]
    BadChunkCrc,
    #[error("Could not find chunk with given type")]
    NoSuchChunk,
    #[error("Does not start with the PNG header")]
    InvalidHeader,
    #[error("File too short")]
    FileTooShort,
}
