# bevy_winit_hook

Exposes hooks to update winit::window::WindowBuilder and the resulting winit::window::Window.  
Also exposes a callback for change events.

This is a fork of [bevy_winit](https://github.com/bevyengine/bevy/tree/main/crates/bevy_winit).

Currently compatible with bevy v0.12.1.

## Example

```rust
use bevy_winit_hook::HookedWinitPlugin;
use bevy_winit_hook::WindowHook;
// winit feature 'x11' enabled
use winit::platform::x11::WindowType;
use bevy::prelude::*;

#[derive(Clone, Component)]
struct X11Ext {
    window_types: Option<Vec<WindowType>>,
}

impl WindowHook for X11Ext {
    fn builder_hook(&self, window: &Window, winit_builder: WindowBuilder) -> WindowBuilder {
        match &self.window_types {
            Some(types) => winit_builder.with_x11_window_type(types.clone()),
            None => winit_builder,
        }
    }
}

// Need to replace default WinitPlugin
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<WinitPlugin>()
                .add_after::<WinitPlugin, _>(HookedWinitPlugin::<X11Ext>::default()),
        )
        .add_systems(Startup, spawn_window)
        .run();
}

fn spawn_window(
    mut commands: Commands,
) {
    commands.spawn((
        Window::default(),
        X11Ext {
            window_types: Some(vec![WindowType::Splash]),
        },
    ));
}
```
