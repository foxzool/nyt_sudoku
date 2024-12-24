use crate::color::{DARK_BLACK, WHITE_COLOR};
use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::WindowFocused;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (check_window_focus, fade_in_animation, fade_out_animation)
            .run_if(in_state(GameState::Playing)),
    )
    .add_observer(on_pause_game);
}

pub(crate) fn dialog_container(font_assets: &Res<FontAssets>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("dialog-container"),
            DialogContainer,
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
            spawn_pause(font_assets, builder);
        });
}

fn spawn_pause(font_assets: &Res<FontAssets>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("pause-container"),
            Visibility::Hidden,
            PauseContainer,
            Node {
                bottom: Val::Px(-30.0),
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
    mut q_pause: Single<(Entity, &mut Visibility), With<PauseContainer>>,
) {
    let (entity, mut visibility) = q_pause.into_inner();
    if ev.event().0 {
        time.pause();
        // *visibility = Visibility::Visible;
        let fade_in = FadeIn(Timer::from_seconds(0.2, TimerMode::Once));
        commands.entity(entity).insert(fade_in);
    } else {
        time.unpause();
        // *visibility = Visibility::Hidden;
        let fade_out = FadeOut(Timer::from_seconds(0.2, TimerMode::Once));
        commands.entity(entity).insert(fade_out);
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
    mut q: Query<(Entity, &mut Node, &mut Visibility, &mut FadeOut)>,
    mut commands: Commands,
) {
    for (entity, mut node, mut visibility, mut fade_out) in &mut q.iter_mut() {
        fade_out.0.tick(time.delta());
        node.bottom = Val::Px(-fade_out.percent() * 60.0);
        if fade_out.0.just_finished() {
            *visibility = Visibility::Hidden;
            commands.entity(entity).remove::<FadeOut>();
        }
    }
}
