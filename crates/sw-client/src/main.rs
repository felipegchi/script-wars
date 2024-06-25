use specs::WorldExt;

use sw_client::structures::graphics::Graphics;
use sw_client::RendererPlugin;

use sw_engine::{
    events::EventsPlugin,
    BasicPlugin,
};

use sw_renderer::{
    model::Material, texture::Texture, window::ElementState, window::MouseButton, PhysicalSize,
    Window, WindowEvents,
};

/// Loads all the resources that are needed to run the game
async fn load(graphics: &mut Graphics) {
    let texture =
        Texture::from_bytes(&graphics.renderer, include_bytes!("../../../assets/mike.png"), "Bulacha").unwrap();

    let material = Material::from_texture(
        &graphics.renderer,
        texture,
        &graphics.primary.texture_bind_group_layout,
    );

    graphics.add_material(material);
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let window = Window::new("sw-engine", PhysicalSize::new(500, 500));
    let mut graphics = Graphics::new(&window).await;

    load(&mut graphics).await;

    let mut engine = sw_engine::Builder::new()
        .with(BasicPlugin)
        .with(EventsPlugin)
        .with(RendererPlugin { graphics })
        .build();

    window.half_window_size();
    window.center_window();

    window.run(move |_window, event| match event {
        WindowEvents::Keyboard {
            state,
            virtual_keycode: Some(key),
        } => {
            let mut graphics = engine.world.write_resource::<Graphics>();
            graphics.camera_controller.process_keyboard(key, state);
        }
        WindowEvents::MouseWheel { delta } => {
            let mut graphics = engine.world.write_resource::<Graphics>();
            graphics.camera_controller.process_scroll(&delta);
        }
        WindowEvents::MouseInput {
            button: MouseButton::Left,
            state,
            ..
        } => {
            let mut graphics = engine.world.write_resource::<Graphics>();
            graphics.camera_controller.mouse_pressed = state == ElementState::Pressed;
        }
        WindowEvents::MouseMotion { delta } => {
            let mut graphics = engine.world.write_resource::<Graphics>();
            graphics.camera_controller.process_mouse(delta.0, delta.1)
        }
        WindowEvents::Resized(size) => {
            engine.world.write_resource::<Graphics>().resize(size)
        },
        WindowEvents::Draw => engine.run(),
        _ => (),
    })
}