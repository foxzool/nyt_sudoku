use crate::color::DARK_BLACK;
use crate::loading::{FontAssets, TextureAssets};
use crate::share::title_bar;
use crate::GameState;
use bevy::prelude::*;
use bevy::winit::cursor::CustomCursor::Image;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>, font_assets: Res<FontAssets>) {
    info!("menu");
    commands.spawn((Camera2d, Msaa::Off, Menu));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            title_bar(&font_assets, children);
            children
                .spawn((
                    Name::new("menu-container"),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Vh(90.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        flex_wrap: FlexWrap::Wrap,
                        // height: Val::Percent(100.),
                        padding: UiRect::horizontal(Val::Px(15.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb_u8(251, 155, 0)),
                ))
                .with_children(|children| {
                    children
                        .spawn((
                            Name::new("menu-wrapper"),
                            Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                flex_wrap: FlexWrap::Wrap,
                                height: Val::Percent(100.),
                                padding: UiRect::axes(Val::Px(15.0), Val::Px(30.0)),
                                ..default()
                            },
                        ))
                        .with_children(|children| {
                            children.spawn((
                                ImageNode::from(textures.logo.clone()),
                                Node {
                                    width: Val::Px(64.0),
                                    height: Val::Px(64.0),
                                    margin: UiRect {
                                        bottom: Val::Px(12.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                // BackgroundColor(Color::WHITE),
                            ));

                            children.spawn((
                                Text::new("Sudoku"),
                                TextFont {
                                    font_size: 48.0,
                                    font: font_assets.karnak.clone(),
                                    ..default()
                                },
                                TextColor::BLACK,
                            ));

                            children.spawn((
                                Text::new("Try this numbers game,"),
                                TextFont {
                                    font_size: 36.0,
                                    font: font_assets.karnak_500.clone(),
                                    ..default()
                                },
                                TextColor::BLACK,
                            ));
                            children.spawn((
                                Text::new("minus the math."),
                                TextFont {
                                    font_size: 36.0,
                                    font: font_assets.karnak_500.clone(),
                                    ..default()
                                },
                                TextColor::BLACK,
                                Node {
                                    margin: UiRect {
                                        bottom: Val::Px(36.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));

                            children.spawn((
                                Text::new("Choose Your Puzzle:"),
                                TextFont {
                                    font_size: 16.0,
                                    font: font_assets.franklin_700.clone(),
                                    ..default()
                                },
                                TextColor::BLACK,
                                Node {
                                    margin: UiRect {
                                        bottom: Val::Px(24.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));

                            button_item(&font_assets, children, "Easy");
                            button_item(&font_assets, children, "Medium");
                            button_item(&font_assets, children, "Hard");

                            let date_str = chrono::Local::now().format("%B %d, %Y").to_string();
                            children.spawn((
                                Text::new(date_str),
                                TextFont {
                                    font_size: 16.0,
                                    font: font_assets.franklin_700.clone(),
                                    ..default()
                                },
                                TextColor::BLACK,
                                Node {
                                    margin: UiRect {
                                        top: Val::Px(24.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });
                });
        });
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(5.),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Made with Bevy"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode {
                            image: textures.bevy.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::linear_rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/foxzool/bevy_sudoku"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Open source"),
                        TextFont {
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
                    ));
                    parent.spawn((
                        ImageNode::new(textures.github.clone()),
                        Node {
                            width: Val::Px(32.),
                            ..default()
                        },
                    ));
                });
        });
}

fn button_item(font_assets: &Res<FontAssets>, children: &mut ChildBuilder, text: &str) {
    let button_colors = ButtonColors {
        normal: *DARK_BLACK,
        hovered: *DARK_BLACK,
    };
    children
        .spawn((
            Button,
            Node {
                height: Val::Px(44.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Px(10.0),
                    bottom: Val::Px(8.0),
                    ..default()
                },
                padding: UiRect::axes(Val::Px(16.0), Val::Px(1.0)),
                min_width: Val::Px(150.0),
                ..Default::default()
            },
            BorderRadius::all(Val::Px(24.0)),
            // BackgroundColor(button_colors.normal),
            button_colors,
            ChangeState(GameState::Playing),
        ))
        .with_child((
            Text::new(text),
            TextFont {
                font_size: 16.0,
                font: font_assets.franklin_600.clone(),
                ..default()
            },
            TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
        ));
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
