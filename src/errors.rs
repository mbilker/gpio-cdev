// Copyright (c) 2018 The rust-gpio-cdev Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::convert::From;
use std::io;
use std::path::PathBuf;
use nix;

pub type Result<T> = ::std::result::Result<T, Error>;

#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into());
    };
}

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    #[snafu(display("{}", source))]
    Io { source: io::Error },

    #[snafu(display("{}", source))]
    Nix { source: nix::Error },

    #[snafu(display("Offset out of range"))]
    OffsetOutOfRange,

    #[snafu(display("unable to read /dev directory"))]
    ReadDevDirectory { source: io::Error },

    #[snafu(display("unable to open chip at path {}", path.display()))]
    OpenChip {
        source: io::Error,
        path: PathBuf,
    },
    #[snafu(display("unable to get chip info for {}", path.display()))]
    GetChipInfo {
        source: nix::Error,
        path: PathBuf,
    },

    LineeventIoctl { source: nix::Error },
    LineinfoIoctl { source: nix::Error },
    LinehandleRequestIoctl { source: nix::Error },

    GetLineValue { source: nix::Error },
    SetLineValue { source: nix::Error },
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Error {
        Error::Io { source }
    }
}

impl From<nix::Error> for Error {
    fn from(source: nix::Error) -> Error {
        Error::Nix { source }
    }
}
