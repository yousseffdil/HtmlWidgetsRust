use gtk4::ApplicationWindow;

#[cfg(target_os="linux")]
pub fn set_as_desktop_widget(
    window: &ApplicationWindow, width: i32, height: i32, x: Option<i32>, y: Option<i32>) {
    //TODO Make the compatibility for linux "use the layer library from wayland"

    //window.set_decorated(false);
}