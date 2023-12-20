use crate::prelude::*;

pub mod hud;
pub mod assets;
pub mod spawning;
pub mod body;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((hud::HudPlugin, assets::AssetPlugin));
    }
}
