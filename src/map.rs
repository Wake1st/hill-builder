use bevy::prelude::*;

use crate::{block::BlockBundle, mesh::create_cube_mesh};

const MAP_SIZE: i32 = 22;
const GAP: f32 = 0.1;

const ROW_AMPLITUDE: f32 = 2.8;
const COL_AMPLITUDE: f32 = 1.4;
const ROW_WAVELENGTH: f32 = 0.2;
const COL_WAVELENGTH: f32 = 0.1;

const GROUND_COLOR: Color = Color::srgb(0.0, 0.9, 0.1);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let offset: f32 = (MAP_SIZE as f32) * (1. + GAP) / 2.0;

    for j in 0..MAP_SIZE {
        for i in 0..MAP_SIZE {
            let layer: i32 = (ROW_AMPLITUDE * ops::sin(i as f32 * ROW_WAVELENGTH)
                + COL_AMPLITUDE * ops::cos(j as f32 * COL_WAVELENGTH))
                as i32;

            // create and save a handle to the mesh.
            let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());

            // render the mesh with the custom texture, and add the marker.
            commands.spawn(BlockBundle::new(
                cube_mesh_handle,
                &mut materials,
                GROUND_COLOR,
                offset,
                IVec3::new(i, j, layer),
            ));
        }
    }
}
