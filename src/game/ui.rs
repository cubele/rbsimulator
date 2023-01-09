use bevy::prelude::*;
use super::consts::*;
use crate::fumen::Fumen;

/// This shit is too annoying so I hardcoded most of it
#[derive(Resource)]
struct BGTexture {
    frame: Handle<TextureAtlas>,
    background: Handle<Image>,
    topslot: Handle<Image>,
    judgeline: Handle<Image>,
}

impl BGTexture {
    // github copilot is cool
    fn parse_frame (asset_server: &AssetServer) -> TextureAtlas {
        let frame_image = asset_server.load("images\\frame_popn.png");
        let mut atlas = TextureAtlas::new_empty(frame_image, Vec2::new(512., 512.));
        // left tophalf
        atlas.add_texture(Rect::new(0., 0., 73., 511.));
        // left bottomhalf
        atlas.add_texture(Rect::new(362., 0., 435., 511.));
        // top lefthalf
        atlas.add_texture(Rect::new(266., 0., 287., 312.));
        // top righthalf
        atlas.add_texture(Rect::new(290., 0., 311., 312.));
        // bottom lefthalf
        atlas.add_texture(Rect::new(314., 0., 335., 312.));
        // bottom righthalf
        atlas.add_texture(Rect::new(338., 0., 359., 312.));
        // right tophalf
        atlas.add_texture(Rect::new(76., 0., 149., 511.));
        // right bottomhalf
        atlas.add_texture(Rect::new(438., 0., 511., 511.));
        atlas
    }
}

fn vec_from_lefttop_and_size(left: f32, top: f32, width: f32, height: f32, z: f32) -> Vec3 {
    Vec3::new(left + width / 2., top - height / 2., z)
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    
    let materials = BGTexture {
        frame: texture_atlas.add(BGTexture::parse_frame(&asset_server)),
        background: asset_server.load("images\\background.png"),
        topslot: asset_server.load("images\\topslot.png"),
        judgeline: asset_server.load("images\\judgeline.png"),
    };
    commands.spawn(SpriteBundle {
        texture: materials.background.clone(),
        transform: Transform::from_translation(Vec3::new(
            0., 0., 0.
        )),
        sprite: Sprite {
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        ..default()
    });
    // notes cover
    commands.spawn(SpriteBundle {
        texture: materials.background.clone(),
        transform: Transform::from_translation(Vec3::new(
            0., WINDOW_HEIGHT / 2., OBJECT_Z + 1.
        )),
        sprite: Sprite {
            custom_size: Some(Vec2::new(WINDOW_WIDTH, (WINDOW_HEIGHT - INNER_WINDOW_HEIGHT + 1.) * 2.)),
            ..default()
        },
        ..default()
    });
    // left frame
    // TODO: for unknown reason an offsets of 1 must be added otherwise the frame will be split
    // This may have something to do with the fact that bevy uses the center instead of the corner as coords
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(0),
        texture_atlas: materials.frame.clone(),
        transform: Transform {
            translation: vec_from_lefttop_and_size(
                INNER_WINDOW_X_MIN, INNER_WINDOW_Y_MAX,
                LEFT_FRAME_FULL_WIDTH, INNER_WINDOW_HEIGHT / 2.,
                FRAME_Z
            ),
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(1),
        texture_atlas: materials.frame.clone(),
        transform:  Transform {
            translation: vec_from_lefttop_and_size(
                INNER_WINDOW_X_MIN, INNER_WINDOW_Y_MAX - INNER_WINDOW_HEIGHT / 2. + 1.,
                LEFT_FRAME_FULL_WIDTH, INNER_WINDOW_HEIGHT / 2.,
                FRAME_Z + 1.
            ),
            ..default()
        },
        ..default()
    });
    // top frame
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(2),
        texture_atlas: materials.frame.clone(),
        transform: Transform {
            translation: vec_from_lefttop_and_size(
                INNER_WINDOW_X_MIN + 74. - 1., INNER_WINDOW_Y_MAX - 4.5,
                313., TOP_FRAME_HEIGHT,
                FRAME_Z
            ),
            ..default()
        }.with_rotation(Quat::from_rotation_z(-std::f32::consts::PI / 2.)),
        ..default()
    });
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(3),
        texture_atlas: materials.frame.clone(),
        transform: Transform {
            translation: vec_from_lefttop_and_size(
                INNER_WINDOW_X_MIN + 74. + 313. - 2., INNER_WINDOW_Y_MAX - 4.5,
                313., TOP_FRAME_HEIGHT,
                FRAME_Z
            ),
            ..default()
        }.with_rotation(Quat::from_rotation_z(-std::f32::consts::PI / 2.)),
        ..default()
    });
    // bottom frame
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(4),
        texture_atlas: materials.frame.clone(),
        transform: Transform {
            translation: vec_from_lefttop_and_size(
                INNER_WINDOW_X_MIN + 74. - 1., INNER_WINDOW_Y_MIN + TOP_FRAME_HEIGHT + 5.,
                313., TOP_FRAME_HEIGHT,
                FRAME_Z
            ),
            ..default()
        }.with_rotation(Quat::from_rotation_z(-std::f32::consts::PI / 2.)),
        ..default()
    });
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(5),
        texture_atlas: materials.frame.clone(),
        transform: Transform {
            translation: vec_from_lefttop_and_size(
                INNER_WINDOW_X_MIN + 74. + 313. - 2., INNER_WINDOW_Y_MIN + TOP_FRAME_HEIGHT + 5.,
                313., TOP_FRAME_HEIGHT,
                FRAME_Z
            ),
            ..default()
        }.with_rotation(Quat::from_rotation_z(-std::f32::consts::PI / 2.)),
        ..default()
    });
    // right frame
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(6),
        texture_atlas: materials.frame.clone(),
        transform: Transform {
            translation: vec_from_lefttop_and_size(
                INNER_WINDOW_X_MAX - RIGHT_FRAME_FULL_WIDTH, INNER_WINDOW_Y_MAX,
                RIGHT_FRAME_FULL_WIDTH, INNER_WINDOW_HEIGHT / 2.,
                FRAME_Z
            ),
            ..default()
        },
        ..default()
    });
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(7),
        texture_atlas: materials.frame.clone(),
        transform: Transform {
            translation: vec_from_lefttop_and_size(
                INNER_WINDOW_X_MAX - RIGHT_FRAME_FULL_WIDTH, INNER_WINDOW_Y_MAX - INNER_WINDOW_HEIGHT / 2. + 1.,
                RIGHT_FRAME_FULL_WIDTH, INNER_WINDOW_HEIGHT / 2.,
                FRAME_Z + 1.
            ),
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
struct InfoText;

const INFO_FIELDS: [&str; 6] = ["Title: ", "Artist: ", "BPM: ", "charter: ", "Difficulty: ", "Level: "];
fn setup_info_text(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let font = asset_server.load("fonts\\MOBO.otf");
    let style = TextStyle {
        font_size: 30.0,
        color: Color::WHITE,
        font: font.clone(),
        ..default()
    };

    // Info text node
    commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(60.),
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
                        sections: {
                            let mut sections = Vec::new();
                            for field in INFO_FIELDS {
                                sections.push(TextSection {
                                    value: field.to_string(),
                                    style: style.clone(),
                                });
                            }
                            sections
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(InfoText);
        });
}

fn update_info_text(
    mut query: Query<&mut Text, With<InfoText>>,
    fumen: Res<Fumen>
) {
    let values = [
        &fumen.metadata.name,
        &fumen.metadata.artist,
        &fumen.metadata.bpm.to_string(),
        &fumen.metadata.charter,
        &fumen.metadata.difficulty,
        &fumen.metadata.level.to_string(),
    ];
    let mut text = query.single_mut();
    for (id, (field, value)) in INFO_FIELDS.iter().zip(values.iter()).enumerate() {
        text.sections[id].value = format!("{}{}\n", field, value);
    }
}

#[derive(Component)]
struct TimeText;

fn setup_time_text(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let font = asset_server.load("fonts\\MOBO.otf");
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
                                font_size: 50.0,
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

// TODO: withRunCriteria
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_startup_system(setup_time_text)
            .add_startup_system(setup_info_text)
            .add_system(update_time_text)
            .add_system(update_info_text);
    }
}