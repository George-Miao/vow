use serde::{de::DeserializeOwned, Serialize};

use crate::{MaybeSend, VowResult};

pub trait ToFormat: MaybeSend {
    fn to_format(self) -> Format;
}

#[cfg(feature = "format-json")]
#[derive(Debug, Clone, Copy, Default)]
pub struct Json {
    pub pretty: bool,
}

#[cfg(feature = "format-toml")]
#[derive(Debug, Clone, Copy, Default)]
pub struct Toml {}

#[cfg(feature = "format-json")]
impl ToFormat for Json {
    fn to_format(self) -> Format {
        Format::Json { pretty: false }
    }
}

#[cfg(feature = "format-toml")]
impl ToFormat for Toml {
    fn to_format(self) -> Format {
        Format::Toml
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "format-json")] {
        pub type DefaultFormat = Json;
        impl Json {
            pub const fn default() -> Self {
                Self { pretty: false }
            }
        }
    } else if #[cfg(feature = "format-toml")] {
        pub type DefaultFormat = Toml;
        impl Toml {
            pub const fn default() -> Self {
                Self {}
            }
        }
    } else {
        compile_error!("No format feature enabled");
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Format {
    #[cfg(feature = "format-json")]
    Json { pretty: bool },

    #[cfg(feature = "format-toml")]
    Toml,
}

impl Format {
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn set_pretty(&mut self) {
        match self {
            #[cfg(feature = "format-json")]
            Self::Json { pretty } => *pretty = true,
            #[cfg(feature = "format-toml")]
            Self::Toml => (),
            #[allow(unreachable_patterns)]
            _ => todo!(),
        }
    }
}

pub fn des<T: DeserializeOwned>(format: Format, buf: &[u8]) -> VowResult<T> {
    let res = match format {
        #[cfg(feature = "format-json")]
        Format::Json { .. } => serde_json::from_slice(buf)?,
        #[cfg(feature = "format-toml")]
        Format::Toml { .. } => basic_toml::from_slice(buf)?,
    };
    Ok(res)
}

pub fn se<T: Serialize>(format: Format, writer: &mut Vec<u8>, value: &T) -> VowResult<()> {
    match format {
        #[cfg(feature = "format-json")]
        Format::Json { pretty } => {
            if pretty {
                serde_json::to_writer_pretty(writer, value)?;
            } else {
                serde_json::to_writer(writer, value)?;
            }
        }
        #[cfg(feature = "format-toml")]
        Format::Toml => {
            writer.extend_from_slice(basic_toml::to_string(value)?.as_bytes());
        }
    };
    Ok(())
}
