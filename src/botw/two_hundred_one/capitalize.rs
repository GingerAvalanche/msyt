use crate::Result;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Control201Capitalize {}

impl Control201Capitalize {
    pub(crate) fn parse(_: &Header, _: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Control201Capitalize { })
    }

    pub(crate) fn write(&self, _: &Header, _: &mut dyn Write) -> Result<()> {
        Ok(())
    }
}
