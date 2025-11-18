use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider};
use gtk4::glib;

mod parser;
mod renderer;

use parser::html_parser::{parse_html, DomNode, WindowConfig};
use renderer::gtk_renderer::render_dom_to_gtk;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, LPARAM, BOOL};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, FindWindowExW, SendMessageTimeoutW, SetParent, 
    EnumWindows, GetClassNameW, SMTO_NORMAL, SEND_MESSAGE_TIMEOUT_FLAGS,
};
#[cfg(target_os = "windows")]
use windows::core::{PCWSTR, w};

const HTML_SOURCE: &str = include_str!("html_source.html");

#[cfg(target_os = "windows")]
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let mut class_name = [0u16; 256];
    let _ = GetClassNameW(hwnd, &mut class_name);
    
    let class_str = String::from_utf16_lossy(&class_name);
    
    if class_str.starts_with("WorkerW") {
        let shelldll_hwnd = FindWindowExW(hwnd, HWND(0), w!("SHELLDLL_DefView"), PCWSTR::null());
        
        if shelldll_hwnd.0 != 0 {
            let worker_w = FindWindowExW(HWND(0), hwnd, w!("WorkerW"), PCWSTR::null());
            *(lparam.0 as *mut isize) = worker_w.0;
            return BOOL(0); // Stop enumeration
        }
    }
    BOOL(1) // Continue enumeration
}

#[cfg(target_os = "windows")]
unsafe fn get_desktop_workerw() -> Option<HWND> {
    let progman = FindWindowW(w!("Progman"), PCWSTR::null());
    
    if progman.0 == 0 {
        return None;
    }
    
    // Trigger creation of WorkerW window
    let mut result: usize = 0;
    let _ = SendMessageTimeoutW(
        progman,
        0x052C, // Undocumented message
        windows::Win32::Foundation::WPARAM(0),
        LPARAM(0),
        SEND_MESSAGE_TIMEOUT_FLAGS(SMTO_NORMAL.0),
        1000,
        Some(&mut result),
    );
    
    // Find the WorkerW window
    let mut workerw: isize = 0;
    let _ = EnumWindows(
        Some(enum_windows_callback),
        LPARAM(&mut workerw as *mut _ as isize),
    );
    
    if workerw != 0 {
        Some(HWND(workerw))
    } else {
        Some(progman)
    }
}

#[cfg(target_os = "windows")]
fn set_as_desktop_widget(window: &ApplicationWindow, window_width: i32, window_height: i32) {
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, SWP_NOACTIVATE, SWP_SHOWWINDOW, HWND_BOTTOM,
        GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, GWL_STYLE,
        WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT,
        WS_POPUP, WINDOW_STYLE,
    };
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITOR_DEFAULTTOPRIMARY, MONITORINFO,
    };
    
    gtk4::prelude::WidgetExt::realize(window);
    
    if let Some(surface) = window.surface() {
        unsafe {
            let hwnd = get_hwnd_from_surface(&surface);
            
            if let Some(hwnd) = hwnd {
                println!("✓ HWND obtenido: {:?}", hwnd);
                
                // Get screen dimensions
                let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
                let mut monitor_info = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    ..Default::default()
                };
                
                GetMonitorInfoW(hmonitor, &mut monitor_info);
                
                let screen_width = monitor_info.rcWork.right - monitor_info.rcWork.left;
                let screen_height = monitor_info.rcWork.bottom - monitor_info.rcWork.top;
                
                let x = (screen_width - window_width) / 2;
                let y = (screen_height - window_height) / 2;
                
                println!("✓ Pantalla: {}x{}", screen_width, screen_height);
                println!("✓ Ventana: {}x{}", window_width, window_height);
                println!("✓ Posición centrada: ({}, {})", x, y);
                
                // Cambiar estilo de ventana para que sea tipo desktop widget
                // NO usamos SetParent, solo cambiamos los estilos
                let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
                SetWindowLongPtrW(hwnd, GWL_STYLE, WS_POPUP.0 as isize);
                
                let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                SetWindowLongPtrW(
                    hwnd, 
                    GWL_EXSTYLE, 
                    ex_style | WS_EX_NOACTIVATE.0 as isize | WS_EX_TOOLWINDOW.0 as isize
                );
                
                // Posicionar en el fondo (debajo de otras ventanas pero visible)
                let _ = SetWindowPos(
                    hwnd,
                    HWND_BOTTOM,
                    x,
                    y,
                    window_width,
                    window_height,
                    SWP_NOACTIVATE | SWP_SHOWWINDOW,
                );
                
                println!("✓ Ventana configurada como desktop widget!");
                println!("  - Posicionada debajo de otras ventanas");
                println!("  - No se activará al hacer click");
            } else {
                eprintln!("✗ Error: No se pudo obtener el HWND de la ventana");
            }
        }
    }
}

#[cfg(target_os = "windows")]
unsafe fn get_hwnd_from_surface(surface: &gtk4::gdk::Surface) -> Option<HWND> {
    // Para GTK4 en Windows, necesitamos obtener el handle nativo
    // Esto requiere acceso a la implementación Win32 de GDK
    
    use std::ffi::c_void;
    use gtk4::glib::object::IsA;
    use gtk4::glib::translate::ToGlibPtr;
    
    // Función externa de GDK para obtener el handle de Windows
    extern "C" {
        fn gdk_win32_surface_get_handle(surface: *mut c_void) -> *mut c_void;
    }
    
    // Convertir el surface de GTK a puntero usando el tipo correcto
    let surface_ptr: *mut gtk4::glib::gobject_ffi::GObject = surface.as_ptr() as *mut _;
    
    let handle = gdk_win32_surface_get_handle(surface_ptr as *mut c_void);
    
    if !handle.is_null() {
        Some(HWND(handle as isize))
    } else {
        None
    }
}

fn build_ui(app: &Application) {
    let parse_result = parse_html(HTML_SOURCE);
    let root_dom: DomNode;
    let config: WindowConfig;

    if let Some(result) = parse_result {
        root_dom = result.body.clone();
        config = result.config;
        println!("✓ HTML parseado correctamente");
        println!("  - Tamaño: {}x{}", config.width, config.height);
        println!("  - Decoraciones: {}", config.decorations);
        
        // DEBUG: Ver estructura del DOM
        println!("\n=== ESTRUCTURA DOM ===");
        print_dom_tree(&root_dom, 0);
        println!("======================\n");
    } else {
        root_dom = DomNode {
            tag_name: "div".into(),
            attributes: Default::default(),
            children: vec![],
            text_content: Some("Error al parsear HTML".into()),
        };
        config = WindowConfig::default();
        eprintln!("✗ Error al parsear HTML");
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(config.width as i32)
        .default_height(config.height as i32)
        .decorated(false)
        .resizable(config.resizable)
        .build();

    // IMPORTANTE: Configurar CSS para fondo visible
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(
        "window { background-color: white; }
         box { background-color: white; }
         label { color: black; font-size: 14px; }"
    );
    
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let root_widget = render_dom_to_gtk(&root_dom);
    window.set_child(Some(&root_widget));
    
    println!("✓ Ventana GTK creada");
    println!("✓ Widget root renderizado: {:?}", root_widget.widget_name());
    
    // Mostrar la ventana PRIMERO, antes de hacer SetParent
    window.present();
    println!("✓ Ventana mostrada");
    
    #[cfg(target_os = "windows")]
    {
        let win_width = config.width;
        let win_height = config.height;
        
        // Usar un timer para hacer el SetParent DESPUÉS de que GTK haya renderizado
        let window_clone = window.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
            println!("→ Convirtiendo a desktop widget...");
            set_as_desktop_widget(&window_clone, win_width as i32, win_height as i32);
            
            // Forzar queue_draw periódicamente
            let window_clone2 = window_clone.clone();
            glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
                window_clone2.queue_draw();
                glib::ControlFlow::Continue
            });
        });
    }
}

// Función helper para debug
fn print_dom_tree(node: &DomNode, level: usize) {
    let indent = "  ".repeat(level);
    println!("{}Tag: {}", indent, node.tag_name);
    
    if !node.attributes.is_empty() {
        println!("{}  Attrs: {:?}", indent, node.attributes);
    }
    
    if let Some(text) = &node.text_content {
        println!("{}  Text: '{}'", indent, text);
    }
    
    for child in &node.children {
        print_dom_tree(child, level + 1);
    }
}

fn main() {
    let app = Application::builder()
        .application_id("htmlwidgets.rust.gtk")
        .build();

    app.connect_activate(build_ui);
    app.run();
}