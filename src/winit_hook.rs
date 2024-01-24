use bevy_ecs::component::Component;
use bevy_window::Window;
use winit::window::WindowBuilder;

/// Types that represent extra data to be stored with a window.
#[allow(unused_variables)]
pub trait WindowHook: Clone + Component {
    /// Modifies a [`winit::window::WindowBuilder`] with extra configuration.
    fn builder_hook(&self, window: &Window, winit_builder: WindowBuilder) -> WindowBuilder;
    /// Modifies a [`winit::window::Window`] with extra configuration.
    fn window_hook(&self, window: &Window, winit_window: &winit::window::Window) {}
    /// Updates a [`winit::window::Window`] when the corresponding [`WindowHook`] has changed.
    fn changed_hook(&mut self, winit_window: &winit::window::Window, cached: &Self) {}
}

/// Component that represents no hook. It should not be instanced.
#[derive(Clone, Component, Debug, Default)]
pub struct NoHook;

impl WindowHook for NoHook {
    fn builder_hook(&self, _: &Window, window_builder: WindowBuilder) -> WindowBuilder {
        window_builder
    }
}
