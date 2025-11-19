use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};

mod parser;
mod renderer;
mod platform; 

use parser::html_parser::{parse_html, DomNode, WidgetDefinition};
use renderer::gtk_renderer::render_dom_to_gtk;

use std::sync::atomic::{AtomicBool, Ordering};

mod utils;
use utils::VERBOSE;


fn build_ui(app: &Application) {
    use std::fs;

    let exe_path = std::env::current_exe().expect("No se pudo obtener la ruta del ejecutable");
    let exe_dir = exe_path.parent().expect("No se pudo obtener el directorio del ejecutable");
    let widget_dir = exe_dir.join("widget");

    vprintln!("Buscando widgets en: {:?}", widget_dir);

    if !widget_dir.exists() {
        eprintln!("Error: La carpeta 'widget' no existe en {:?}", exe_dir);
        eprintln!("  Crea la carpeta y añade archivos .html dentro");
        return;
    }

    let mut all_widgets = Vec::new();

    for entry in fs::read_dir(&widget_dir).unwrap_or_else(|e| {
        eprintln!("✗ Error al leer la carpeta widget/: {}", e);
        std::process::exit(1);
    }) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("html") {
                vprintln!("Cargando: {:?}", path.file_name().unwrap());

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
                            vprintln!("  Parseado correctamente");
                        } else {
                            eprintln!("  Error al parsear HTML");
                        }
                    }
                    Err(e) => {
                        eprintln!("  Error al leer archivo: {}", e);
                    }
                }
            }
        }
    }

    if all_widgets.is_empty() {
        eprintln!("No se encontraron widgets válidos en la carpeta widget/");
        return;
    }

    vprintln!("\nTotal de widgets encontrados: {}", all_widgets.len());

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

    let win_width = config.width;
    let win_height = config.height;
    let x_pos = config.x;
    let y_pos = config.y;

    #[cfg(target_os = "windows")]
    platform::set_as_desktop_widget(
        &window,
        win_width as i32,
        win_height as i32,
        x_pos.map(|v| v as i32),
        y_pos.map(|v| v as i32),
    );
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

    let app = Application::builder()
        .application_id("htmlwidgets.rust.gtk")
        .build();

    app.connect_activate(build_ui);
    app.run_with_args(&args);
}
