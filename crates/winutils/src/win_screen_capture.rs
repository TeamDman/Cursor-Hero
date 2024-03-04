#![allow(unused_imports)]
// most code from https://github.com/nashaofu/screenshots-rs/ commit 999faac06f85bd93638c2a9cda6cbb25ad9f5c73
// my changes are MPLv2, original code is Apache 2.0
// modifications aim to reduce redundant work for successive screen capture calls

// might also be interesting:
// https://github.com/rhinostream/win_desktop_duplication/tree/master
// https://github.com/rustdesk/rustdesk
// https://github.com/RustBuddies/desktop-sharing
// https://github.com/mira-screen-share/sharer/blob/main/src/capture/wgc/display.rs

#[cfg(target_arch = "x86")]
use std::arch::x86::_mm_shuffle_epi8;
use std::arch::x86_64::__m128i;
use std::arch::x86_64::_mm_loadu_si128;
use std::arch::x86_64::_mm_setr_epi8;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_mm_shuffle_epi8;
use std::arch::x86_64::_mm_storeu_si128;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;
use bevy::math::IRect;
use cursor_hero_bevy::prelude::LeftI;
use cursor_hero_bevy::prelude::TopI;
use cursor_hero_bevy::prelude::TopLeftI;
use cursor_hero_bevy::prelude::TranslateIVec2;
// use display_info::DisplayInfo;
// use fxhash::hash32;
use image::RgbaImage;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;
use widestring::U16CString;
use windows::core::PCWSTR;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::BitBlt;
use windows::Win32::Graphics::Gdi::CreateCompatibleBitmap;
use windows::Win32::Graphics::Gdi::CreateCompatibleDC;
use windows::Win32::Graphics::Gdi::CreateDCW;
use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::Graphics::Gdi::EnumDisplayMonitors;
use windows::Win32::Graphics::Gdi::GetDIBits;
use windows::Win32::Graphics::Gdi::GetMonitorInfoW;
use windows::Win32::Graphics::Gdi::GetObjectW;
use windows::Win32::Graphics::Gdi::SelectObject;
use windows::Win32::Graphics::Gdi::SetStretchBltMode;
use windows::Win32::Graphics::Gdi::StretchBlt;
use windows::Win32::Graphics::Gdi::BITMAP;
use windows::Win32::Graphics::Gdi::BITMAPINFO;
use windows::Win32::Graphics::Gdi::BITMAPINFOHEADER;
use windows::Win32::Graphics::Gdi::DIB_RGB_COLORS;
use windows::Win32::Graphics::Gdi::HBITMAP;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::Graphics::Gdi::MONITORINFOEXW;
use windows::Win32::Graphics::Gdi::RGBQUAD;
use windows::Win32::Graphics::Gdi::SRCCOPY;
use windows::Win32::Graphics::Gdi::STRETCH_HALFTONE;

use cursor_hero_metrics::Metrics;

use crate::ToBevyIRect;

//////////////////////
/// GET MONITOR INFOS
//////////////////////

#[derive(Debug)]
pub struct MonitorInfo {
    pub id: u32,
    pub name: String,
    pub rect: IRect,
    pub work_area: IRect, // the area of the monitor not covered by the taskbar
    pub is_primary: bool,
}

pub fn get_monitor_infos() -> Result<Vec<MonitorInfo>> {
    // box it up so we can pass it to the callback
    let results: *mut Vec<MONITORINFOEXW> = Box::into_raw(Box::default());

    // use proc method to iterate monitors and collect into results vec
    unsafe {
        EnumDisplayMonitors(
            HDC::default(),
            None,
            Some(monitor_enum_proc),
            LPARAM(results as isize),
        )
        .ok()?;
    };

    // convert results back into a vec
    let results = unsafe { &Box::from_raw(results) };

    // convert vec of MONITORINFOEXW into vec of MonitorInfo
    let results = results
        .iter()
        .map(|info| {
            let sz_device_ptr = info.szDevice.as_ptr();
            let sz_device_string =
                unsafe { U16CString::from_ptr_str(sz_device_ptr).to_string_lossy() };
            MonitorInfo {
                id: fxhash::hash32(sz_device_string.as_bytes()), // same algorithm as screen crate
                name: sz_device_string,
                rect: info.monitorInfo.rcMonitor.to_bevy_irect(),
                work_area: info.monitorInfo.rcWork.to_bevy_irect(),
                is_primary: info.monitorInfo.dwFlags == 1,
            }
        })
        .collect::<Vec<MonitorInfo>>();
    Ok(results)
}

extern "system" fn monitor_enum_proc(
    h_monitor: HMONITOR,
    _: HDC,
    _: *mut RECT,
    data: LPARAM,
) -> BOOL {
    let results = unsafe { Box::from_raw(data.0 as *mut Vec<MONITORINFOEXW>) };
    let results = Box::leak(results);

    match get_monitor_info_exw(h_monitor) {
        Ok(monitor_info_exw) => {
            results.push(monitor_info_exw);
            BOOL::from(true)
        }
        Err(_) => BOOL::from(false),
    }
}

fn get_monitor_info_exw(h_monitor: HMONITOR) -> Result<MONITORINFOEXW> {
    let mut monitor_info_exw: MONITORINFOEXW = unsafe { mem::zeroed() };
    monitor_info_exw.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
    let monitor_info_exw_ptr = <*mut _>::cast(&mut monitor_info_exw);

    unsafe {
        GetMonitorInfoW(h_monitor, monitor_info_exw_ptr).ok()?;
    };
    Ok(monitor_info_exw)
}

//////////////////
/// GET MONITORS
//////////////////
pub struct Monitor {
    pub info: MonitorInfo,
    device_context: HDC,
}

pub fn get_all_monitors() -> Result<Vec<Monitor>> {
    let monitor_infos = get_monitor_infos()?;
    let mut monitors = Vec::new();

    for monitor_info in monitor_infos {
        // intermediate variables are required to ensure the pointer contents remain in scope
        let a = U16CString::from_str(&monitor_info.name)?;
        let b = a.as_ptr();
        let name_pcwstr = PCWSTR(b);
        let device_context =
            unsafe { CreateDCW(name_pcwstr, name_pcwstr, PCWSTR(ptr::null()), None) };

        monitors.push(Monitor {
            info: monitor_info,
            device_context,
        });
    }

    Ok(monitors)
}

/////////////////////////////
/// MONITOR REGION CAPTURER
/////////////////////////////

pub struct MonitorRegionCapturer {
    pub monitor: Arc<Monitor>,
    pub capture_region: IRect,
    device_context: HDC,
    bitmap: HBITMAP,
}

pub fn get_full_monitor_capturers() -> Result<Vec<MonitorRegionCapturer>> {
    let monitors = get_all_monitors()?;
    let mut capturers = Vec::new();

    for monitor in monitors {
        let region = monitor.info.rect;
        let capturer = get_monitor_capturer(Arc::new(monitor), region);
        capturers.push(capturer);
    }

    Ok(capturers)
}

pub fn get_monitor_capturer(monitor: Arc<Monitor>, capture_region: IRect) -> MonitorRegionCapturer {
    let capture_device_context = unsafe { CreateCompatibleDC(monitor.device_context) };
    let bitmap = unsafe {
        CreateCompatibleBitmap(
            monitor.device_context,
            capture_region.width(),
            capture_region.height(),
        )
    };

    unsafe {
        SelectObject(capture_device_context, bitmap);
        SetStretchBltMode(monitor.device_context, STRETCH_HALFTONE);
    };

    MonitorRegionCapturer {
        monitor,
        device_context: capture_device_context,
        bitmap,
        capture_region,
    }
}

impl Drop for MonitorRegionCapturer {
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.bitmap);
            DeleteDC(self.device_context);
        }
    }
}
impl MonitorRegionCapturer {
    // pub fn capture(&self) -> Result<RgbaImage> {
    pub fn capture(&self, metrics: &mut Option<Metrics>) -> Result<RgbaImage> {
        // todo: try https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgioutputduplication-acquirenextframe
        unsafe {
            if let Some(metrics) = metrics {
                metrics.begin("blit");
            }
            StretchBlt(
                self.device_context,
                0,
                0,
                self.capture_region.width(),
                self.capture_region.height(),
                self.monitor.device_context,
                self.monitor.info.rect.left() - self.capture_region.left(),
                self.monitor.info.rect.top() - self.capture_region.top(),
                self.capture_region.width(),
                self.capture_region.height(),
                SRCCOPY,
            )
            .ok()?;
            if let Some(metrics) = metrics {
                metrics.end("blit");
            }
        };

        let mut bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: self.capture_region.width(),
                biHeight: self.capture_region.height(), // Here you can pass a negative number, but don't know why it will throw an error
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD::default(); 1],
        };

        let data =
            vec![0u8; (self.capture_region.width() * self.capture_region.height()) as usize * 4];
        let buf_prt = data.as_ptr() as *mut _;

        if let Some(metrics) = metrics {
            metrics.begin("getdibits");
        }
        let is_success = unsafe {
            GetDIBits(
                self.device_context,
                self.bitmap,
                0,
                self.capture_region.height() as u32,
                Some(buf_prt),
                &mut bitmap_info,
                DIB_RGB_COLORS,
            ) == 0
        };
        if let Some(metrics) = metrics {
            metrics.end("getdibits");
        }

        if is_success {
            return Err(anyhow!("Get RGBA data failed"));
        }

        let mut bitmap = BITMAP::default();
        let bitmap_ptr = <*mut _>::cast(&mut bitmap);

        if let Some(metrics) = metrics {
            metrics.begin("getobject");
        }
        unsafe {
            // Get the BITMAP from the HBITMAP.
            GetObjectW(
                self.bitmap,
                mem::size_of::<BITMAP>() as i32,
                Some(bitmap_ptr),
            );
        }
        if let Some(metrics) = metrics {
            metrics.end("getobject");
        }

        // Rotate the image; the image data is inverted.
        if let Some(metrics) = metrics {
            metrics.begin("reverse");
        }
        let mut data = data
            .chunks(self.capture_region.width() as usize * 4)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<u8>>>();
        data.reverse();
        let mut data = data.concat();
        if let Some(metrics) = metrics {
            metrics.end("reverse");
        }

        // The shuffle mask for converting BGRA -> RGBA
        if let Some(metrics) = metrics {
            metrics.begin("shuffle");
        }
        let mask: __m128i = unsafe {
            _mm_setr_epi8(
                2, 1, 0, 3, // First pixel
                6, 5, 4, 7, // Second pixel
                10, 9, 8, 11, // Third pixel
                14, 13, 12, 15, // Fourth pixel
            )
        };
        // For each 16-byte chunk in your data
        for chunk in data.chunks_exact_mut(16) {
            let mut vector = unsafe { _mm_loadu_si128(chunk.as_ptr() as *const __m128i) };
            vector = unsafe { _mm_shuffle_epi8(vector, mask) };
            unsafe { _mm_storeu_si128(chunk.as_mut_ptr() as *mut __m128i, vector) };
        }
        if let Some(metrics) = metrics {
            metrics.end("shuffle");
        }

        let data = RgbaImage::from_vec(
            self.capture_region.width() as u32,
            self.capture_region.height() as u32,
            data,
        );
        data.ok_or_else(|| anyhow!("Invalid image data"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use bevy::math::IVec2;

    use super::*;

    #[test]
    fn names() {
        get_monitor_infos().unwrap().iter().for_each(|info| {
            println!("{:?}", info);
        });
    }

    #[test]
    fn full_screenshots() {
        let capturers = get_full_monitor_capturers().unwrap();
        std::fs::create_dir_all("target/capture").unwrap();

        capturers.iter().for_each(|capturer| {
            let capture = capturer.capture(&mut None).unwrap();
            let mon_name_good = capturer.monitor.info.name.replace(r"\\.\", "");
            let path = format!("target/capture/full-{}.png", mon_name_good);
            capture.save(path).unwrap();
        });
    }

    #[test]
    fn region_screenshots() {
        let monitors = get_all_monitors().unwrap();
        let mut capturers = Vec::new();

        for monitor in monitors {
            let p0 = monitor.info.rect.top_left();
            let p1 = p0 + IVec2::new(100, 100);
            let region = IRect::from_corners(p0, p1);
            let capturer = get_monitor_capturer(Arc::new(monitor), region);
            capturers.push(capturer);
        }
        std::fs::create_dir_all("target/capture").unwrap();

        capturers.iter().for_each(|capturer| {
            let capture = capturer.capture(&mut None).unwrap();
            let mon_name_good = capturer.monitor.info.name.replace(r"\\.\", "");
            let path = format!("target/capture/region-{}.png", mon_name_good);
            capture.save(path).unwrap();
        });
    }

    #[test]
    fn capture_avg() {
        let capturers = get_full_monitor_capturers().unwrap();
        std::fs::create_dir_all("target/capture").unwrap();

        for _ in 0..100 {
            capturers.iter().for_each(|capturer| {
                let capture = capturer.capture(&mut None).unwrap();
                let (mut tot_r, mut tot_g, mut tot_b) = (0, 0, 0);

                for pixel in capture.enumerate_pixels() {
                    let image::Rgba([r, g, b, _]) = pixel.2; // Destructure the Rgba struct
                    tot_r += *r as u64;
                    tot_g += *g as u64;
                    tot_b += *b as u64;
                }
                let size = capture.iter().count() as u64;
                print!(
                    "{} -- avg: {:?}\t",
                    capturer.monitor.info.name,
                    (tot_r / size, tot_g / size, tot_b / size)
                );
            });
            print!("\n");
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    #[test]
    fn screenshot_speed() {
        let capturers = get_full_monitor_capturers().unwrap();
        let mut durations = Vec::new();
        for _ in 0..100 {
            capturers.iter().for_each(|capturer| {
                let start = std::time::Instant::now();
                let _ = capturer.capture(&mut None).unwrap();
                let duration = start.elapsed();
                durations.push(duration.as_millis());
            });
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let avg = durations.iter().sum::<u128>() / durations.len() as u128;
        println!("avg: {:?}ms", avg);
    }
}
