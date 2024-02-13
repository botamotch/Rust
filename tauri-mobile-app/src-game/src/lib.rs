use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn draw() {
  let resolution = WindowResolution::new(320.0, 480.0);
  let primary_window = Some(Window {
    title: "Mine Sweeper!".to_string(),
    canvas: Some("#canvas1".to_string()),
    resolution,
    ..default()
  });

  App::new()
    .insert_resource(AssetMetaCheck::Never)
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window,
      ..default()
    }))
    .add_plugins(MineSweeper)
    .run();
}

struct MineSweeper;

impl Plugin for MineSweeper {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, camera_setup)
      .add_systems(Startup, create_board);
  }
}

fn camera_setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

fn create_board(mut commands: Commands) {
  commands
    .spawn((GlobalTransform::default(), Transform::from_xyz(0., 0., 1.)))
    .with_children(|parent| {
      parent.spawn((
        Name::new("Background"),
        SpriteBundle {
          sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(10., 10.)),
            ..Default::default()
          },
          transform: Transform::from_xyz(0., 0., 2.),
          ..Default::default()
        },
      ));
    });
}
