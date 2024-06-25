//! Module that contains a struct called [Graphics] that holds all information needed to render the
//! game.

use sw_renderer::{
    camera::{self, Camera, CameraController, Projection},
    globals::Globals,
    model::Material,
    primary::PrimaryPass,
    renderer::Renderer,
    ui::UIPass,
    PhysicalSize, Window,
};

/// All the things that are needed to render everything in the voxelia-engine crate.
pub struct Graphics {
    pub renderer: Renderer,
    pub materials: Vec<Material>,
    pub globals: Globals,
    pub primary: PrimaryPass,
    pub ui: UIPass,
    pub projection: Projection,
    pub camera: Camera,
    pub camera_controller: CameraController,
}

impl Graphics {
    pub async fn new(window: &Window) -> Graphics {
        let renderer = Renderer::new(window).await;
        let globals = Globals::new(&renderer);
        let primary = PrimaryPass::new(&renderer, &globals);
        let ui = UIPass::new(&renderer, &globals);

        let projection = Projection::new(renderer.size);

        let camera = Camera::new((5.0, 2.5, -15.0), cgmath::Deg(90.0), cgmath::Deg(0.0));

        let camera_controller = camera::CameraController::new(1.0, 0.1);

        let mut info = Graphics {
            renderer,
            materials: Vec::new(),
            globals,
            primary,
            ui,
            projection,
            camera,
            camera_controller,
        };

        info.update_camera();

        info
    }

    pub fn add_material(&mut self, material: Material) {
        self.materials.push(material)
    }

    pub fn update_camera(&mut self) {
        self.camera_controller.update_camera(&mut self.camera, 0.05);
        self.globals
            .update_camera(&self.renderer, &self.camera, &self.projection);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.renderer.resize(size);
        let config = &self.renderer.config;
        self.projection.aspect = config.width as f32 / config.height as f32;
        self.update_camera();
        self.primary.resize(&self.renderer);
        self.ui.resize(&self.renderer);
    }
}
