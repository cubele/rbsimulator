use bevy::prelude::*;
use crate::consts::*;
use crate::fumen::Fumen;

#[derive(Resource)]
struct BGTexture {
    frame: Handle<Image>,
    topslot: Handle<Image>,
    judgeline: Handle<Image>,
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let materials = BGTexture {
        frame: asset_server.load("images\\frame.png"),
        topslot: asset_server.load("images\\topslot.png"),
        judgeline: asset_server.load("images\\judgeline.png"),
    };
    commands.spawn(SpriteBundle {
        texture: materials.frame.clone(),
        transform: Transform::from_translation(Vec3::new(
            0., 0., 0.
        )),
        sprite: Sprite {
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        ..default()
    });
    for i in 0..TOP_SLOT_COUNT {
        commands.spawn(SpriteBundle {
            texture: materials.topslot.clone(),
            transform: Transform::from_translation(Vec3::new(
                TOP_SLOT_START_X + TOP_SLOT_SPACING * i as f32, TOP_SLOT_Y, 1.
            )),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TOP_SLOT_SIZE, TOP_SLOT_SIZE)),
                ..default()
            },
            ..default()
        });
    }
    commands.spawn(SpriteBundle {
        texture: materials.judgeline.clone(),
        transform: Transform::from_translation(Vec3::new(
            0., JUDGE_LINE_POSITION, 1.
        )),
        sprite: Sprite {
            custom_size: Some(Vec2::new(JUDGE_LINE_WIDTH, JUDGE_LINE_HEIGHT)),
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct TimeText;

fn setup_time_text(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let font = asset_server.load("fonts\\BLOBBYCHUG.ttf");
    // Time text node
    commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Measure: ".to_string(),
                            style: TextStyle {
                                font_size: 30.0,
                                color: Color::GOLD,
                                font: font.clone(),
                                ..default()
                            },
                        }],
                        ..default()
                    },
                    ..default()
                })
                .insert(TimeText);
        });
}

fn update_time_text(time: Res<Time>,
                    mut query: Query<&mut Text, With<TimeText>>,
                    fumen: Res<Fumen>) {
    let meas = (time.elapsed_seconds_f64() - fumen.song_start_time - fumen.delay) / fumen.seconds_per_measure;

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Measure: {}", f64::floor(meas));
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_startup_system(setup_time_text)
            .add_system(update_time_text);
    }
}