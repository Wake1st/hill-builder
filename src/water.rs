use bevy::prelude::*;

use crate::{
    draining::CheckDrainable,
    grid::{GridCell, CELL_HEIGHT},
    ground::Ground,
    mesh::{create_cube_mesh, CubeBundle},
    selection::{update_water_selection, WaterSelected},
};

pub const FILL_KEY: KeyCode = KeyCode::Tab;

pub const WATER_MESH_SCALE: f32 = 0.98;
pub const WATER_COLOR: Color = Color::srgb(0.0, 0.2, 0.9);

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaterToggle(false));
        app.add_event::<FillWater>();
        app.add_systems(Update, (toggle_water, fill_water, increase_water));
    }
}

#[derive(Resource)]
pub struct WaterToggle(pub bool);

pub fn toggle_water(keys: Res<ButtonInput<KeyCode>>, mut toggle: ResMut<WaterToggle>) {
    if keys.just_pressed(FILL_KEY) {
        toggle.0 = !toggle.0;
    }
}

#[derive(Component, Debug, Default)]
pub struct Water {
    pub amount: f32,
    pub rate: f32,
}

#[derive(Event)]
pub struct FillWater {
    pub cell: Entity,
}

fn fill_water(
    mut event: EventReader<FillWater>,
    grounds: Query<(&GridCell, &GlobalTransform), (With<Ground>, Without<Water>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut check_drainable: EventWriter<CheckDrainable>,
) {
    for fill in event.read() {
        //  get the ground data
        let Ok((ground_cell, ground_transform)) = grounds.get(fill.cell) else {
            continue;
        };

        //  spawning new water
        let mesh_handle = meshes.add(create_cube_mesh(Some(WATER_MESH_SCALE)));
        let mesh_matl = materials.add(WATER_COLOR);

        let water_entity = commands
            .spawn((
                Name::new("water"),
                Water {
                    amount: CELL_HEIGHT,
                    ..default()
                },
                Transform::from_translation(
                    ground_transform.translation() + Vec3::new(0.0, CELL_HEIGHT, 0.0),
                ),
                GridCell::from_grid_cell(ground_cell, CELL_HEIGHT),
                CubeBundle::new(mesh_handle, mesh_matl),
            ))
            .observe(update_water_selection::<Pointer<Down>>())
            .id();

        check_drainable.send(CheckDrainable { cell: water_entity });
    }
}

fn increase_water(
    mut event: EventReader<WaterSelected>,
    mut waters: Query<(&mut Water, &mut Transform), Without<Ground>>,
    mut check_drainable: EventWriter<CheckDrainable>,
) {
    for fill in event.read() {
        let Ok((mut water, mut transform)) = waters.get_mut(fill.entity) else {
            continue;
        };

        //  update found water
        water.amount += CELL_HEIGHT;
        transform.translation.y += CELL_HEIGHT;

        check_drainable.send(CheckDrainable { cell: fill.entity });
    }
}
