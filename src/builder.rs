use std::{
    io,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{format::DefaultFormat, Io, Vow, VowAsync, VowFile, VowFileAsync, VowResult};
use crate::{
    format::{Json, ToFormat},
    marker::{Async, Blocking, Just, Nothing, ToOption},
};

/// Builder for [`Vow`].
///
/// By default, it will:
///
/// - use `JSON` format without pretty printing.
/// - not overwrite the file if it already exists and has valid content.
/// - not fail on (deny) invalid content (instead, overwrite it silently).
///
/// It is usually recommended to use [`Vow::builder`] or [`Vow::open`] and its async counterparts to create a new builder.
///
/// # Example
///
/// ```no_run
/// use vow::VowBuilder;
/// use serde::{Deserialize, Serialize};
/// use std::fs::File;
///
/// #[derive(Serialize, Deserialize)]
/// struct Data {
///    value: i32,
/// }
///
/// let file = File::create("data.json").unwrap();
/// let vow = VowBuilder::new(file)
///     .json()
///     .pretty()
///     .default(Data { value: 42 })
///     .deny_invalid()
///     .overwrite_local()
///     .build();
/// ```
pub struct VowBuilder<T, F, A, Fo> {
    file: FileBuilder<F, A>,
    default: T,
    overwrite: bool,
    deny_invalid: bool,
    format: Fo,
}

enum FileBuilder<F, A> {
    File(F),
    Path(PathBuf, PhantomData<A>),
}

impl<F: VowFileAsync> FileBuilder<F, Async> {
    async fn open(self) -> io::Result<F> {
        match self {
            Self::File(file) => Ok(file),
            Self::Path(path, _) => F::open(&path).await,
        }
    }
}

impl<F: VowFile> FileBuilder<F, Blocking> {
    fn open(self) -> io::Result<F> {
        match self {
            Self::File(file) => Ok(file),
            Self::Path(path, _) => F::open(&path),
        }
    }
}

impl<T, F> VowBuilder<Nothing<T>, F, Blocking, DefaultFormat> {
    /// Create a new blocking builder.
    pub const fn new(file: F) -> Self {
        Self {
            file: FileBuilder::File(file),
            default: Nothing(PhantomData),
            overwrite: false,
            deny_invalid: false,
            format: DefaultFormat::default(),
        }
    }

    /// Open a blocking file at the given path.
    pub fn open(path: impl AsRef<Path>) -> Self {
        Self {
            file: FileBuilder::Path(path.as_ref().to_path_buf(), PhantomData),
            default: Nothing(PhantomData),
            overwrite: false,
            deny_invalid: false,
            format: DefaultFormat::default(),
        }
    }
}

impl<T, F> VowBuilder<Nothing<T>, F, Async, DefaultFormat> {
    /// Create a new async builder.
    pub const fn new(file: F) -> Self {
        Self {
            file: FileBuilder::File(file),
            default: Nothing(PhantomData),
            overwrite: false,
            deny_invalid: false,
            format: DefaultFormat::default(),
        }
    }

    /// Open a blocking file at the given path.
    pub fn open(path: impl AsRef<Path>) -> Self {
        Self {
            file: FileBuilder::Path(path.as_ref().to_path_buf(), PhantomData),
            default: Nothing(PhantomData),
            overwrite: false,
            deny_invalid: false,
            format: DefaultFormat::default(),
        }
    }
}

impl<T, F, A> VowBuilder<Nothing<T>, F, A, DefaultFormat> {
    /// Set the type of the value but not provide any default value.
    ///
    /// This is useful when you know there's existsing value stored in file and its type, so
    /// we don't need to provide a default value.
    pub fn with_type<U>(self) -> VowBuilder<Nothing<U>, F, A, DefaultFormat> {
        VowBuilder {
            file: self.file,
            default: Nothing(PhantomData),
            overwrite: self.overwrite,
            deny_invalid: self.deny_invalid,
            format: self.format,
        }
    }

    /// Set the default value in case the file is empty or has invalid content.
    pub fn default<U>(self, value: U) -> VowBuilder<Just<U>, F, A, DefaultFormat> {
        VowBuilder {
            default: Just(value),
            file: self.file,
            overwrite: self.overwrite,
            deny_invalid: self.deny_invalid,
            format: self.format,
        }
    }
}

impl<T, F, A> VowBuilder<T, F, A, DefaultFormat> {
    /// When set `true`, [`Vow`] will fail to build if the file already exists and has invalid content.
    #[must_use]
    pub const fn deny_invalid(mut self) -> Self {
        self.deny_invalid = true;
        self
    }

    /// Overwrite the file no matter whether it already exists or if it has valid content
    #[must_use]
    pub const fn overwrite_local(mut self) -> Self {
        self.overwrite = true;
        self
    }

    /// Keep the file content unless it doesn't exist or contains invalid content
    #[must_use]
    pub const fn keep_local(mut self) -> Self {
        self.overwrite = false;
        self
    }
}

impl<T, F, A> VowBuilder<T, F, A, Json> {
    /// Output the data in JSON format.
    #[must_use]
    #[cfg(feature = "format-json")]
    pub const fn json(mut self, pretty: bool) -> Self {
        self.format = Json { pretty };
        self
    }
}

impl<T, F, Fo> VowBuilder<T, F, Async, Fo>
where
    T: ToOption,
    F: VowFileAsync,
    T::Some: Serialize + DeserializeOwned,
    Fo: ToFormat,
{
    /// Build the [`VowAsync`] instance.
    pub async fn build(self) -> VowResult<VowAsync<T::Some, F>> {
        let mut io = Io::new(
            self.file.open().await?,
            self.format.to_format(),
            self.deny_invalid,
        );
        let default = self.default.maybe();
        let value = io.sync(default, self.overwrite).await?;

        Ok(VowAsync {
            value: Some(value),
            io,
        })
    }
}

impl<T, F, Fo> VowBuilder<T, F, Blocking, Fo>
where
    T: ToOption,
    F: VowFile,
    T::Some: Serialize + DeserializeOwned,
    Fo: ToFormat,
{
    /// Build the [`Vow`] instance.
    pub fn build(self) -> VowResult<Vow<T::Some, F>> {
        let mut io = Io::new(
            self.file.open()?,
            self.format.to_format(),
            self.deny_invalid,
        );
        let default = self.default.maybe();
        let value = io.sync(default, self.overwrite)?;

        Ok(Vow {
            value: Some(value),
            io,
        })
    }
}
