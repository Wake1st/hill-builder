use bevy::prelude::*;

use crate::{cursor::StoreCursor, mesh::create_cube_mesh};

const GROUND_COLOR: Color = Color::srgb(0.0, 0.9, 0.1);
const MAP_SIZE: i32 = 4;
const GAP: f32 = 0.2;

const SHIFT_RATE: f32 = 1.4;
const SHIFT_AMOUNT: f32 = 0.5;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShiftFinished>()
            .add_systems(Startup, setup)
            .add_systems(Update, (read_position, shift_blocks));
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let offset: f32 = (MAP_SIZE as f32) * GAP / 2.0;

    for j in 0..MAP_SIZE {
        for i in 0..MAP_SIZE {
            // Create and save a handle to the mesh.
            let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());
        
            // Render the mesh with the custom texture, and add the marker.
            commands.spawn((
                Mesh3d(cube_mesh_handle),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: GROUND_COLOR,
                    ..default()
                })),
                Transform::from_xyz((i as f32)*(1.0 + GAP) - offset, 0.0, (j as f32)*(1.0 + GAP) - offset),
                Block
            ));
        }
    }

    // Text to describe the controls.
    commands.spawn((
        Text::new("Left click a block to pull it up; right click a block to push it down."),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct Shifting {
    pub remainder: f32,
    pub up: bool,
}

impl Shifting {
    fn set_direction(up: bool) -> Self {
        if up {
            Self { remainder: SHIFT_AMOUNT, up: true }
        } else {
            Self { remainder: SHIFT_AMOUNT, up: false }
        }
    }
}

fn read_position(
    mut cursor_position: EventReader<StoreCursor>,
    mut blocks: Query<(Entity, &GlobalTransform), With<Block>>,
    mut commands: Commands,
) {
    for cursor in cursor_position.read() {
        for (entity, transform) in blocks.iter_mut() {
            let rect = Rect::from_center_half_size(
                transform.translation().xz(), 
                Vec2::new(0.5, 0.5)
            );
            if rect.contains(cursor.position) {
                // info!("rect: {:?}\tcursor: {:?}", rect, cursor.position);
                commands.entity(entity).insert(Shifting::set_direction(cursor.pull));
                break;
            }
        }
    }
}

#[derive(Event)]
pub struct ShiftFinished {
    pub position: Vec2,
}

fn shift_blocks(
    mut shifters: Query<(Entity, &mut Transform, &mut Shifting)>,
    time: Res<Time>,
    mut shift_finished: EventWriter<ShiftFinished>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut shifting) in shifters.iter_mut() {
        //  calculate shift and check finish
        let mut delta = SHIFT_RATE * time.delta_secs();
        shifting.remainder -= delta;
        if shifting.remainder < 0.0 {
            //  send event to check neighbors
            shift_finished.send(ShiftFinished { position: transform.translation.xz() });

            //  remove the shifting component
            commands.entity(entity).remove::<Shifting>();

            //  the remainder would be a small overlap
            delta -= shifting.remainder;
        }

        //  move cube
        transform.translation.y += delta * if shifting.up { 1.0 } else { -1.0 };
    }
}
