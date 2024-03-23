use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt, LE};
use ico::IconDir;
use riff::{self, ChunkContents, ChunkId, LIST_ID, RIFF_ID};
use std::io::{self, Read, Seek, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error or system error")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Ani {
    pub header: AniHeader,
    pub frames: Vec<IconDir>,
}

#[derive(Default)]
pub struct AniHeader {
    /// The number of stored frames in this animation.
    pub num_frames: u32,
    /// The number of steps in this animation. Since the `seq` chunk is not implemented, it should
    /// be equal to `num_frames`.
    pub num_steps: u32,
    /// The width.
    pub width: u32,
    /// The height.
    pub height: u32,
    /// The number of jiffies (1/60 sec) that each frame displays.
    pub frame_rate: u32,
}
impl Ani {
    pub fn new() -> Self {
        Self::default()
    }
}
impl AniHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Ani {
    pub fn read<R: Read + Seek>(mut reader: R) -> io::Result<Self> {
        let chunk = riff::Chunk::read(&mut reader, 118)?;
        for child in chunk.iter(&mut reader) {
            if let Ok(chunk) = child {
                debug!(chunk); // Use the debug macro
            }
        }

        // RIFF_ID
        Ok(Self::new())
    }

    pub fn encode<T: Seek + Write>(&self, mut writer: T) -> Result<u64> {
        const fn chunk_id(value: &[u8; 4]) -> ChunkId {
            ChunkId { value: *value }
        }

        let contents = ChunkContents::Children(
            RIFF_ID.clone(),
            chunk_id(b"ACON"),
            vec![
                ChunkContents::Data(chunk_id(b"anih"), self.encode_header()?),
                ChunkContents::Children(LIST_ID.clone(), chunk_id(b"fram"), {
                    let mut chunks = Vec::new();
                    for cur in &self.frames {
                        let mut data = Vec::new();
                        cur.write(&mut data)?;
                        chunks.push(ChunkContents::Data(chunk_id(b"icon"), data));
                    }
                    chunks
                }),
            ],
        );

        contents.write(&mut writer).map_err(From::from)
    }

    fn encode_header(&self) -> Result<Vec<u8>> {
        // 4 (header size) + 32 (the rest)
        let mut data = Vec::with_capacity(36);

        data.write_u32::<LE>(36)?; // Header size

        data.write_u32::<LE>(self.header.num_frames)?;
        data.write_u32::<LE>(self.header.num_steps)?;
        data.write_u32::<LE>(self.header.width)?;
        data.write_u32::<LE>(self.header.height)?;
        data.write_u32::<LE>(32)?; // Color depth
        data.write_u32::<LE>(1)?; // Number of planes
        data.write_u32::<LE>(self.header.frame_rate)?;
        data.write_u32::<LE>(0b01)?; // Flags

        Ok(data)
    }
}
