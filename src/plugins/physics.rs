use avian3d::prelude::*;
use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(Vec3::new(0.0, -9.81, 0.0)));
}

// Example of a test for a plugin
#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, plugin));
        app
    }

    #[test]
    fn app_plugin_registers_plugins() {
        let app = setup();

        // Check if the plugins are registered
        assert!(app.world().contains_resource::<Gravity>());
    }
}
