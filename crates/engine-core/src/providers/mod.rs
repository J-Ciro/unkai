// System providers module
pub mod audio;
pub mod battery;
pub mod memory;
pub mod network;

pub use audio::{AudioDevice, AudioProvider};
pub use battery::{BatteryProvider, BatteryState};
pub use memory::{MemoryProvider, MemoryStats};
pub use network::{NetworkInterface, NetworkProvider, NetworkTraffic};
