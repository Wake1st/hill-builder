use bevy::{color::palettes::css::{RED, YELLOW}, prelude::*, window::SystemCursorIcon};
use bevy_lunex::{hover_set, Anchor, OnHoverSetCursor, Rh, Rl, UiBase, UiColor, UiFetchFromCamera, UiHover, UiLayout, UiLayoutRoot, UiStateTrait, UiTextSize};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
// Create UI
commands.spawn((
    // Initialize the UI root for 2D
    UiLayoutRoot::new_2d(),
    // Make the UI synchronized with camera viewport size
    UiFetchFromCamera::<0>,
)).with_children(|ui| {

    // Spawn a button in the middle of the screen
    ui.spawn((
        Name::new("My Button"),
        // Specify the position and size of the button
        UiLayout::window().pos(Rl((50.0, 50.0))).size((200.0, 50.0)).pack(),
        // When hovered, it will request the cursor icon to be changed
        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
    )).with_children(|ui| {
        
        // Spawn a child node but with a background
        ui.spawn((
            // You can define layouts for multiple states
            UiLayout::new(vec![
                // The default state, just fill the parent
                (UiBase::id(), UiLayout::window().full()),
                // The hover state, grow to 105% of the parent from center
                (UiHover::id(), UiLayout::window().anchor(Anchor::Center).size(Rl(105.0)))
            ]),
            // Enable the hover state and give it some properties
            UiHover::new().forward_speed(20.0).backward_speed(4.0),
            // You can specify colors for multiple states
            UiColor::new(vec![
                (UiBase::id(), RED.with_alpha(0.15)),
                (UiHover::id(), YELLOW.with_alpha(1.2))
            ]),
            // You can attach any form of rendering to the node, be it sprite, mesh or something custom
            Sprite {
                image: asset_server.load("block.png"),
                // Here we enable sprite slicing
                image_mode: SpriteImageMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),
                ..default()
            },
            // Make sure it does not cover the bounding zone of parent
            PickingBehavior::IGNORE,
        )).with_children(|ui| {

            // Spawn a text child node
            ui.spawn((
                // For text we always use window layout to position it. The size is computed at runtime from text bounds
                UiLayout::window().pos((Rh(40.0), Rl(50.0))).anchor(Anchor::CenterLeft).pack(),
                UiColor::new(vec![
                    (UiBase::id(), RED.with_alpha(0.15)),
                    (UiHover::id(), YELLOW.with_alpha(1.2))
                ]),
                UiHover::new().forward_speed(20.0).backward_speed(4.0),
                // Here we specify the text height proportional to the parent node
                UiTextSize::from(Rh(60.0)),
                // You can attach text like this
                Text2d::new("Click me!"),
                TextFont {
                    // font: asset_server.load("fonts/semibold.ttf"),
                    font_size: 64.0,
                    ..default()
                },
                // Make sure it does not cover the bounding zone of parent
                PickingBehavior::IGNORE,
            ));
        });
    })
    // Utility observers that enable the hover state on trigger
    .observe(hover_set::<Pointer<Over>, true>)
    .observe(hover_set::<Pointer<Out>, false>)
    // Interactivity is done through observers, you can query anything here
    .observe(|_: Trigger<Pointer<Click>>| {
        // ... Do something on click
    });
});
}