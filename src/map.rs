use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    block::{Block, BlockBundle, Neighborhood},
    mesh::create_cube_mesh,
    selection::{update_block_selection, update_material_on},
};

const MAP_SIZE: i32 = 44;
const GAP: f32 = 0.1;

const ROW_AMPLITUDE: f32 = 2.8;
const COL_AMPLITUDE: f32 = 1.4;
const ROW_WAVELENGTH: f32 = 0.2;
const COL_WAVELENGTH: f32 = 0.1;

const GROUND_COLOR: Color = Color::srgb(0.0, 0.9, 0.1);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapGenSettings::default());
        app.add_systems(Startup, (pre_setup, setup, allocate_neighbors).chain());
    }
}

#[derive(Resource, Default, Debug)]
pub struct MapGenSettings {
    pub amplitude: Vec2,
    pub wavelength: Vec2,
    pub vertical_shift: Vec2,
    pub phase_shift: Vec2,
}

fn pre_setup(mut map_gen_settings: ResMut<MapGenSettings>) {
    *map_gen_settings = MapGenSettings {
        amplitude: Vec2::new(ROW_AMPLITUDE, COL_AMPLITUDE),
        wavelength: Vec2::new(ROW_WAVELENGTH, COL_WAVELENGTH),
        vertical_shift: Vec2::ZERO,
        phase_shift: Vec2::new(-PI, PI / 2.),
    };
}

#[derive(Resource, Default, Debug)]
pub struct MapGenSettings {
    pub amplitude: Vec2,
    pub wavelength: Vec2,
    pub vertical_shift: Vec2,
    pub phase_shift: Vec2,
}

fn pre_setup(mut map_gen_settings: ResMut<MapGenSettings>) {
    *map_gen_settings = MapGenSettings {
        amplitude: Vec2::new(ROW_AMPLITUDE, COL_AMPLITUDE),
        wavelength: Vec2::new(ROW_WAVELENGTH, COL_WAVELENGTH),
        vertical_shift: Vec2::ZERO,
        phase_shift: Vec2::new(-PI, PI / 2.),
    };
}

pub enum TerrainType {
    FLAT,
    CURVED()
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    map_gen_settings: Res<MapGenSettings>,
) {
    let hover_matl = materials.add(Color::WHITE);
    let ground_matl = materials.add(GROUND_COLOR);
    let map_offset: f32 = (MAP_SIZE as f32) * (1. + GAP) / 2.0;

    for i in 0..MAP_SIZE {
        for j in 0..MAP_SIZE {
            let layer: i32 = generate_layer(i, j, &map_gen_settings);

            // create and save a handle to the mesh.
            let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());

            // render the mesh with the custom texture, and add the marker.
            commands
                .spawn(BlockBundle::new(
                    cube_mesh_handle,
                    ground_matl.clone(),
                    map_offset,
                    IVec3::new(i, j, layer),
                ))
                .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
                .observe(update_material_on::<Pointer<Out>>(ground_matl.clone()))
                .observe(update_block_selection::<Pointer<Down>>());
        }
    }
}

fn generate_layer(x: i32, y: i32, map_gen_settings: &MapGenSettings) -> i32 {
    (map_gen_settings.amplitude.x
        * ops::sin(x as f32 * map_gen_settings.wavelength.x + map_gen_settings.phase_shift.x)
        + map_gen_settings.vertical_shift.x
        + map_gen_settings.amplitude.y
            * ops::cos(y as f32 * map_gen_settings.wavelength.y + map_gen_settings.phase_shift.y)
        + map_gen_settings.vertical_shift.y) as i32
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
