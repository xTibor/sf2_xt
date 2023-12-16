use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum RiffError {
    #[display(fmt = "Missing chunk")]
    MissingChunk,

    #[display(fmt = "Normal chunks cannot have subchunks")]
    NormalChunkNoSubchunks,

    #[display(fmt = "Container chunks cannot have data")]
    ContainerChunkNoData,

    #[display(fmt = "Truncated chunk data")]
    TruncatedChunkData,

    #[display(fmt = "Malformed identifier")]
    MalformedIdentifier,
}
