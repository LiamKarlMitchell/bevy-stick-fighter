use bevy::prelude::*;

mod avian3d;
mod bevy_enhanced_input;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((avian3d::plugin, bevy_enhanced_input::plugin));
}
