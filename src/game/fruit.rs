use crate::{prelude::*, logic::{assets::GeneratedAssets, body::BodyBundle}};


#[derive(Component)]
pub struct Fruit;


impl Fruit {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn(
        pos: Vec2,
        dir: Vec2,
        color: String,
        commands: &mut Commands,
        assets: &Res<GeneratedAssets>
    ) -> Entity {
        let mesh = assets.meshes.get("DEBUG").unwrap();
        let (color, material) = assets.colors.get(&color).unwrap();
        let body = BodyBundle::spawn(
            RADIUS,
            *color,
            pos,
            dir,
            mesh.clone(),
            material.clone(),
            commands
        );

        body
    }
}
