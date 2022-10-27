use std::fs::OpenOptions;
use std::io::{BufReader, SeekFrom, Seek, Read, BufWriter, Write};
use std::{collections::HashMap, fs::File, io};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;
use serde_derive::{Serialize, Deserialize};

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

macro_rules! debug {
    () => {
        if (cfg!(debug_assertions)) { print!("\n") }
    };
    ($($arg:tt)*) => {{
        if (cfg!(debug_assertions)) {
            println!("{}", format_args!($($arg)*))
        }
    }};
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(path)?;
        debug!("file obj: {:#?}", f);
        let index = HashMap::new();
        Ok(ActionKV { f, index })
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            debug!("load: position={}", position);
            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => {
                            break;
                        },
                        _ => return Err(err),
                    }
                }
            };
            self.index.insert(kv.key, position);
        }
        Ok(())
    }

    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        self.f.seek(SeekFrom::End(0))
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position,
        };
        let kv = self.get_at(position)?;
        Ok(Some(kv.value))
    }

    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;
        Ok(kv)
    }

    pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);
        let mut found: Option<(u64, ByteString)> = None;
        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => { break; },
                        _ => {
                            debug!("{:#?}", err);
                            return Err(err)
                        },
                    }
                }
            };
            if kv.key == target {
                found = Some((position, kv.value));
            }
        }
        Ok(found)
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut f = BufWriter::new(&mut self.f);
        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + val_len);

        for byte in key   { tmp.push(*byte); }
        for byte in value { tmp.push(*byte); }

        let checksum = crc32::checksum_ieee(&tmp);

        let next_byte = SeekFrom::End(0);
        let curr_position = f.seek(SeekFrom::Current(0))?;
        f.seek(next_byte)?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&tmp)?;

        Ok(curr_position)
    }

    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }

    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        debug!("record: crc={:04x}", saved_checksum);

        let key_len = f.read_u32::<LittleEndian>()?;
        let val_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;
        let mut data = ByteString::with_capacity(data_len as usize);

        {
            f.by_ref()
            .take(data_len as u64)
            .read_to_end(&mut data)?;
        }

        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = crc32::checksum_ieee(&data);

        if checksum != saved_checksum {
            let error_msg = format!("data corruption encountered ({:08x} != {:08x})", checksum, saved_checksum);
            Err(io::Error::new(io::ErrorKind::InvalidData, error_msg))
        } else {
            let value = data.split_off(key_len as usize);
            let key = data;
            Ok(KeyValuePair { key, value })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let path = Path::new("/tmp/empty.kv");

        let kv = ActionKV::open(&path);

        assert!(kv.is_ok());
        assert_eq!(kv.unwrap().index.len(), 0);
    }

    #[test]
    fn test_load() -> Result<(), std::io::Error> {
        let path = Path::new("/tmp/some.kv");
        {
            // should close file before reading in the store
            let file = File::create(&path)?;
            let mut buf = BufWriter::new(file);
            buf.write_u32::<LittleEndian>(crc32::checksum_ieee(&[0xAA, 0xBB]))?;
            buf.write_u32::<LittleEndian>(0x01)?;
            buf.write_u32::<LittleEndian>(0x01)?;
            buf.write_u8(0xAA)?;
            buf.write_u8(0xBB)?;
        }

        let mut kv = ActionKV::open(&path)?;
        kv.load()?;
        assert_eq!(kv.index.len(), 1);

        Ok(())
    }
}
