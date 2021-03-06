use crate::chunk_type::ChunkType;
use std::{
    convert::TryFrom,
    io::{BufReader, Read},
    string::FromUtf8Error,
};

#[derive(Debug)]
struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}
impl Chunk {
    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn calc_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let crc_data: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .copied()
            .chain(data.iter().copied())
            .collect();

        crc::crc32::checksum_ieee(&crc_data)
    }
}
impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buffer: [u8; 4] = [0; 4];
        let mut reader = BufReader::new(value);

        reader.read_exact(&mut buffer).unwrap();
        let length = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer).unwrap();
        let chunk_type = ChunkType::try_from(buffer).unwrap();

        let mut data = vec![0; length as usize];
        reader.read_exact(&mut data).unwrap();

        reader.read_exact(&mut buffer).unwrap();
        let crc = u32::from_be_bytes(buffer);
        let crc_check = Chunk::calc_crc(&chunk_type, &data);
        if crc_check != crc {
            return Err("CRC check failed");
        }

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    // #[test]
    // pub fn test_chunk_trait_impls() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656334;

    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();

    //     let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

    //     let _chunk_string = format!("{}", chunk);
    // }
}
