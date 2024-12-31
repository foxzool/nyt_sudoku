use crate::color::{DARK_BLACK, WHITE_COLOR};
use crate::game::{GameTimer, ResetPuzzle, Settings, SudokuManager};
use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy::window::WindowFocused;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (check_window_focus, fade_in_animation, fade_out_animation)
            .run_if(in_state(GameState::Playing)),
    )
    .add_observer(on_pause_game)
    .add_observer(on_show_settings)
    .add_observer(on_show_congrats)
    .add_observer(on_hint);
}

pub(crate) fn dialog_container(_font_assets: &Res<FontAssets>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("dialog-container"),
            DialogContainer,
            Visibility::Hidden,
            Node {
                display: Display::Flex,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::NoWrap,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                ..default()
            },
            ZIndex(999),
            // BackgroundColor(RED.into()),
        ))
        .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.trigger(PauseGame(false));
            commands.trigger(ShowHint(false));
            commands.trigger(ShowSettings(false));
        });
}

fn dialog_child_body() -> (Node, BorderRadius, BoxShadow, BackgroundColor) {
    (
        Node {
            // position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            height: Val::Auto,
            width: Val::Px(667.0),
            min_height: Val::Px(332.0),
            padding: UiRect::all(Val::Px(48.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderRadius::all(Val::Px(4.0)),
        BoxShadow {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(3.0),
            spread_radius: Val::Px(-1.0),
            blur_radius: Val::Px(12.0),
        },
        BackgroundColor(WHITE_COLOR),
    )
}

fn spawn_pause(font_assets: &Res<FontAssets>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("pause-container"),
            PauseContainer,
            dialog_child_body(),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Name::new("pause-title"),
                    Node {
                        margin: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("Your game has been paused"),
                        TextFont {
                            font_size: 16.0,
                            font: font_assets.franklin_600.clone(),
                            ..default()
                        },
                        TextColor(*DARK_BLACK),
                    ));
                });

            builder
                .spawn((
                    Name::new("pause-buttons"),
                    Button,
                    Node {
                        display: Display::Flex,
                        width: Val::Auto,
                        margin: UiRect {
                            top: Val::Px(30.0),
                            ..default()
                        },
                        padding: UiRect::horizontal(Val::Px(38.0)),
                        min_height: Val::Px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(40.0)),
                    BackgroundColor(*DARK_BLACK),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("Resume"),
                        TextFont {
                            font_size: 14.0,
                            font: font_assets.franklin_500.clone(),
                            ..default()
                        },
                        TextColor(WHITE_COLOR),
                    ));
                })
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(PauseGame(false));
                    },
                );
        });
}

fn spawn_hint(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
) {
    builder
        .spawn((
            Name::new("hint-container"),
            HintContainer,
            dialog_child_body(),
        ))
        .with_children(|builder| {
            builder.spawn((
                ImageNode {
                    image: texture_assets.close.clone(),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    margin: UiRect::all(Val::Px(20.0)),
                    top: Val::Px(0.0),
                    right: Val::Px(0.0),
                    height: Val::Px(18.0),
                    width: Val::Px(18.0),
                    ..default()
                },
            )).observe(
                |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(ShowHint(false));
                },
            );

            builder
                .spawn((
                    Name::new("hint-content"),
                    Node {
                        display: Display::Flex,
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        // margin: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                    // BackgroundColor(*DARK_BLACK),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("How to play Sudoku"),
                        TextFont {
                            font_size: 28.0,
                            font: font_assets.karnak.clone(),
                            ..default()
                        },
                        TextColor(*DARK_BLACK),
                    ));

                    builder.spawn((
                        Text::new("Fill each 3 x 3 set with numbers 1–9."),
                        TextFont {
                            font_size: 16.0,
                            font: font_assets.franklin_600.clone(),
                            ..default()
                        },
                        TextColor(*DARK_BLACK),
                    ));

                    builder
                        .spawn((
                            Name::new("list"),
                            Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::vertical(Val::Px(16.0)),
                                ..default()
                            },
                            // BackgroundColor(YELLOW.into()),
                        ))
                        .with_children(|builder| {
                            ui_list(font_assets, texture_assets, builder, "Tap a cell in any set, then select a number.");
                            ui_list(font_assets, texture_assets, builder, "Fill cells until the board is complete. Numbers in sets, rows or columns cannot repeat.");
                            ui_list(font_assets, texture_assets, builder, "Note: Each number can only appear on the board 9 times.");
                        });

                    builder.spawn((
                        Text::new("Play modes and tips"),
                        TextFont {
                            font_size: 28.0,
                            font: font_assets.karnak.clone(),
                            ..default()
                        },
                        TextColor(*DARK_BLACK),
                    ));

                    builder
                        .spawn((
                            Name::new("list"),
                            Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::vertical(Val::Px(16.0)),
                                ..default()
                            },
                            // BackgroundColor(YELLOW.into()),
                        ))
                        .with_children(|builder| {
                            ui_list(font_assets, texture_assets, builder, "Normal mode: Add 1 number to a cell.");
                            ui_list(font_assets, texture_assets, builder, "Fill cells until the board is complete. Numbers in sets, rows or columns cannot repeat.");
                            ui_list(font_assets, texture_assets, builder, "Candidate mode: Add several numbers to a cell (for multiple options).");
                            ui_list(font_assets, texture_assets, builder, "Need a clue? Tap -> \"Hint\" to see the next logical cell to solve.");
                            ui_list(font_assets, texture_assets, builder, "Choose from 3 levels — easy, medium and hard. To change levels, tap \"Back\" in the toolbar.");
                            ui_list(font_assets, texture_assets, builder, "New puzzles for each level are released daily: Sunday–Thursday at 10 p.m. E.T.; Friday–Saturday at 6 p.m. E.T.");
                        });


                    builder.spawn((
                        Name::new("hint-feedback"),
                        Node {
                            ..default()
                        },
                    )).with_children(|builder| {
                        builder.spawn((
                            Text::new("Have feedback? "),
                            TextFont {
                                font_size: 16.0,
                                font: font_assets.franklin_600.clone(),
                                ..default()
                            },
                            TextColor(*DARK_BLACK),
                        ));

                        builder.spawn((
                            Text::new("Email us"),
                            TextFont {
                                font_size: 16.0,
                                font: font_assets.franklin_600.clone(),
                                ..default()
                            },
                            TextColor(*DARK_BLACK),
                        ));
                    })
                    ;
                });
        });
}

fn ui_list(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
    text: &str,
) {
    builder
        .spawn(Node {
            display: Display::Flex,
            // align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                ImageNode {
                    image: texture_assets.dot.clone(),
                    ..default()
                },
                Node {
                    margin: UiRect {
                        top: Val::Px(4.0),
                        ..default()
                    },
                    height: Val::Px(18.0),
                    width: Val::Px(18.0),
                    ..default()
                },
            ));

            builder.spawn((
                Text::new(text),
                TextFont {
                    font_size: 16.0,
                    font: font_assets.franklin_600.clone(),
                    ..default()
                },
                TextColor(*DARK_BLACK),
            ));
        });
}

fn check_window_focus(mut windows: EventReader<WindowFocused>, mut commands: Commands) {
    for window in windows.read() {
        if !window.focused {
            commands.trigger(PauseGame(true));
        }
    }
}

#[derive(Event)]
pub struct PauseGame(pub bool);

fn on_pause_game(
    ev: Trigger<PauseGame>,
    mut time: ResMut<Time<Virtual>>,
    mut commands: Commands,
    q_dialog: Single<(Entity, &mut Visibility), With<DialogContainer>>,
    font_assets: Res<FontAssets>,
    q_pause: Query<Entity, With<PauseContainer>>,
    sudoku_manager: Res<SudokuManager>,
) {
    let (entity, mut visibility) = q_dialog.into_inner();
    if ev.event().0 {
        if time.is_paused() || sudoku_manager.solved {
            return;
        }

        time.pause();
        *visibility = Visibility::Visible;
        commands.entity(entity).with_children(|builder| {
            spawn_pause(&font_assets, builder);
        });
    } else {
        time.unpause();
        for pause in q_pause.iter() {
            commands
                .entity(pause)
                .insert(FadeOut(Timer::from_seconds(0.2, TimerMode::Once)));
        }
    }
}

#[derive(Component)]
pub struct DialogContainer;

#[derive(Component)]
pub struct PauseContainer;

#[derive(Component)]
pub struct FadeIn(pub Timer);

impl FadeIn {
    pub fn percent(&self) -> f32 {
        self.0.elapsed_secs() / self.0.duration().as_secs_f32()
    }
}

#[derive(Component)]
pub struct FadeOut(pub Timer);

impl FadeOut {
    pub fn percent(&self) -> f32 {
        self.0.elapsed_secs() / self.0.duration().as_secs_f32()
    }
}

fn fade_in_animation(
    time: Res<Time<Real>>,
    mut q: Query<(Entity, &mut Node, &mut Visibility, &mut FadeIn)>,
    mut commands: Commands,
) {
    for (entity, mut node, mut visibility, mut fade_in) in &mut q.iter_mut() {
        *visibility = Visibility::Visible;
        fade_in.0.tick(time.delta());
        node.bottom = Val::Px(fade_in.percent() * 30.0 - 30.0);
        if fade_in.0.just_finished() {
            node.bottom = Val::Px(0.0);
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn fade_out_animation(
    time: Res<Time<Real>>,
    mut q: Query<(Entity, &mut Node, &mut FadeOut), Without<DialogContainer>>,
    mut q_dialog: Single<&mut Visibility, (With<DialogContainer>, Without<FadeOut>)>,
    mut commands: Commands,
) {
    for (entity, mut node, mut fade_out) in &mut q.iter_mut() {
        fade_out.0.tick(time.delta());
        node.bottom = Val::Px(-fade_out.percent() * 60.0);
        if fade_out.0.just_finished() {
            **q_dialog = Visibility::Hidden;
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Default)]
pub struct Opened(pub bool);

#[derive(Component)]
pub struct HintContainer;

#[derive(Event)]
pub struct ShowHint(pub bool);

#[allow(clippy::too_many_arguments)]
fn on_hint(
    trigger: Trigger<ShowHint>,
    mut time: ResMut<Time<Virtual>>,
    mut commands: Commands,
    q_dialog: Single<(Entity, &mut Visibility), With<DialogContainer>>,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
    q_hint: Query<Entity, With<HintContainer>>,
    mut opened: Local<Opened>,
) {
    let (entity, mut visibility) = q_dialog.into_inner();
    let show_hint = trigger.event().0;
    if show_hint && !opened.0 {
        opened.0 = true;
        time.pause();
        *visibility = Visibility::Visible;
        commands.entity(entity).with_children(|builder| {
            spawn_hint(&font_assets, &texture_assets, builder);
        });
    } else {
        opened.0 = false;
        time.unpause();
        for hint in q_hint.iter() {
            commands
                .entity(hint)
                .insert(FadeOut(Timer::from_seconds(0.2, TimerMode::Once)));
        }
    }
}

#[derive(Event)]
pub struct ShowSettings(pub bool);

#[derive(Component)]
pub struct SettingContainer;

#[allow(clippy::too_many_arguments)]
fn on_show_settings(
    trigger: Trigger<ShowSettings>,
    mut time: ResMut<Time<Virtual>>,
    mut commands: Commands,
    q_dialog: Single<(Entity, &mut Visibility), With<DialogContainer>>,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
    q_setting: Query<Entity, With<SettingContainer>>,
    setting: Res<Settings>,
    mut opened: Local<Opened>,
) {
    let (entity, mut visibility) = q_dialog.into_inner();
    if trigger.event().0 && !opened.0 {
        opened.0 = true;
        time.pause();
        *visibility = Visibility::Visible;
        commands.entity(entity).with_children(|builder| {
            spawn_settings(&font_assets, &texture_assets, builder, &setting);
        });
    } else {
        opened.0 = false;
        time.unpause();
        for hint in q_setting.iter() {
            commands
                .entity(hint)
                .insert(FadeOut(Timer::from_seconds(0.2, TimerMode::Once)));
        }
    }
}

fn spawn_settings(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
    settings: &Res<Settings>,
) {
    builder
        .spawn((
            Name::new("setting-container"),
            SettingContainer,
            dialog_child_body(),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    ImageNode {
                        image: texture_assets.close.clone(),
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        margin: UiRect::all(Val::Px(20.0)),
                        top: Val::Px(0.0),
                        right: Val::Px(0.0),
                        height: Val::Px(18.0),
                        width: Val::Px(18.0),
                        ..default()
                    },
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(ShowSettings(false));
                    },
                );

            builder
                .spawn((
                    Name::new("setting-content"),
                    Node {
                        display: Display::Flex,
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        // margin: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                    // BackgroundColor(*DARK_BLACK),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("Settings"),
                        TextFont {
                            font_size: 28.0,
                            font: font_assets.karnak.clone(),
                            ..default()
                        },
                        TextColor(*DARK_BLACK),
                        Node {
                            margin: UiRect {
                                bottom: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        },
                    ));

                    setting_item(
                        font_assets,
                        texture_assets,
                        builder,
                        "Check guesses when entered",
                        settings.check_guesses_when_entered,
                        |_trigger, mut settings| {
                            settings.check_guesses_when_entered =
                                !settings.check_guesses_when_entered;
                        },
                    );
                    setting_item(
                        font_assets,
                        texture_assets,
                        builder,
                        "Start in automatic mode",
                        settings.start_in_automatic_mode,
                        |_trigger, mut settings| {
                            settings.start_in_automatic_mode = !settings.start_in_automatic_mode;
                        },
                    );
                    setting_item(
                        font_assets,
                        texture_assets,
                        builder,
                        "Highlight conflicts",
                        settings.highlight_conflicts,
                        |_trigger, mut settings| {
                            settings.highlight_conflicts = !settings.highlight_conflicts;
                        },
                    );
                    setting_item(
                        font_assets,
                        texture_assets,
                        builder,
                        "Play sound on solve",
                        settings.play_sound_on_solve,
                        |_trigger, mut settings| {
                            settings.play_sound_on_solve = !settings.play_sound_on_solve;
                        },
                    );
                    setting_item(
                        font_assets,
                        texture_assets,
                        builder,
                        "Show clock",
                        settings.show_clock,
                        |_trigger, mut settings| {
                            settings.show_clock = !settings.show_clock;
                        },
                    );
                });
        });
}

fn setting_item(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
    text: &str,
    checked: bool,
    change_setting: fn(Trigger<Pointer<Click>>, settings: ResMut<Settings>),
) {
    builder
        .spawn((
            Name::new("text"),
            Node {
                margin: UiRect {
                    top: Val::Px(10.0),
                    ..default()
                },
                display: Display::Flex,
                align_items: AlignItems::Center,
                ..default()
            },
            CheckOption(checked),
        ))
        .observe(click_setting_option)
        .observe(change_setting)
        .with_children(|builder| {
            builder.spawn((
                if checked {
                    ImageNode::new(texture_assets.check.clone())
                } else {
                    ImageNode::new(texture_assets.blank_check.clone())
                },
                Node {
                    width: Val::Px(13.0),
                    height: Val::Px(13.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ));

            builder.spawn((
                Text::new(text),
                TextFont {
                    font: font_assets.franklin_600.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(*DARK_BLACK),
                Node {
                    margin: UiRect {
                        left: Val::Px(18.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

#[derive(Component)]
pub struct CheckOption(pub bool);

fn click_setting_option(
    trigger: Trigger<Pointer<Click>>,
    mut q_option: Query<(&mut CheckOption, &Children)>,
    mut q_image: Query<&mut ImageNode>,
    texture_assets: Res<TextureAssets>,
) {
    if let Ok((mut checked, children)) = q_option.get_mut(trigger.entity()) {
        checked.0 = !checked.0;
        for child in children {
            if let Ok(mut image) = q_image.get_mut(*child) {
                if checked.0 {
                    image.image = texture_assets.check.clone();
                } else {
                    image.image = texture_assets.blank_check.clone();
                }
            }
        }
    }
}

#[derive(Event)]
pub struct ShowCongrats(pub bool);

#[derive(Component)]
pub struct CongratsContainer;

fn on_show_congrats(
    trigger: Trigger<ShowCongrats>,
    mut commands: Commands,
    q_dialog: Single<(Entity, &mut Visibility), With<DialogContainer>>,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
    q_congrats: Query<Entity, With<CongratsContainer>>,
    game_timer: Res<GameTimer>,
) {
    let (entity, mut visibility) = q_dialog.into_inner();
    if trigger.event().0 {
        *visibility = Visibility::Visible;
        commands.entity(entity).with_children(|builder| {
            spawn_congrats(&font_assets, &texture_assets, builder, game_timer.clone());
        });
    } else {
        for congrats in q_congrats.iter() {
            commands
                .entity(congrats)
                .insert(FadeOut(Timer::from_seconds(0.2, TimerMode::Once)));
        }
    }
}

fn spawn_congrats(
    font_assets: &Res<FontAssets>,
    texture_assets: &Res<TextureAssets>,
    builder: &mut ChildBuilder,
    timer: GameTimer,
) {
    builder
        .spawn((
            Name::new("congrats-container"),
            CongratsContainer,
            dialog_child_body(),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    ImageNode {
                        image: texture_assets.close.clone(),
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        margin: UiRect::all(Val::Px(20.0)),
                        top: Val::Px(0.0),
                        right: Val::Px(0.0),
                        height: Val::Px(18.0),
                        width: Val::Px(18.0),
                        ..default()
                    },
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(ShowCongrats(false));
                    },
                );

            builder.spawn((
                Name::new("congrats-star"),
                ImageNode {
                    image: texture_assets.congrats_star.clone(),
                    ..default()
                },
                Node {
                    width: Val::Px(55.0),
                    height: Val::Px(55.0),
                    margin: UiRect {
                        bottom: Val::Px(16.0),
                        ..default()
                    },
                    ..default()
                },
            ));

            builder.spawn((
                Node {
                    margin: UiRect {
                        top: Val::Px(18.0),
                        ..default()
                    },
                    height: Val::Px(20.0),
                    ..default()
                },
                Text::new(format!("You finished a Easy puzzle in {}", timer)),
                TextColor(*DARK_BLACK),
                TextLayout::default(),
                TextFont {
                    font: font_assets.franklin_600.clone(),
                    font_size: 16.0,
                    ..default()
                },
            ));

            builder
                .spawn((
                    Name::new("replay-button"),
                    Button,
                    Node {
                        display: Display::Flex,
                        width: Val::Auto,
                        margin: UiRect {
                            top: Val::Px(30.0),
                            ..default()
                        },
                        padding: UiRect::horizontal(Val::Px(38.0)),
                        min_height: Val::Px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(40.0)),
                    BackgroundColor(*DARK_BLACK),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("Play another Sudoku"),
                        TextFont {
                            font_size: 14.0,
                            font: font_assets.franklin_500.clone(),
                            ..default()
                        },
                        TextColor(WHITE_COLOR),
                    ));
                })
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.trigger(ResetPuzzle);
                        commands.trigger(ShowCongrats(false));
                    },
                );
        });
}
