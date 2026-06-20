pub mod error;
#[cfg(feature = "fabric")]
pub mod fabric;
#[cfg(feature = "forge")]
pub mod forge;
#[cfg(feature = "modrinth")]
pub mod modrinth;
#[cfg(feature = "mojang")]
pub mod mojang;
#[cfg(feature = "neoforge")]
pub mod neoforge;
#[cfg(feature = "paper")]
pub mod paper;
#[cfg(feature = "purpur")]
pub mod purpur;

pub mod ffi;
