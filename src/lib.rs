use bevy::prelude::*;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "{{project-name}}".to_string(),
        ..Default::default()
    });
    {% if wasm_resize_enabled %}
    #[cfg(target_arch = "wasm32")]
        app.add_system(handle_browser_resize);
    {% endif %}

    app.run();
}

{% if wasm_resize_enabled %}
#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
    }
}
{% endif %}
