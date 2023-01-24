mod engine;

use engine::Context;
use engine::Globals;
use engine::Renderer;
use engine::Resource;
use engine::Window;
use engine::camera::Camera;
use engine::instance::Instance;
use engine::mesh::Mesh;
use engine::shapes::Drawable;
use engine::shapes::rect::SpriteMesh;
use specs::Join;
use specs::RunNow;
use specs::WorldExt;
use specs::{Component, DenseVecStorage, ReadStorage, System, WriteStorage};

use sw_engine::Position;

#[derive(Component)]
struct Renderable {}

struct FrontEnd {
    sprite: Resource,
    renderer: Renderer,
    context: Context,
    camera: Camera
}

impl<'a> System<'a> for FrontEnd {
    type SystemData = (ReadStorage<'a, Renderable>, WriteStorage<'a, Position>);

    fn run(&mut self, (renderable, position): Self::SystemData) {
        let mut to_draw = vec![(self.sprite, vec![])];

        for (_, position) in (&renderable, &position).join() {
            to_draw[0].1.push(Instance {
                position: cgmath::Point2 { x: position.x as f32, y: position.y as f32 },
                tex_coords: cgmath::Point2 { x: 0.0, y: 0.0 },
                z_index: 0.0
            });
        }

        self.renderer.render(&self.context, &to_draw, Globals {
            view_proj: self.camera.build_matrix().into(),
        });
    }
}

pub fn update(world: &specs::World, event: sw_engine::Event) {
    use sw_engine::Event::*;
    match event {
        NewEntity(id) => {
            let mut tracked = world.write_storage::<Renderable>();
            tracked.insert(id, Renderable {}).unwrap();
        }
    }
}

pub async fn run() {
    let window = engine::Window::new();
    let context = engine::Context::new(&window).await.unwrap();
    let mut renderer = engine::Renderer::new(&context);

    let sprite_shader = renderer.add_pipeline(engine::shaders::sprite_pipeline(&context, &renderer));
    let sprite_rect = Mesh::new(&context, engine::shapes::rect::VERTICES, engine::shapes::rect::INDICES);
    let sprite = renderer.add_resource(Box::new(SpriteMesh::new(sprite_shader, sprite_rect)));

    // Setup of the engine
    let mut engine = sw_engine::State::new(&update);

    // Setup of all the renderable things.
    engine.world.register::<Renderable>();

    engine.init();

    // Systems setup
    let mut render_system = FrontEnd {
        sprite,
        renderer,
        context,
        camera: Default::default(),
    };

    window.block(move |event| {
        match event {
            engine::Event::Draw => {
                engine.update();
                render_system.run_now(&engine.world);
            },
            engine::Event::Resize(new_size) => {
                render_system.context.resize(new_size);
                render_system.camera.aspect = render_system.context.aspect();
            },
        }
    })
}
