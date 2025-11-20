use crate::vprintln;
use gtk4::prelude::{NativeExt, ObjectType, WidgetExt};
use gtk4::ApplicationWindow;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTOPRIMARY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM, SWP_NOACTIVATE,
    SWP_SHOWWINDOW, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW,
};
pub fn set_as_desktop_widget(
    window: &ApplicationWindow,
    window_width: i32,
    window_height: i32,
    x_pos: Option<i32>,
    y_pos: Option<i32>,
) {
    gtk4::prelude::WidgetExt::realize(window);
    if let Some(surface) = window.surface() {
        unsafe {
            let hwnd = get_hwnd_from_surface(&surface);
            if let Some(hwnd) = hwnd {
                vprintln!("✓ HWND obtenido: {:?}", hwnd);
                let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
                let mut monitor_info = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    ..Default::default()
                };
                GetMonitorInfoW(hmonitor, &mut monitor_info);

                let screen_width = monitor_info.rcWork.right - monitor_info.rcWork.left;
                let screen_height = monitor_info.rcWork.bottom - monitor_info.rcWork.top;
                
                // I do this?
                let x = x_pos.unwrap_or((screen_width - window_width) / 2);
                let y = y_pos.unwrap_or((screen_height - window_height) / 2);

                vprintln!("Pantalla: {}x{}", screen_width, screen_height);
                vprintln!("Ventana: {}x{}", window_width, window_height);
                vprintln!("Posición: ({}, {})", x, y);
                let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                SetWindowLongPtrW(
                    hwnd,
                    GWL_EXSTYLE,
                    ex_style | WS_EX_NOACTIVATE.0 as isize | WS_EX_TOOLWINDOW.0 as isize,
                );
                let _ = SetWindowPos(
                    hwnd,
                    HWND_BOTTOM,
                    x,
                    y,
                    window_width,
                    window_height,
                    SWP_NOACTIVATE | SWP_SHOWWINDOW,
                );
                vprintln!("Ventana configurada como desktop widget!");
            }
        }
    }
}
unsafe fn get_hwnd_from_surface(surface: &gtk4::gdk::Surface) -> Option<HWND> {
    use std::ffi::c_void;
    extern "C" {
        fn gdk_win32_surface_get_handle(surface: *mut c_void) -> *mut c_void;
    }
    let surface_ptr = surface.as_ptr() as *mut c_void;
    let handle = gdk_win32_surface_get_handle(surface_ptr);
    if !handle.is_null() {
        Some(HWND(handle as isize))
    } else {
        None
    }
}
