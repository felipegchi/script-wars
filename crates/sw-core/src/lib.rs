use components::Position;
use events::EventHandler;
use events::UserEvent;
use shrev::EventChannel;
use specs::Dispatcher;
use specs::DispatcherBuilder;
use specs::World;
use specs::WorldExt;
use world::generators::classic::ClassicGenerator;
use world::manager::ChunkManager;

pub mod components;
pub mod entity;
pub mod events;
pub mod id;
pub mod world;

pub type UserChannel = EventChannel<UserEvent>;

pub struct Engine {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>,
}

impl Engine {
    pub fn dispatch(&mut self) {
        self.dispatcher.dispatch(&self.world);
    }
}

impl Default for Engine {
    fn default() -> Self {
        let mut world = World::default();

        world.register::<Position>();
        world.insert(ChunkManager::new(Box::new(ClassicGenerator::new(1))));

        Engine {
            world,
            dispatcher: DispatcherBuilder::new().build(),
        }
    }
}

pub fn init() -> Engine {
    Engine::default()
}
