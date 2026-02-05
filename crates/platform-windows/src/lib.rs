// platform-windows skeleton
use anyhow::Result;
pub struct MonitorInfo {
    pub id: String,
    pub name: String,
    pub dpi: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
pub fn enumerate_monitors() -> Result<Vec<MonitorInfo>> {
    Ok(Vec::new())
}
