use crate::{
    prelude::*, logic::{assets::GeneratedAssets, hud::SelectedPos},
};

pub mod snow;
use snow::Snow;


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

    let wall_collider: Collider = Collider::cuboid(wall_half_x, wall_half_y);
    let floor_collider: Collider = Collider::cuboid(floor_half_x, floor_half_y);

    let color = &assets.colors.get("BLACK").expect("Colors always exist").1;
    let wall_mesh = assets.meshes.get("WALL").expect("meshes");
    let floor_mesh = assets.meshes.get("FLOOR").expect("floor");
    // left

    let pos = Transform::from_translation(Vec3::new(-floor_half_x, 0.0, VISIBLE_Z));
    let render = make_render(wall_mesh.clone(), color.clone(), pos);
    commands.spawn(Wall).insert(RigidBody::Fixed).insert(wall_collider.clone()).insert(render);

    // right
    let pos = Transform::from_translation(Vec3::new(floor_half_x, 0.0, VISIBLE_Z));
    let render = make_render(wall_mesh.clone(), color.clone(), pos);
    commands.spawn(Wall).insert(RigidBody::Fixed).insert(wall_collider).insert(render);

    // bottom
    let pos = Transform::from_translation(Vec3::new(0.0, -wall_half_y, VISIBLE_Z));
    let render = make_render(floor_mesh.clone(), color.clone(), pos);
    commands.spawn(Wall).insert(RigidBody::Fixed).insert(floor_collider).insert(render);



    // Add 1 agent to the center of the world

    let pos = Vec2::ZERO;
    let dir = Vec2::Y;
    let color = String::from("WHITE");
    Snow::spawn(pos, dir, color, &mut commands, &assets, RADIUS);
}

