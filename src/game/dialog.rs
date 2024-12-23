use crate::color::{DARK_BLACK, WHITE_COLOR};
use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::WindowFocused;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (check_window_focus,).run_if(in_state(GameState::Playing)),
    )
    .add_observer(on_pause_game);
}

pub(crate) fn dialog_container(font_assets: &Res<FontAssets>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("dialog-container"),
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
        .with_children(|builder| {
            builder
                .spawn((
                    Name::new("pause-container"),
                    Visibility::Hidden,
                    PauseContainer,
                    Node {
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
    mut q_pause: Single<&mut Visibility, With<PauseContainer>>,
) {
    if ev.event().0 {
        time.pause();
        **q_pause = Visibility::Visible;
    } else {
        time.unpause();
        **q_pause = Visibility::Hidden;
    }
}

#[derive(Component)]
pub struct PauseContainer;
