use crate::{
    botw::{Control, Localisation, MainControl, RawControl},
    Result,
};

use byteordered::Endian;

use anyhow::Context;

use msbt::Header;

use serde_derive::{Deserialize, Serialize};

use std::io::{Cursor, Write};

pub(crate) mod localisation;
pub(crate) mod one_field;
pub(crate) mod info;
pub(crate) mod definite;
pub(crate) mod indefinite;
pub(crate) mod capitalize;
pub(crate) mod downcase;
pub(crate) mod gender;
pub(crate) mod pluralize;
pub(crate) mod longvowel;

use self::{
    info::Control201Info,
    localisation::Control201Localisation,
    definite::Control201Definite,
    indefinite::Control201Indefinite,
    capitalize::Control201Capitalize,
    downcase::Control201Downcase,
    gender::Control201Gender,
    pluralize::Control201Pluralize,
    longvowel::Control201LongVowel,
};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Control201 {
    Info(u16, Control201Info),
    Definite(u16, Control201Definite),
    Indefinite(u16, Control201Indefinite),
    Capitalize(u16, Control201Capitalize),
    Downcase(u16, Control201Downcase),
    Gender(u16, Control201Gender),
    Pluralize(u16, Control201Pluralize),
    LongVowel(u16, Control201LongVowel),
}

impl MainControl for Control201 {
    fn marker(&self) -> u16 {
        201
    }

    fn parse(header: &Header, buf: &[u8]) -> Result<(usize, Control)> {
        let mut c = Cursor::new(buf);

        let kind = header.endianness().read_u16(&mut c)?;
        let control = match kind {
            0 => Control201::Info(
                kind,
                Control201Info::parse(header, &mut c)
                    .with_context(|| "could not parse control subtype dynamic")?,
            ),
            1 => Control201::Definite(kind, Control201Definite::parse(header, &mut c)
                .with_context(|| "could not parse Definite")?),
            2 => Control201::Indefinite(kind, Control201Indefinite::parse(header, &mut c)
                .with_context(|| "could not parse Indefinite")?),
            3 => Control201::Capitalize(kind, Control201Capitalize::parse(header, &mut c)
                .with_context(|| "could not parse Definite")?),
            4 => Control201::Downcase(kind, Control201Downcase::parse(header, &mut c)
                .with_context(|| "could not parse Indefinite")?),
            5 => Control201::Gender(kind, Control201Gender::parse(header, &mut c)
                .with_context(|| "could not parse Gender fields")?),
            6 => Control201::Pluralize(kind, Control201Pluralize::parse(header, &mut c)
                .with_context(|| "could not parse Pluralize fields")?),
            7..=8 => {
                let longvowel_kind = Localisation::from_u16(kind);
                let sub = Control201LongVowel::parse(header, &mut c)
                    .with_context(|| "could not parse control subtype longvowel")?;
                return Ok((
                    c.position() as usize,
                    Control::LongVowel {
                        longvowel_kind,
                        options: sub.strings
                    }
                ));
            },
            5..=8 => {
                let localisation_kind = Localisation::from_u16(kind);
                let sub = Control201Localisation::parse(header, &mut c)
                    .with_context(|| "could not parse control subtype localisation")?;
                return Ok((
                    c.position() as usize,
                    Control::Localisation {
                        localisation_kind,
                        options: sub.strings,
                    },
                ));
            }
            x => anyhow::bail!("unknown control 201 type: {}", x),
        };

        Ok((
            c.position() as usize,
            Control::Raw(RawControl::TwoHundredOne(control)),
        ))
    }

    fn write(&self, header: &Header, mut writer: &mut dyn Write) -> Result<()> {
        match *self {
            Control201::Info(marker, ref control) => {
                header
                    .endianness()
                    .write_u16(&mut writer, marker)
                    .with_context(|| format!("could not write marker for subtype {}", marker))?;
                control
                    .write(header, &mut writer)
                    .with_context(|| format!("could not write subtype {}", marker))
                    .map_err(Into::into)
            },
            Control201::Definite(marker, ref control) => {
                header
                    .endianness()
                    .write_u16(&mut writer, marker)
                    .with_context(|| format!("could not write marker for subtype {}", marker))?;
                control
                    .write(header, &mut writer)
                    .with_context(|| format!("could not write subtype {}", marker))
                    .map_err(Into::into)
            },
            Control201::Indefinite(marker, ref control) => {
                header
                    .endianness()
                    .write_u16(&mut writer, marker)
                    .with_context(|| format!("could not write marker for subtype {}", marker))?;
                control
                    .write(header, &mut writer)
                    .with_context(|| format!("could not write subtype {}", marker))
                    .map_err(Into::into)
            },
            Control201::Capitalize(marker, ref control) => {
                header
                    .endianness()
                    .write_u16(&mut writer, marker)
                    .with_context(|| format!("could not write marker for subtype {}", marker))?;
                control
                    .write(header, &mut writer)
                    .with_context(|| format!("could not write subtype {}", marker))
                    .map_err(Into::into)
            },
            Control201::Downcase(marker, ref control) => {
                header
                    .endianness()
                    .write_u16(&mut writer, marker)
                    .with_context(|| format!("could not write marker for subtype {}", marker))?;
                control
                    .write(header, &mut writer)
                    .with_context(|| format!("could not write subtype {}", marker))
                    .map_err(Into::into)
            },
            Control201::Gender(marker, ref control) => {
                header
                    .endianness()
                    .write_u16(&mut writer, marker)
                    .with_context(|| format!("could not write marker for subtype {}", marker))?;
                control
                    .write(header, &mut writer)
                    .with_context(|| format!("could not write subtype {}", marker))
                    .map_err(Into::into)
            },
            Control201::Pluralize(marker, ref control) => {
                header
                    .endianness()
                    .write_u16(&mut writer, marker)
                    .with_context(|| format!("could not write marker for subtype {}", marker))?;
                control
                    .write(header, &mut writer)
                    .with_context(|| format!("could not write subtype {}", marker))
                    .map_err(Into::into)
            },
            Control201::Localisation(kind, ref c) => {
                header
                    .endianness()
                    .write_u16(&mut writer, kind.as_u16())
                    .with_context(|| {
                        format!("could not write control subtype marker {}", kind.as_u16())
                    })?;
                c.write(header, &mut writer)
                    .with_context(|| format!("could not write control subtype {}", kind.as_u16()))
                    .map_err(Into::into)
            }
        }
    }
}
