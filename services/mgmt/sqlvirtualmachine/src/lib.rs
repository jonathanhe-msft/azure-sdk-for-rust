#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-07")]
pub mod package_preview_2022_07;
#[cfg(all(feature = "package-preview-2022-07", not(feature = "no-default-tag")))]
pub use package_preview_2022_07::*;
#[cfg(feature = "package-preview-2022-02")]
pub mod package_preview_2022_02;
#[cfg(all(feature = "package-preview-2022-02", not(feature = "no-default-tag")))]
pub use package_preview_2022_02::*;
#[cfg(feature = "package-preview-2021-11")]
pub mod package_preview_2021_11;
#[cfg(all(feature = "package-preview-2021-11", not(feature = "no-default-tag")))]
pub use package_preview_2021_11::*;
#[cfg(feature = "package-2022-02")]
pub mod package_2022_02;
#[cfg(all(feature = "package-2022-02", not(feature = "no-default-tag")))]
pub use package_2022_02::*;
#[cfg(feature = "package-2017-03-01-preview")]
pub mod package_2017_03_01_preview;
#[cfg(all(feature = "package-2017-03-01-preview", not(feature = "no-default-tag")))]
pub use package_2017_03_01_preview::*;
