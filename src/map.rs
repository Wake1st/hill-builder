use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    block::{Block, BlockBundle}, mesh::create_cube_mesh, neighborhood::Neighborhood, selection::{update_block_selection, update_material_on}, shifting::SHIFT_AMOUNT, water::{Water, WATER_COLOR, WATER_MESH_SCALE}
};

const MAP_SIZE_DEFAULT: i32 = 8;
const GAP: f32 = 0.1;

const GROUND_COLOR: Color = Color::srgb(0.0, 0.9, 0.1);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentMapSettings::default());

        app.add_event::<GenerateMap>()
            .add_event::<ClearMap>();
        
        app.add_systems(Update, (clear_map, store_map, (generate_map, allocate_neighbors).chain()));
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
            terrain: Default::default() 
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum TerrainSettings {
    #[default]
    FLAT,
    CURVED(CurvedTerrainSettings)
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
    mut blocks: Query<Entity, With<Block>>,
    mut commands: Commands,
) {
    for _ in event.read() {
        for entity in blocks.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn store_map(
    mut event: EventReader<GenerateMap>,
    mut settings: ResMut<CurrentMapSettings>
) {
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
        let water_matl = materials.add(WATER_COLOR);

        let ground_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh(None));
        let water_mesh_handle = meshes.add(create_cube_mesh(Some(WATER_MESH_SCALE)));
    
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
                    .spawn(BlockBundle::new(
                        ground_mesh_handle.clone(),
                        ground_matl.clone(),
                        map_offset,
                        IVec3::new(i, j, layer),
                    ))
                    .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
                    .observe(update_material_on::<Pointer<Out>>(ground_matl.clone()))
                    .observe(update_block_selection::<Pointer<Down>>())
                    .with_child((
                        Water { amount: 0.0 },
                        Mesh3d(water_mesh_handle.clone()),
                        MeshMaterial3d(water_matl.clone()),
                        Transform::from_xyz(0.0, SHIFT_AMOUNT, 0.0),
                    ));
            }
        }
    }
}

fn generate_layer(x: i32, y: i32, settings: &CurvedTerrainSettings) -> i32 {
    (settings.amplitude.x
        * ops::sin(x as f32 * settings.wavelength.x + settings.phase_shift.x)
        + settings.vertical_shift.x
        + settings.amplitude.y
            * ops::cos(y as f32 * settings.wavelength.y + settings.phase_shift.y)
        + settings.vertical_shift.y) as i32
}

fn allocate_neighbors(
    mut blocks: Query<(Entity, &Block, &mut Neighborhood)>,
    neighbors: Query<(Entity, &Block)>,
) {
    for (_, block, mut neighborhood) in blocks.iter_mut() {
        for (neighbor_entity, neighbor) in neighbors.iter() {
            if block.row - 1 == neighbor.row && block.col == neighbor.col {
                neighborhood.left_neighbor = neighbor_entity;
            }
            if block.row + 1 == neighbor.row && block.col == neighbor.col {
                neighborhood.right_neighbor = neighbor_entity;
            }
            if block.col - 1 == neighbor.col && block.row == neighbor.row {
                neighborhood.front_neighbor = neighbor_entity;
            }
            if block.col + 1 == neighbor.col && block.row == neighbor.row {
                neighborhood.back_neighbor = neighbor_entity;
            }
        }
    }
}
