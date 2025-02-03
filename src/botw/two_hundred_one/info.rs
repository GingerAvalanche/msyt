use crate::Result;

use byteordered::{byteorder::{ReadBytesExt, WriteBytesExt}, Endian};

use anyhow::Context;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Control201Info {
    pub gender: u8,
    pub definite: u8,
    pub indefinite: u8,
    pub plural: u8,
}

impl Control201Info {
    pub(crate) fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
        let len = header
            .endianness()
            .read_u16(&mut reader)
            .with_context(|| "could not read len")?;

        if len != 4 {
            return Err(anyhow::anyhow!("Info not length 4"));
        }

        Ok(Control201Info { 
            gender: reader.read_u8()?,
            definite: reader.read_u8()?,
            indefinite: reader.read_u8()?,
            plural: reader.read_u8()?,
         })
    }

    pub(crate) fn write(&self, header: &Header, mut writer: &mut dyn Write) -> Result<()> {
        header
            .endianness()
            .write_u16(&mut writer, 4)
            .with_context(|| "could not write len")?;
        writer
            .write_u8(self.gender)
            .with_context(|| "could not write gender")?;
        writer
            .write_u8(self.definite)
            .with_context(|| "could not write definite")?;
        writer
            .write_u8(self.indefinite)
            .with_context(|| "could not write indefinite")?;
        writer
            .write_u8(self.plural)
            .with_context(|| "could not write plural")?;

        Ok(())
    }
}
