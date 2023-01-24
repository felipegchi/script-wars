use winit::dpi::PhysicalSize;

/// Window events ready for the person making the game to use.
pub enum Event {
    Draw,
    Resize(PhysicalSize<u32>)
}

/// Input events like mouse or keyboard.
pub struct InputEvent {}