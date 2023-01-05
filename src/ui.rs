use bevy::prelude::*;

#[derive(Component)]
struct TimeText;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let font = asset_server.load("fonts\\BLOBBYCHUG.ttf");
    commands
        // Time text node
        .spawn(NodeBundle {
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
                            value: "Time: ".to_string(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
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

fn update_time_text(time: Res<Time>, mut query: Query<&mut Text, With<TimeText>>) {
    let secs = time.elapsed_seconds_f64();

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", secs);
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_system(update_time_text);
    }
}