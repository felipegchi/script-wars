//! This module defines a [Pass] that is a trait to render something with a [RenderPipeline].

use wgpu::{CommandEncoder, TextureView};

use crate::{globals::Globals, model::{Material, Mesh}, renderer::Renderer};

pub mod primary;
pub mod ui;

/// Shared behaviour of being something that is able to render thigns to the screen
pub trait Pass {
    fn draw(
        &self,
        view: &TextureView,
        encoder: &mut CommandEncoder,
        materials: &[Material],
        meshes: &[&Mesh],
        globals: &Globals
    );
}
