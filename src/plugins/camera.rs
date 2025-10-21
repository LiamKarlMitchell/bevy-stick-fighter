use bevy::prelude::*;

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera);
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
    ));
}
