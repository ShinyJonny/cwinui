use crate::backend::Backend;
use crate::render::{Draw, Render};


#[derive(Debug)]
pub struct Screen<B: Backend> {
    backend: B,
}

impl<B: Backend> Screen<B> {
    /// Creates a new `Screen`.
    #[inline]
    pub const fn new(backend: B) -> Self
    {
        Self {
            backend,
        }
    }

    /// Renders `ui` into the internal buffer.
    pub fn render<F>(&mut self, ui: F)
    where
        F: FnOnce(&mut B::Renderer)
    {
        self.backend.render(ui);
    }

    /// Renders `drawable` in the full area of the backend's [`Render`]er.
    pub fn render_fullscreen<D: Draw<B::Renderer>>(&mut self, drawable: &D)
    {
        self.backend.render(|renderer| {
            drawable.draw(renderer, renderer.area());
        });
    }

    /// Flushes and displays the contents of the internal buffer.
    pub fn refresh(&mut self) -> Result<(), B::FlushError>
    {
        self.backend.flush()
    }
}
