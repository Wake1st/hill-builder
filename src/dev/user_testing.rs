use bevy::prelude::*;

use crate::{
    grid::{GridCell, CELL_HEIGHT},
    ground::Ground,
    mesh::{create_cube_mesh, CubeBundle},
    neighborhood::Neighborhood,
    pair::Pair,
    water::{ShiftWater, Water, WATER_COLOR, WATER_MESH_SCALE},
};

pub const FILL_KEY: KeyCode = KeyCode::Tab;

pub struct UserTestingPlugin;

impl Plugin for UserTestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateWater>();
        app.insert_resource(WaterToggle(false));
        app.add_systems(Update, (toggle_water, create_water));
    }
}

#[derive(Resource)]
pub struct WaterToggle(pub bool);

#[derive(Event, Debug)]
pub struct WaterToggled;

pub fn toggle_water(
    keys: Res<ButtonInput<KeyCode>>,
    mut toggle: ResMut<WaterToggle>,
    mut toggled: EventWriter<WaterToggled>,
) {
    if keys.just_pressed(FILL_KEY) {
        toggle.0 = !toggle.0;
        toggled.send(WaterToggled);
    }
}

/// An observer that runs the selection event for water
pub fn update_water_selection<E>() -> impl Fn(Trigger<E>, EventWriter<ShiftWater>) {
    move |trigger, mut selection| {
        selection.send(ShiftWater {
            entity: trigger.entity(),
            upward: true,
        });
    }
}

#[derive(Event)]
pub struct CreateWater {
    pub ground: Entity,
}

fn create_water(
    mut event: EventReader<CreateWater>,
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
        // info!("spawned: {:?}", water_entity);

        commands.spawn((
            Name::new("Pair"),
            Pair {
                ground: ground_entity,
                water: water_entity,
            },
        ));
    }
}
