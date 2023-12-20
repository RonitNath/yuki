use std::f32::consts::PI;

pub use bevy::{ prelude::*, utils::{ HashMap, HashSet } };
use bevy::sprite::{ MaterialMesh2dBundle, Mesh2dHandle };
pub use bevy_rapier2d::prelude::*;
use rand::{ seq::SliceRandom, Rng };
pub use bevy::reflect::Reflect;

pub use crate::config::*;

pub type Render = MaterialMesh2dBundle<ColorMaterial>;

pub fn make_render(
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
    transform: Transform
) -> Render {
    MaterialMesh2dBundle {
        mesh: mesh.into(),
        material,
        transform,
        ..Default::default()
    }
}

#[derive(Bundle, Clone)]
pub struct PhysicsBody {
    pub body: RigidBody,
    pub collider: Collider,
    pub render: Render,
    pub x_f: ExternalForce,
    pub x_i: ExternalImpulse,
    pub vel: Velocity,
    pub damping: Damping,
}

impl PhysicsBody {
    pub fn new(collider: Collider, render: Render) -> Self {
        Self {
            collider,
            render,
            body: RigidBody::Dynamic,
            x_f: ExternalForce::default(),
            x_i: ExternalImpulse::default(),
            vel: Velocity::default(),
            damping: Damping {
                linear_damping: DEFAULT_LIN_DAMPING,
                angular_damping: DEFAULT_ANG_DAMPING,
            },
        }
    }

    pub fn from_assets(
        radius: f32,
        color: Color,
        pos: Vec2,
        dir: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> Self {
        let collider = Collider::ball(radius);
        let mesh = Mesh::from(shape::Circle::new(radius));
        let mut transform = Transform::from_xyz(pos.x, pos.y, VISIBLE_Z);
        transform.rotate_z(Vec2::Y.angle_between(dir));
        let render = Render {
            mesh: meshes.add(mesh).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform,
            ..Default::default()
        };
        Self::new(collider, render)
    }

    pub fn color(mut self, color: Handle<ColorMaterial>) -> Self {
        self.render.material = color;
        self
    }

    pub fn pos(mut self, pos: Vec3) -> Self {
        self.render.transform = Transform::from_translation(pos);
        self
    }

    pub fn basic(
        mesh: Mesh2dHandle,
        material: Handle<ColorMaterial>,
        collider: Collider,
        pos: Vec3
    ) -> Self {
        Self::new(collider, Render {
            mesh: mesh.into(),
            material,
            transform: Transform::from_translation(pos),
            ..Default::default()
        })
    }
}
