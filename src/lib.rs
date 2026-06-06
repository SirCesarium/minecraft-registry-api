pub mod error;
#[cfg(feature = "modrinth")]
pub mod modrinth;
#[cfg(feature = "mojang")]
pub mod mojang;
#[cfg(feature = "paper")]
pub mod paper;
#[cfg(feature = "purpur")]
pub mod purpur;
#[cfg(feature = "fabric")]
pub mod fabric;
#[cfg(feature = "forge")]
pub mod forge;
#[cfg(feature = "neoforge")]
pub mod neoforge;

pub mod ffi;
