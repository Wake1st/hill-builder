use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    grid::{GridCell, GridCellBundle},
    ground::Ground,
    mesh::{create_cube_mesh, CubeBundle},
    neighborhood::Neighborhood,
    selection::{update_ground_selection, update_material_on}, water::Water,
};

const MAP_SIZE_DEFAULT: i32 = 8;
const GAP: f32 = 0.1;

const GROUND_COLOR: Color = Color::srgb(0.0, 0.9, 0.1);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentMapSettings::default());

        app.add_event::<GenerateMap>().add_event::<ClearMap>();

        app.add_systems(
            Update,
            (
                clear_map,
                store_map,
                (generate_map, connect_ground_neighbors).chain(),
            ),
        );
    }
}

#[derive(Resource, Debug, Default)]
pub struct CurrentMapSettings {
    pub value: MapGenerationSettings,
}

#[derive(Event)]
pub struct GenerateMap {
    pub settings: MapGenerationSettings,
}

#[derive(Event)]
pub struct ClearMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapGenerationSettings {
    pub size: i32,
    pub terrain: TerrainSettings,
}

impl Default for MapGenerationSettings {
    fn default() -> Self {
        Self {
            size: MAP_SIZE_DEFAULT,
            terrain: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum TerrainSettings {
    #[default]
    FLAT,
    CURVED(CurvedTerrainSettings),
}

#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CurvedTerrainSettings {
    pub amplitude: Vec2,
    pub wavelength: Vec2,
    pub vertical_shift: Vec2,
    pub phase_shift: Vec2,
}

fn clear_map(
    mut event: EventReader<ClearMap>,
    mut cells: Query<Entity, With<GridCell>>,
    mut commands: Commands,
) {
    //  TODO: despawn water
    for _ in event.read() {
        for entity in cells.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn store_map(mut event: EventReader<GenerateMap>, mut settings: ResMut<CurrentMapSettings>) {
    for generation in event.read() {
        settings.value = generation.settings.clone();
    }
}

fn generate_map(
    mut event: EventReader<GenerateMap>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for generation in event.read() {
        let hover_matl = materials.add(Color::WHITE);
        let ground_matl = materials.add(GROUND_COLOR);

        let ground_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh(None));

        let map_size = generation.settings.size;
        let map_offset: f32 = (map_size as f32) * (1. + GAP) / 2.0;

        for i in 0..map_size {
            for j in 0..map_size {
                let layer: i32 = match &generation.settings.terrain {
                    TerrainSettings::FLAT => 0,
                    TerrainSettings::CURVED(settings) => generate_layer(i, j, &settings),
                };

                // render the mesh with the custom texture, and add the marker.
                commands
                    .spawn((
                        Ground,
                        CubeBundle::new(ground_mesh_handle.clone(), ground_matl.clone()),
                        GridCellBundle::new(map_offset, IVec3::new(i, j, layer)),
                    ))
                    .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
                    .observe(update_material_on::<Pointer<Out>>(ground_matl.clone()))
                    .observe(update_ground_selection::<Pointer<Down>>());
            }
        }
    }
}

fn generate_layer(x: i32, y: i32, settings: &CurvedTerrainSettings) -> i32 {
    (settings.amplitude.x * ops::sin(x as f32 * settings.wavelength.x + settings.phase_shift.x)
        + settings.vertical_shift.x
        + settings.amplitude.y
            * ops::cos(y as f32 * settings.wavelength.y + settings.phase_shift.y)
        + settings.vertical_shift.y) as i32
}

fn connect_ground_neighbors(
    mut cells: Query<(&GridCell, &mut Neighborhood), (With<Ground>, Without<Water>)>,
    neighbors: Query<(Entity, &GridCell), (With<Ground>, Without<Water>)>,
) {
    for (cell, mut neighborhood) in cells.iter_mut() {
        for (neighbor_entity, neighbor) in neighbors.iter() {
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
