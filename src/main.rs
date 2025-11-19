use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider};
use gtk4::glib;

mod parser;
mod renderer;

use parser::html_parser::{parse_html, DomNode, WindowConfig, WidgetDefinition};
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
            return BOOL(0); 
        }
    }
    BOOL(1) 
}

#[cfg(target_os = "windows")]
unsafe fn get_desktop_workerw() -> Option<HWND> {
    let progman = FindWindowW(w!("Progman"), PCWSTR::null());
    
    if progman.0 == 0 {
        return None;
    }
    
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
fn set_as_desktop_widget(window: &ApplicationWindow, window_width: i32, window_height: i32, x_pos: Option<i32>, y_pos: Option<i32>) {
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
                
                // Usar posición especificada o centrar
                let x = x_pos.unwrap_or((screen_width - window_width) / 2);
                let y = y_pos.unwrap_or((screen_height - window_height) / 2);
                
                println!("✓ Pantalla: {}x{}", screen_width, screen_height);
                println!("✓ Ventana: {}x{}", window_width, window_height);
                println!("✓ Posición: ({}, {})", x, y);
                
                let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
                SetWindowLongPtrW(hwnd, GWL_STYLE, WS_POPUP.0 as isize);
                
                let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                SetWindowLongPtrW(
                    hwnd, 
                    GWL_EXSTYLE, 
                    ex_style | WS_EX_NOACTIVATE.0 as isize | WS_EX_TOOLWINDOW.0 as isize
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
                
                println!("✓ Ventana configurada como desktop widget!");
            } else {
                eprintln!("✗ Error: No se pudo obtener el HWND de la ventana");
            }
        }
    }
}

#[cfg(target_os = "windows")]
unsafe fn get_hwnd_from_surface(surface: &gtk4::gdk::Surface) -> Option<HWND> {
    use std::ffi::c_void;
    use gtk4::glib::object::IsA;
    use gtk4::glib::translate::ToGlibPtr;
    
    extern "C" {
        fn gdk_win32_surface_get_handle(surface: *mut c_void) -> *mut c_void;
    }
    
    let surface_ptr: *mut gtk4::glib::gobject_ffi::GObject = surface.as_ptr() as *mut _;
    
    let handle = gdk_win32_surface_get_handle(surface_ptr as *mut c_void);
    
    if !handle.is_null() {
        Some(HWND(handle as isize))
    } else {
        None
    }
}

fn build_ui(app: &Application) {
    use std::fs;
    use std::path::Path;
    
    // Obtener el directorio del ejecutable
    let exe_path = std::env::current_exe().expect("No se pudo obtener la ruta del ejecutable");
    let exe_dir = exe_path.parent().expect("No se pudo obtener el directorio del ejecutable");
    let widget_dir = exe_dir.join("widget");
    
    println!("🔍 Buscando widgets en: {:?}", widget_dir);
    
    if !widget_dir.exists() {
        eprintln!("✗ Error: La carpeta 'widget' no existe en {:?}", exe_dir);
        eprintln!("  Crea la carpeta y añade archivos .html dentro");
        return;
    }
    
    let mut all_widgets = Vec::new();
    
    // Leer todos los archivos .html en la carpeta widget/
    match fs::read_dir(&widget_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.extension().and_then(|s| s.to_str()) == Some("html") {
                        println!("📄 Cargando: {:?}", path.file_name().unwrap());
                        
                        match fs::read_to_string(&path) {
                            Ok(html_content) => {
                                // Parsear el HTML del archivo
                                if let Some(mut widgets) = parse_html(&html_content) {
                                    // Si el archivo no tiene ID de widget, usar el nombre del archivo
                                    for widget in &mut widgets {
                                        if widget.id == "main" {
                                            let filename = path.file_stem()
                                                .and_then(|s| s.to_str())
                                                .unwrap_or("unnamed");
                                            widget.id = filename.to_string();
                                        }
                                    }
                                    
                                    all_widgets.extend(widgets);
                                    println!("  ✓ Parseado correctamente");
                                } else {
                                    eprintln!("  ✗ Error al parsear HTML");
                                }
                            }
                            Err(e) => {
                                eprintln!("  ✗ Error al leer archivo: {}", e);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Error al leer la carpeta widget/: {}", e);
            return;
        }
    }
    
    if all_widgets.is_empty() {
        eprintln!("✗ No se encontraron widgets válidos en la carpeta widget/");
        eprintln!("  Asegúrate de tener archivos .html en la carpeta widget/");
        return;
    }
    
    println!("\n✓ Total de widgets encontrados: {}", all_widgets.len());
    
    for widget_def in all_widgets {
        create_widget_window(app, &widget_def);
    }
}

fn create_widget_window(app: &Application, widget_def: &WidgetDefinition) {
    let config = &widget_def.config;
    
    println!("\n=== CREANDO WIDGET: {} ===", widget_def.id);
    println!("  - Tamaño: {}x{}", config.width, config.height);
    println!("  - Decoraciones: {}", config.decorations);
    println!("  - Posición: {:?}", (config.x, config.y));
    
    // DEBUG: Ver estructura del DOM
    println!("\n=== ESTRUCTURA DOM ===");
    print_dom_tree(&widget_def.body, 0);
    println!("======================\n");
    
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(config.width as i32)
        .default_height(config.height as i32)
        .decorated(config.decorations)
        .resizable(config.resizable)
        .build();

    // Configurar CSS para fondo visible
    let css_provider = CssProvider::new();
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

    let root_widget = render_dom_to_gtk(&widget_def.body);
    window.set_child(Some(&root_widget));
    
    println!("✓ Ventana GTK creada para widget '{}'", widget_def.id);
    println!("✓ Widget root renderizado: {:?}", root_widget.widget_name());
    
    // Mostrar la ventana PRIMERO, antes de hacer SetParent
    window.present();
    println!("✓ Ventana mostrada");
    
    #[cfg(target_os = "windows")]
    {
        let win_width = config.width;
        let win_height = config.height;
        let x_pos = config.x;
        let y_pos = config.y;
        
        // Usar un timer para hacer el SetParent DESPUÉS de que GTK haya renderizado
        let window_clone = window.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
            println!("→ Convirtiendo a desktop widget...");
            set_as_desktop_widget(&window_clone, win_width as i32, win_height as i32, x_pos, y_pos);
            
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