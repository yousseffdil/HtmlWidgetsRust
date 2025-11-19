use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk4::glib;

mod parser;
mod renderer;

use parser::html_parser::{parse_html, DomNode, WidgetDefinition};
use renderer::gtk_renderer::render_dom_to_gtk;

use std::sync::atomic::{AtomicBool, Ordering};
static VERBOSE: AtomicBool = AtomicBool::new(false);

// Macro para imprimir solo si verbose está activado
macro_rules! vprintln {
    ($($arg:tt)*) => {
        if VERBOSE.load(Ordering::Relaxed) {
            println!($($arg)*);
        }
    };
}

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND};

#[cfg(target_os = "windows")]
fn set_as_desktop_widget(window: &ApplicationWindow, window_width: i32, window_height: i32, x_pos: Option<i32>, y_pos: Option<i32>) {
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, SWP_NOACTIVATE, SWP_SHOWWINDOW, HWND_BOTTOM,
        GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW,
    };
    use windows::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITOR_DEFAULTTOPRIMARY, MONITORINFO,
    };
    
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
                
                let x = x_pos.unwrap_or((screen_width - window_width) / 2);
                let y = y_pos.unwrap_or((screen_height - window_height) / 2);
                
                vprintln!("✓ Pantalla: {}x{}", screen_width, screen_height);
                vprintln!("✓ Ventana: {}x{}", window_width, window_height);
                vprintln!("✓ Posición: ({}, {})", x, y);
                
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
                
                vprintln!("✓ Ventana configurada como desktop widget!");
            } else {
                eprintln!("✗ Error: No se pudo obtener el HWND de la ventana");
            }
        }
    }
}

#[cfg(target_os = "windows")]
unsafe fn get_hwnd_from_surface(surface: &gtk4::gdk::Surface) -> Option<HWND> {
    use std::ffi::c_void;
    
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
    
    
    let exe_path = std::env::current_exe().expect("No se pudo obtener la ruta del ejecutable");
    let exe_dir = exe_path.parent().expect("No se pudo obtener el directorio del ejecutable");
    let widget_dir = exe_dir.join("widget");
    
    vprintln!("🔍 Buscando widgets en: {:?}", widget_dir);
    
    if !widget_dir.exists() {
        eprintln!("✗ Error: La carpeta 'widget' no existe en {:?}", exe_dir);
        eprintln!("  Crea la carpeta y añade archivos .html dentro");
        return;
    }
    
    let mut all_widgets = Vec::new();
    
    match fs::read_dir(&widget_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.extension().and_then(|s| s.to_str()) == Some("html") {
                        vprintln!("📄 Cargando: {:?}", path.file_name().unwrap());
                        
                        match fs::read_to_string(&path) {
                            Ok(html_content) => {
                                if let Some(mut widgets) = parse_html(&html_content) {
                                    for widget in &mut widgets {
                                        if widget.id == "main" {
                                            let filename = path.file_stem().and_then(|s| s.to_str()).unwrap_or("unnamed");
                                            widget.id = filename.to_string();
                                        }
                                    }
                                    all_widgets.extend(widgets);
                                    vprintln!("  ✓ Parseado correctamente");
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
    
    vprintln!("\n✓ Total de widgets encontrados: {}", all_widgets.len());
    
    for widget_def in all_widgets {
        create_widget_window(app, &widget_def);
    }
}

fn create_widget_window(app: &Application, widget_def: &WidgetDefinition) {
    let config = &widget_def.config;
    
    vprintln!("\n=== CREANDO WIDGET: {} ===", widget_def.id);
    vprintln!("  - Tamaño: {}x{}", config.width, config.height);
    vprintln!("  - Decoraciones: {}", config.decorations);
    vprintln!("  - Posición: {:?}", (config.x, config.y));
    
    vprintln!("\n=== ESTRUCTURA DOM ===");
    print_dom_tree(&widget_def.body, 0);
    vprintln!("======================\n");
    
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(config.width as i32)
        .default_height(config.height as i32)
        .decorated(config.decorations)
        .resizable(config.resizable)
        .build();

    let css_provider = CssProvider::new();
    css_provider.load_from_path("style.css");
    
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &css_provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let root_widget = render_dom_to_gtk(&widget_def.body);
    window.set_child(Some(&root_widget));
    
    vprintln!("✓ Ventana GTK creada para widget '{}'", widget_def.id);
    vprintln!("✓ Widget root renderizado: {:?}", root_widget.widget_name());
    window.present();
    vprintln!("✓ Ventana mostrada");
    
    #[cfg(target_os = "windows")]
    {
        let win_width = config.width;
        let win_height = config.height;
        let x_pos = config.x;
        let y_pos = config.y;
        
        let window_clone = window.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(500), move || {
            vprintln!("→ Convirtiendo a desktop widget...");
            set_as_desktop_widget(&window_clone, win_width as i32, win_height as i32, x_pos, y_pos);
            
            let window_clone2 = window_clone.clone();
            /*
            glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
                window_clone2.queue_draw();
                glib::ControlFlow::Continue
            });
            */
        });
    }
}

fn print_dom_tree(node: &DomNode, level: usize) {
    let indent = "  ".repeat(level);
    vprintln!("{}Tag: {}", indent, node.tag_name);
    
    if !node.attributes.is_empty() {
        vprintln!("{}  Attrs: {:?}", indent, node.attributes);
    }
    
    if let Some(text) = &node.text_content {
        vprintln!("{}  Text: '{}'", indent, text);
    }
    
    for child in &node.children {
        print_dom_tree(child, level + 1);
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    if let Some(pos) = args.iter().position(|arg| arg == "--verbose") {
        VERBOSE.store(true, Ordering::Relaxed);
        args.remove(pos);
    }

    // Pasar solo los argumentos filtrados a GTK
    let app = Application::builder()
        .application_id("htmlwidgets.rust.gtk")
        .build();

    app.connect_activate(build_ui);
    app.run_with_args(&args);
}

