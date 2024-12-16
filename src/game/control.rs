use crate::color::{DARK_BLACK, LIGHT_GRAY, WHITE_COLOR};
use bevy::color::palettes::basic::GRAY;
use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<SelectedTab>();
}

#[derive(Component, Clone)]
struct ButtonColors {
    normal_bg: Color,
    selected_bg: Color,
    normal_text: Color,
    selected_text: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal_bg: Color::WHITE,
            selected_bg: Color::srgb_u8(18, 18, 18),
            normal_text: Color::BLACK,
            selected_text: Color::srgb_u8(223, 223, 223),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
enum ControlTab {
    #[default]
    Normal,
    Candidate,
}

#[derive(Component)]
struct ChangeTab(ControlTab);

#[derive(Resource, Debug, Deref, DerefMut, Default)]
struct SelectedTab(ControlTab);

pub(crate) fn control_board(font: &Handle<Font>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Node {
                margin: UiRect {
                    left: Val::Px(40.0),
                    right: Val::Px(40.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                display: Display::Block,
                ..default()
            },
            BackgroundColor(GRAY.into()),
        ))
        .with_children(|builder| {
            // 切换按钮
            builder
                .spawn((
                    Node {
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::WHITE.into()),
                ))
                .with_children(|builder| {
                    let button_colors = ButtonColors::default();
                    builder
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BackgroundColor(DARK_BLACK),
                            ChangeTab(ControlTab::Normal),
                            BorderColor(WHITE_COLOR),
                        ))
                        .with_child((
                            Text::new("Normal"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(button_colors.selected_text),
                        ));

                    builder
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                ..Default::default()
                            },
                            BackgroundColor(WHITE_COLOR),
                            ChangeTab(ControlTab::Candidate),
                            BorderColor(LIGHT_GRAY),
                        ))
                        .with_child((
                            Text::new("Candidate"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(button_colors.normal_text),
                        ));
                });

            builder.spawn((
                Node {
                    margin: UiRect {
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                        top: Val::Px(20.0),
                        bottom: Val::Px(20.0),
                    },
                    ..default()
                },
                BackgroundColor(Color::BLACK.into()),
            ));
        });
}
