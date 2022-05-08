use crate::prelude::*;

const TITLE_FONT: &str = "font/Eordeoghlakat.ttf";
const MENU_FONT: &str = "font/FiraCode-Regular.ttf";

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::Menu(StartMenu), start_menu)
            .add_system(start_menu_controls.run_in_state(AppState::Menu(StartMenu)))
            .add_exit_system(AppState::Menu(StartMenu), despawn_with::<StartMenuTag>);
    }
}

#[derive(Component)]
pub struct StartMenuTag {}

fn start_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // title text
            parent.spawn_bundle(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text::with_section(
                    "ARACHNOPHOBIC",
                    TextStyle {
                        font: asset_server.load(TITLE_FONT),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                    TextAlignment::default(),
                ),
                ..Default::default()
            });
            // start button
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(300.0), Val::Px(75.0)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|button| {
                    button.spawn_bundle(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "START",
                            TextStyle {
                                font: asset_server.load(MENU_FONT),
                                font_size: 50.0,
                                color: Color::WHITE,
                            },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                });
        })
        .insert(StartMenuTag {});
}

fn start_menu_controls(mut commands: Commands, input: Res<Input<KeyCode>>) {
    for input in input.get_just_pressed() {
        commands.insert_resource(NextState(AppState::Game(InGame)));
        break;
    }
}
