use bevy_egui::{ egui, EguiContexts };

use crate::{ prelude::*};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraFollows>()
            .init_resource::<SelectedPos>()
            .init_resource::<ActiveControl>()
            .add_systems(Update, display);
    }
}

#[derive(Resource, Default)]
pub struct ActiveControl(pub Option<Entity>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CameraFollows(pub Option<Entity>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SelectedPos(pub Option<Vec2>);

pub fn display(
    mut contexts: EguiContexts,
    mut display: Local<HashSet<String>>,
    mut camera_follows: ResMut<CameraFollows>,
    keys: Res<Input<KeyCode>>,
    sp: Res<SelectedPos>,
    mut ac: ResMut<ActiveControl>,
    mut gizmos: Gizmos
) {
    // let mut agent = None;

    egui::Window::new("Active Control").default_open(false).show(contexts.ctx_mut(), |ui| {
        match ac.0 {
            None => {
                ui.label("Select an agent");
            }
            Some(e) => {
                ui.label(format!("Selected agent {:?}", e));
                if ui.button("Deselect").clicked() {
                    ac.0 = None;
                    return;
                }
                match camera_follows.0 {
                    None => {
                        if ui.button("Follow agent").clicked() {
                            camera_follows.0 = Some(e);
                        }
                    }
                    Some(a) => {
                        match a == e {
                            true => {
                                // the agent we are following is the agent we selected
                                if ui.button("Stop following agent").clicked() {
                                    camera_follows.0 = None;
                                }
                            }
                            false => {
                                // we are following an agent other than the one we selected
                                if ui.button("Follow agent").clicked() {
                                    camera_follows.0 = Some(e);
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}
