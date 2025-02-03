use crate::Result;

use byteordered::Endian;

use anyhow::Context;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Control201Definite {}

impl Control201Definite {
    pub(crate) fn parse(header: &Header, mut reader: &mut Cursor<&[u8]>) -> Result<Self> {
        let len = header
            .endianness()
            .read_u16(&mut reader)
            .with_context(|| "could not read len")?;

        if len != 0 {
            return Err(anyhow::anyhow!("Info not length 4"));
        }

        Ok(Control201Definite { })
    }

    pub(crate) fn write(&self, header: &Header, mut writer: &mut dyn Write) -> Result<()> {
        header
            .endianness()
            .write_u16(&mut writer, 0)
            .with_context(|| "could not write len")?;

        Ok(())
    }
}
