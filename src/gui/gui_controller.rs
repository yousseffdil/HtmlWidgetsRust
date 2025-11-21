use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, Separator, Switch};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::process::{Child, Command};
use std::path::Path;

pub struct WidgetManager {
    widgets: Rc<RefCell<HashMap<String, WidgetState>>>,
    processes: Rc<RefCell<HashMap<String, Child>>>,
}

#[derive(Clone, Debug)]
pub struct WidgetState {
    pub name: String,
    pub is_running: bool,
    pub path: String,
}

impl WidgetManager {
    pub fn new() -> Self {
        WidgetManager {
            widgets: Rc::new(RefCell::new(HashMap::new())),
            processes: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn load_widgets(&self, widget_folder: &str) -> Vec<String> {
        let mut widget_list = Vec::new();
    
        if let Ok(entries) = std::fs::read_dir(widget_folder) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(filename) = entry.file_name().into_string().ok() {
                            if filename.ends_with(".ytml") {
                                let widget_name = filename.replace(".ytml", "");
                                let path = format!("{}/{}", widget_folder, filename);
    
                                widget_list.push(widget_name.clone());
    
                                let state = WidgetState {
                                    name: widget_name.clone(),
                                    is_running: false,
                                    path,
                                };
    
                                self.widgets.borrow_mut().insert(widget_name, state);
                            }
                        }
                    }
                }
            }
        }
    
        widget_list.sort();
        widget_list
    }
    

    pub fn start_widget(&self, widget_name: &str) -> Result<(), String> {
        let mut widgets = self.widgets.borrow_mut();
        if let Some(widget) = widgets.get_mut(widget_name) {
            if widget.is_running {
                return Err(format!("Widget '{}' ya está ejecutándose", widget_name));
            }
            widget.is_running = true;
            println!("✓ Widget '{}' iniciado", widget_name);
            Ok(())
        } else {
            Err(format!("Widget '{}' no encontrado", widget_name))
        }
    }

    pub fn stop_widget(&self, widget_name: &str) -> Result<(), String> {
        let mut widgets = self.widgets.borrow_mut();
        if let Some(widget) = widgets.get_mut(widget_name) {
            if !widget.is_running {
                return Err(format!("Widget '{}' no está ejecutándose", widget_name));
            }

            widget.is_running = false;
            println!("✓ Widget '{}' detenido", widget_name);
            Ok(())
        } else {
            Err(format!("Widget '{}' no encontrado", widget_name))
        }
    }

    pub fn toggle_widget(&self, widget_name: &str) -> Result<bool, String> {
        let mut widgets = self.widgets.borrow_mut();
        if let Some(widget) = widgets.get_mut(widget_name) {
            if widget.is_running {
                widget.is_running = false;
                Ok(false)
            } else {
                widget.is_running = true;
                Ok(true)
            }
        } else {
            Err(format!("Widget '{}' no encontrado", widget_name))
        }
    }

    pub fn stop_all(&self) {
        let mut widgets = self.widgets.borrow_mut();
        for widget in widgets.values_mut() {
            widget.is_running = false;
        }
        println!("✓ Todos los widgets detenidos");
    }

    pub fn start_all(&self) {
        let mut widgets = self.widgets.borrow_mut();
        for widget in widgets.values_mut() {
            widget.is_running = true;
        }
        println!("✓ Todos los widgets iniciados");
    }

    pub fn is_running(&self, widget_name: &str) -> bool {
        self.widgets
            .borrow()
            .get(widget_name)
            .map(|w| w.is_running)
            .unwrap_or(false)
    }

    /// Obtener todos los widgets
    pub fn get_all_widgets(&self) -> Vec<String> {
        self.widgets.borrow().keys().cloned().collect()
    }
}

pub fn build_manager_ui(app: &Application, widget_folder: &str) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Widget Manager")
        .default_width(500)
        .default_height(600)
        .build();

    let main_box = Box::new(Orientation::Vertical, 8);
    main_box.set_margin_end(12);

    let title = Label::new(Some("Widget Manager"));
    title.add_css_class("title");
    title.set_widget_name("title_label");
    main_box.append(&title);

    let manager = Rc::new(WidgetManager::new());
    let widgets = manager.load_widgets(widget_folder);

    if widgets.is_empty() {
        let no_widgets = Label::new(Some("No se encontraron widgets."));
        no_widgets.add_css_class("warning");
        main_box.append(&no_widgets);
    } else {
        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::None);

        for widget_name in &widgets {
            let row = ListBoxRow::new();
            row.set_activatable(false);

            let row_box = Box::new(Orientation::Horizontal, 10);
            row_box.set_margin_end(8);

            let info_box = Box::new(Orientation::Vertical, 4);
            let label = Label::new(Some(widget_name));
            label.set_halign(gtk4::Align::Start);

            let status_label = Label::new(Some("Detenido"));
            status_label.set_halign(gtk4::Align::Start);
            status_label.add_css_class("stopped");

            info_box.append(&label);
            info_box.append(&status_label);

            let switch = Switch::new();
            switch.set_valign(gtk4::Align::Center);

            let manager_clone = manager.clone();
            let widget_name_clone = widget_name.clone();
            let status_label_clone = status_label.clone();

            switch.connect_active_notify(move |_| {
                if let Ok(is_running) = manager_clone.toggle_widget(&widget_name_clone) {
                    if is_running {
                        status_label_clone.set_text("✓ Ejecutándose");
                        status_label_clone.remove_css_class("stopped");
                        status_label_clone.add_css_class("running");
                    } else {
                        status_label_clone.set_text("Detenido");
                        status_label_clone.remove_css_class("running");
                        status_label_clone.add_css_class("stopped");
                    }
                }
            });

            row_box.append(&info_box);
            row_box.set_hexpand(true);
            row_box.append(&switch);

            row.set_child(Some(&row_box));
            list_box.append(&row);
        }

        scrolled.set_child(Some(&list_box));
        main_box.append(&scrolled);
    }

    main_box.append(&Separator::new(Orientation::Horizontal));

    let control_box = Box::new(Orientation::Horizontal, 8);
    control_box.set_halign(gtk4::Align::Center);

    let start_all_btn = Button::with_label("Iniciar Todo");
    start_all_btn.add_css_class("flat-button");
    let manager_clone = manager.clone();
    start_all_btn.connect_clicked(move |_| manager_clone.start_all());

    let stop_all_btn = Button::with_label("Detener Todo");
    stop_all_btn.add_css_class("flat-button");
    let manager_clone = manager.clone();
    stop_all_btn.connect_clicked(move |_| manager_clone.stop_all());

    control_box.append(&start_all_btn);
    control_box.append(&stop_all_btn);

    main_box.append(&control_box);

    window.set_child(Some(&main_box));
    window.present();
}
