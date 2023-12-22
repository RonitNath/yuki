use crate::{prelude::*, logic::{assets::GeneratedAssets, body::BodyBundle}};


#[derive(Bundle)]
pub struct Snow {
    ace: ActiveEvents
}


impl Snow {
    pub fn new() -> Self {
        Self {
            ace: ActiveEvents::COLLISION_EVENTS
        }
    }

    pub fn spawn(
        pos: Vec2,
        dir: Vec2,
        color: String,
        commands: &mut Commands,
        assets: &Res<GeneratedAssets>,
        size: f32,
    ) -> Entity {
        dbg!("spawning");
        let mesh = assets.meshes.get((size).to_string().as_str()).expect(format!("Size exists {}", (size).to_string()).as_str());
        let (color, material) = assets.colors.get(&color).unwrap();
        let body = BodyBundle::spawn(
            size,
            *color,
            pos,
            dir,
            mesh.clone(),
            material.clone(),
            commands
        );

        if let Some(mut ec) = commands.get_entity(body) {
            ec.insert(Snow::new());
        } else {
            warn!("Spawned entity not found");
        }

        body
    }
}
