use crate::{
    prelude::*, logic::assets::GeneratedAssets,
};

pub mod fruit;
use fruit::Fruit;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize);
    }
}

#[derive(Component)]
pub struct Wall;


pub fn initialize(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<GeneratedAssets>
) {
    // Create the map
    let wall_half_x = 10.0;
    let wall_half_y = 1000.0;
    let wall_collider: Collider = Collider::cuboid(wall_half_x, wall_half_y);
    let floor_half_x = 800.0;
    let floor_half_y = 10.0;
    let floor_collider: Collider = Collider::cuboid(floor_half_x, floor_half_y);

    // left
    commands.spawn(Wall).insert(RigidBody::Fixed).insert(wall_collider).insert(render).insert(pos);

    // right
    commands.spawn(Wall).insert(RigidBody::Fixed).insert(wall_collider).insert(render).insert(pos);

    // bottom
    commands.spawn(Wall).insert(RigidBody::Fixed).insert(floor_collider).insert(render).insert(pos);



    // Add 1 agent to the center of the world

    let pos = Vec2::ZERO;
    let dir = Vec2::Y;
    let color = String::from("WHITE");
    Fruit::spawn(pos, dir, color, &mut commands, &assets);
}

