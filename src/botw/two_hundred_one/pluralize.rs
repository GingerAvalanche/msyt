use crate::{util, Result};

use byteordered::{byteorder::ReadBytesExt, Endian};

use msbt::{Encoding, Header};

use anyhow::Context;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Read, Write};

fn read_string(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<(usize, String)> {
    let str_len = match header.encoding() {
        Encoding::Utf16 => header
            .endianness()
            .read_u16(&mut reader)
            .with_context(|| "could not read str_len")?
            as usize,
        Encoding::Utf8 => reader
            .read_u8()
            .with_context(|| "could not read str_len")?
            as usize,
    };
    if str_len == 0 {
        return Ok((str_len, Default::default()));
    }

    let mut str_bytes = vec![0; str_len];
    reader
        .read_exact(&mut str_bytes)
        .with_context(|| "could not read string bytes")?;

    let string = match header.encoding() {
        Encoding::Utf16 => {
            let utf16_str: Vec<u16> = str_bytes
                .chunks(2)
                .map(|bs| header.endianness().read_u16(bs).map_err(Into::into))
                .collect::<Result<_>>()
                .with_context(|| "could not read u16s from string bytes")?;
            String::from_utf16(&utf16_str)
                .with_context(|| "could not parse utf-16 string")?
        }
        Encoding::Utf8 => String::from_utf8(if str_bytes.ends_with(&[0]) {
            str_bytes[..str_bytes.len() - 1].to_vec()
        } else {
            str_bytes
        })
        .with_context(|| "could not parse utf-8 string")?,
    };

    Ok((str_len + 2, string))
}

fn get_string_bytes(header: &Header, string: &str) -> Result<(usize, Vec<u8>)> {
    match header.encoding() {
        Encoding::Utf16 => {
            let mut buf = [0; 2];
            Ok((
                string.len() * 2 + 2,
                string.encode_utf16()
                    .flat_map(|x| {
                        header
                            .endianness()
                            .write_u16(&mut buf[..], x)
                            .expect("failed to write to array");
                        buf.to_vec()
                    })
                    .collect()
            ))
        }
        Encoding::Utf8 => Ok((string.len() + 1, util::strip_nul(string).as_bytes().to_vec())),
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Control201Pluralize {
    pub one: String,
    pub more: String,
    pub many: String,
}

impl Control201Pluralize {
    pub fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
        let len = header
            .endianness()
            .read_u16(&mut reader)
            .with_context(|| "could not read len")? as usize;

        let (one_size, one) = read_string(header, &mut reader)?;
        let (more_size, more) = read_string(header, &mut reader)?;
        let (many_size, many) = read_string(header, &mut reader)?;

        if one_size + more_size + many_size != len {
            Err(anyhow::anyhow!("Pluralize fields don't total param size"))
        }
        else {
            Ok(Control201Pluralize { one, more, many })
        }
    }

    pub fn write(&self, header: &Header, mut writer: &mut dyn Write) -> Result<()> {
        let mut encoded_strs = Vec::with_capacity(3);
        let (one_size, one_bytes) = get_string_bytes(header, &self.one)?;
        encoded_strs.push(one_bytes);
        let (more_size, more_bytes) = get_string_bytes(header, &self.more)?;
        encoded_strs.push(more_bytes);
        let (many_size, many_bytes) = get_string_bytes(header, &self.many)?;
        encoded_strs.push(many_bytes);

        let len = one_size + more_size + many_size;

        header
            .endianness()
            .write_u16(&mut writer, len as u16)
            .with_context(|| "could not write len")?;

        for s in encoded_strs {
            match header.encoding() {
                Encoding::Utf16 => header
                    .endianness()
                    .write_u16(&mut writer, s.len() as u16)
                    .with_context(|| "could not write str_len")?,
                Encoding::Utf8 => writer
                    .write_all(&[s.len() as u8])
                    .with_context(|| "could not write str_len")?,
            }
            writer
                .write_all(&s)
                .with_context(|| "could not write string")?;
        }

        Ok(())
    }
}
