use crate::render::Render;


mod termion;


pub use termion::alloc::TermionFixed;


/// Backend used in [`Screen`](crate::Screen).
pub trait Backend {
    type Renderer: Render;
    type FlushError: core::fmt::Debug; // TODO: better bounds.

    fn render<F>(&mut self, ui: F)
    where
        F: FnOnce(&mut Self::Renderer);
    fn flush(&mut self) -> Result<(), Self::FlushError>;
}
