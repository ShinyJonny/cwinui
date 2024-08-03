use crate::render::{Render, Draw};


mod termion;


pub use termion::alloc::{TermionFixed, TermionDyn};


pub trait Backend {
    type Renderer<'r>: Render;
    // FIXME: change to `core::error::Error` when `error_in_core` gets
    // stabilised.
    type FlushError: core::fmt::Debug;

    fn render<'a, 'r, F>(&'a mut self, ui: F)
    where
        F: FnOnce(&mut Self::Renderer<'r>),
        'a: 'r;
    fn flush(&mut self) -> Result<(), Self::FlushError>;
    fn render_fullscreen<'a, 'r, D: Draw<Self::Renderer<'r>>>(
        &'a mut self,
        drawable: &D
    )
    where
        'a: 'r,
    {
        self.render(|renderer| {
            drawable.draw(renderer, renderer.area());
        })
    }
}
