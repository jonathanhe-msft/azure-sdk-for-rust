#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-04-01")]
pub mod package_2021_04_01;
#[cfg(all(feature = "package-2021-04-01", not(feature = "no-default-tag")))]
pub use package_2021_04_01::*;
#[cfg(feature = "package-2020-05-01-preview")]
pub mod package_2020_05_01_preview;
#[cfg(all(feature = "package-2020-05-01-preview", not(feature = "no-default-tag")))]
pub use package_2020_05_01_preview::*;
#[cfg(feature = "package-2019-01-01-preview")]
pub mod package_2019_01_01_preview;
#[cfg(all(feature = "package-2019-01-01-preview", not(feature = "no-default-tag")))]
pub use package_2019_01_01_preview::*;
