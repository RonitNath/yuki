use std::time::Duration;

use bevy::time::Stopwatch;
use rand::Rng;

use crate::{prelude::*, game::snow::Snow};

use self::{body::Body, assets::GeneratedAssets, hud::SelectedPos};

pub mod hud;
pub mod assets;
pub mod spawning;
pub mod body;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Body>().add_plugins((hud::HudPlugin, assets::AssetPlugin)).add_systems(Update, (combine, spawn_on_click.after(crate::controls::mouse_selection)));
    }
}


pub fn spawn_on_click(
    mut sp: ResMut<SelectedPos>,
    mut commands: Commands,
    assets: Res<GeneratedAssets>,
    mut cooldown: Local<f32>,
    time: Res<Time>,
) {

    *cooldown += time.delta_seconds();

    if let Some(pos) = sp.0 {

        if *cooldown > 0.1 {
            let dir = Vec2::Y;
            let color = String::from("WHITE");

            let size = rand::thread_rng().gen_range(1..5);
            Snow::spawn(pos, dir, color, &mut commands, &assets, size as f32 * 2.0);
    
            sp.0 = None;
            *cooldown = 0.0;
            dbg!("spawn sent");
        } else {
            // dbg!("Spwan accepted, waiting on cooldown");
        }

    }
}


pub fn combine(
    mut cev: EventReader<CollisionEvent>,
    mut commands: Commands,
    snow: Query<(&Body, &Transform)>,
    assets: Res<GeneratedAssets>

) {
    for ev in cev.iter() {
        match ev {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok((b1, tf1)) = snow.get(*e1) {
                    if let Ok((b2, tf2)) = snow.get(*e2) {
                        if b1.radius == b2.radius {

                            let pos1 = tf1.translation.truncate();
                            let pos2 = tf2.translation.truncate();

                            let pos = (pos1 + pos2) / 2.0;

                            if let Some(ec) = commands.get_entity(*e1) {
                                ec.despawn_recursive();
                            } else {
                                warn!("Couldn't despawn e1c");
                            }

                            if let Some(ec) = commands.get_entity(*e2) {
                                ec.despawn_recursive();
                            } else {
                                warn!("Couldn't despawn e1c");
                            }

                            let dir = Vec2::Y;
                            let color = String::from("WHITE");
                            Snow::spawn(pos, dir, color, &mut commands, &assets, b1.radius + 2.0);
                        }
                    } else {
                        warn!("E2 not found in combine");
                    }
                } else {
                    warn!("E1 not found in combine");
                }
            },
            CollisionEvent::Stopped(_, _, _) => {
                // do nothing
            },
        }
    }
}