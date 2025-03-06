use bevy::prelude::*;

use crate::{
    dev::user_testing::update_water_selection, grid::{GridCell, CELL_HEIGHT}, ground::Ground, mesh::{create_cube_mesh, CubeBundle}, neighborhood::Neighborhood, pair::Pair
};

pub const WATER_MESH_SCALE: f32 = 0.98;
pub const WATER_COLOR: Color = Color::srgb(0.0, 0.2, 0.9);

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWater>()
            .add_event::<TryShiftWater>()
            .add_event::<ShiftWater>();
        app.add_systems(Update, (
            spawn_water, 
            try_shift_water, 
            shift_water, 
            despawn_water, 
            connect_water_neighbors
        ));
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

fn spawn_water(
    mut event: EventReader<SpawnWater>,
    grounds: Query<(Entity, &GridCell, &GlobalTransform), (With<Ground>, Without<Water>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
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
                Neighborhood::default(),
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
    }
}

#[derive(Event)]
pub struct TryShiftWater {
    pub cell: Entity,
    pub shifting_upward: bool,
}

fn try_shift_water(
    mut event: EventReader<TryShiftWater>,
    grounds: Query<Entity, (With<Ground>, Without<Water>)>,
    pairs: Query<&Pair>,
    mut water_selected: EventWriter<ShiftWater>,
) {
    for check in event.read() {
        //  get the ground data
        let Ok(ground_entity) = grounds.get(check.cell) else {
            continue;
        };

        //  find current water, if exists
        for pair in pairs.iter() {
            if pair.ground == ground_entity {
                water_selected.send(ShiftWater { entity: pair.water, upward: check.shifting_upward });
                break;
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct ShiftWater {
    pub entity: Entity,
    pub upward: bool,
}

fn shift_water(
    mut event: EventReader<ShiftWater>,
    mut waters: Query<(&mut Water, &mut Transform), Without<Ground>>,
) {
    for shift in event.read() {
        let Ok((mut water, mut transform)) = waters.get_mut(shift.entity) else {
            continue;
        };

        //  update found water
        let direction = if shift.upward { 1.0 } else { -1.0 };
        water.amount += direction * CELL_HEIGHT;
        transform.translation.y += direction * CELL_HEIGHT;
    }
}

fn despawn_water(
    waters: Query<(Entity, &Parent, &GlobalTransform)>,
    cells: Query<&GlobalTransform, With<GridCell>>,
    mut commands: Commands,
) {
    for (entity, parent, water_transform) in waters.iter() {
        let Ok(cell_transform) = cells.get(parent.get()) else {
            continue;
        };
        if water_transform.translation().y < cell_transform.translation().y {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn connect_water_neighbors(
    mut waters: Query<(&GridCell, &mut Neighborhood), (With<Water>, Without<Ground>)>,
    neighbors: Query<(Entity, &GridCell), (With<Water>, Without<Ground>)>
) {
    for (cell, mut neighborhood) in waters.iter_mut() {
        for (neighbor_entity, neighbor) in neighbors.iter() {
            //  connect neighbors
            if cell.row - 1 == neighbor.row && cell.col == neighbor.col {
                neighborhood.left_neighbor = neighbor_entity;
            }
            if cell.row + 1 == neighbor.row && cell.col == neighbor.col {
                neighborhood.right_neighbor = neighbor_entity;
            }
            if cell.col - 1 == neighbor.col && cell.row == neighbor.row {
                neighborhood.front_neighbor = neighbor_entity;
            }
            if cell.col + 1 == neighbor.col && cell.row == neighbor.row {
                neighborhood.back_neighbor = neighbor_entity;
            }
        }
    }
}
