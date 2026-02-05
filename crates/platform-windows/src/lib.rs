//! Windows Platform Adapter
//!
//! Proporciona acceso a APIs nativas de Windows (Win32).

use anyhow::{anyhow, Result};
use engine_core::MonitorInfo;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use tracing::{debug, info};
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;

/// Enumerar todos los monitores conectados
pub fn enumerate_monitors() -> Result<Vec<MonitorInfo>> {
    info!("Enumerating monitors...");

    let mut monitors = Vec::new();

    // Callback para each monitor
    unsafe {
        let success = EnumDisplayMonitors(
            None,
            None,
            Some(enum_monitors_callback),
            LPARAM(&mut monitors as *mut Vec<MonitorInfo> as isize),
        );

        if !success.as_bool() {
            return Err(anyhow!("EnumDisplayMonitors failed"));
        }
    }

    debug!("Found {} monitors", monitors.len());
    Ok(monitors)
}

/// Callback para EnumDisplayMonitors
unsafe extern "system" fn enum_monitors_callback(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let monitors: &mut Vec<MonitorInfo> =
        &mut *(lparam.0 as *mut Vec<MonitorInfo>);

    if let Ok(monitor_info) = get_monitor_info(hmonitor) {
        monitors.push(monitor_info);
    }

    BOOL::from(true)
}

/// Obtener información de un monitor
unsafe fn get_monitor_info(hmonitor: HMONITOR) -> Result<MonitorInfo> {
    let mut monitorinfo = MONITORINFOEXW {
        monitorInfo: MONITORINFO::default(),
        szDevice: [0; 32],
    };
    monitorinfo.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

    let success = GetMonitorInfoW(hmonitor, &mut monitorinfo.monitorInfo as *mut _ as *mut _);
    if !success.as_bool() {
        return Err(anyhow!("GetMonitorInfoW failed"));
    }

    // Obtener nombre del device
    let device_name = windows_string_to_rust(&monitorinfo.szDevice);

    // Obtener DPI
    let dpi = get_monitor_dpi(hmonitor).unwrap_or(96);

    // Obtener posición y resolución
    let rect = &monitorinfo.monitorInfo.rcMonitor;
    let x = rect.left;
    let y = rect.top;
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;

    // ¿Es primary?
    let is_primary = (monitorinfo.monitorInfo.dwFlags & MONITORINFOF_PRIMARY.0) != 0;

    // Generar ID único
    let id = format!("monitor_{}", device_name.replace("\\", "_"));

    debug!(
        "Monitor: id={}, name={}, dpi={}, pos=({},{}), size={}x{}",
        id, device_name, dpi, x, y, width, height
    );

    Ok(MonitorInfo {
        id,
        name: device_name,
        dpi,
        x,
        y,
        width,
        height,
        is_primary,
    })
}

/// Obtener DPI de un monitor
unsafe fn get_monitor_dpi(hmonitor: HMONITOR) -> Result<u32> {
    let mut dpi_x: u32 = 96;
    let mut dpi_y: u32 = 96;

    // Intentar GetDpiForMonitor (Windows 8.1+)
    use windows::Win32::System::Com::GetDpiForMonitor;
    use windows::Win32::System::Com::MDT_EFFECTIVE_DPI;

    let result = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);
    if result.is_ok() {
        return Ok(dpi_x);
    }

    // Fallback a GetDC + GetDeviceCaps
    let hdc = GetDC(None);
    if hdc.is_invalid() {
        return Ok(96); // Default DPI
    }

    let dpi = GetDeviceCaps(hdc, LOGPIXELSX);
    let _ = ReleaseDC(None, hdc);

    Ok(dpi as u32)
}

/// Convertir Windows string (UTF-16) a Rust String
fn windows_string_to_rust(wide: &[u16]) -> String {
    let null_pos = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    String::from_utf16_lossy(&wide[..null_pos]).to_string()
}

/// Obtener monitor primario
pub fn get_primary_monitor() -> Result<Option<MonitorInfo>> {
    let monitors = enumerate_monitors()?;
    Ok(monitors.into_iter().find(|m| m.is_primary))
}

pub mod monitor;
pub use monitor::MonitorExt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_monitors() {
        // Este test requiere que se ejecute en Windows
        #[cfg(target_os = "windows")]
        {
            let result = enumerate_monitors();
            assert!(result.is_ok());
            let monitors = result.unwrap();
            assert!(!monitors.is_empty(), "At least one monitor should be detected");
        }
    }

    #[test]
    fn test_get_primary_monitor() {
        #[cfg(target_os = "windows")]
        {
            let result = get_primary_monitor();
            assert!(result.is_ok());
            if let Ok(Some(monitor)) = result {
                assert!(monitor.is_primary);
                assert!(!monitor.id.is_empty());
            }
        }
    }
}
