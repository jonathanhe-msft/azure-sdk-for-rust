#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-06-01")]
pub mod package_2021_06_01;
#[cfg(all(feature = "package-2021-06-01", not(feature = "no-default-tag")))]
pub use package_2021_06_01::*;
#[cfg(feature = "package-2020-10-01")]
pub mod package_2020_10_01;
#[cfg(all(feature = "package-2020-10-01", not(feature = "no-default-tag")))]
pub use package_2020_10_01::*;
#[cfg(feature = "package-2018-09-01")]
pub mod package_2018_09_01;
#[cfg(all(feature = "package-2018-09-01", not(feature = "no-default-tag")))]
pub use package_2018_09_01::*;
