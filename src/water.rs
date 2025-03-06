use bevy::prelude::*;

use crate::{
    dev::user_testing::{update_water_selection, WaterSelected},
    draining::CheckDrainable,
    grid::{GridCell, CELL_HEIGHT},
    ground::Ground,
    mesh::{create_cube_mesh, CubeBundle},
    pair::Pair,
};

pub const WATER_MESH_SCALE: f32 = 0.98;
pub const WATER_COLOR: Color = Color::srgb(0.0, 0.2, 0.9);

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWater>().add_event::<CheckWater>();
        app.add_systems(Update, (create_water, check_water, increase_water));
    }
}

#[derive(Component, Debug, Default)]
pub struct Water {
    pub amount: f32,
    pub rate: f32,
}

#[derive(Event)]
pub struct SpawnWater {
    pub ground: Entity,
}

fn create_water(
    mut event: EventReader<SpawnWater>,
    grounds: Query<(Entity, &GridCell, &GlobalTransform), (With<Ground>, Without<Water>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut check_drainable: EventWriter<CheckDrainable>,
) {
    for spawn in event.read() {
        //  get the ground data
        let Ok((ground_entity, ground_cell, ground_transform)) = grounds.get(spawn.ground) else {
            continue;
        };

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

        commands.spawn((
            Name::new("Pair"),
            Pair {
                ground: ground_entity,
                water: water_entity,
            },
        ));

        check_drainable.send(CheckDrainable { cell: water_entity });
    }
}

#[derive(Event)]
pub struct CheckWater {
    pub cell: Entity,
}

fn check_water(
    mut event: EventReader<CheckWater>,
    grounds: Query<Entity, (With<Ground>, Without<Water>)>,
    pairs: Query<&Pair>,
    mut water_selected: EventWriter<WaterSelected>,
    mut spawn_water: EventWriter<SpawnWater>,
) {
    for fill in event.read() {
        //  get the ground data
        let Ok(ground_entity) = grounds.get(fill.cell) else {
            continue;
        };

        //  find current water, if exists
        let mut pair_found: bool = false;
        for pair in pairs.iter() {
            if pair.ground == ground_entity {
                water_selected.send(WaterSelected { entity: pair.water });

                pair_found = true;
                break;
            }
        }

        //  spawning new water and pair if none exists
        if !pair_found {
            spawn_water.send(SpawnWater {
                ground: ground_entity,
            });
        }
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
