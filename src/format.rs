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

#[cfg(feature = "format-json")]
impl ToFormat for Json {
    fn to_format(self) -> Format {
        Format::Json { pretty: false }
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
    } else {
        compile_error!("No format feature enabled");
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Format {
    #[cfg(feature = "format-json")]
    Json { pretty: bool },
}

impl Format {
    pub fn set_pretty(&mut self) {
        match self {
            #[cfg(feature = "format-json")]
            Self::Json { pretty } => *pretty = true,
            #[allow(unreachable_patterns)]
            _ => todo!(),
        }
    }
}

pub fn des<T: DeserializeOwned>(format: Format, buf: &[u8]) -> VowResult<T> {
    let res = match format {
        #[cfg(feature = "format-json")]
        Format::Json { .. } => serde_json::from_slice(buf)?,
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
    };
    Ok(())
}
