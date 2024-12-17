use crate::color::{DARK_BLACK, DARK_GRAY, LIGHT_GRAY, WHITE_COLOR};
use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<SelectedTab>().add_systems(
        Update,
        update_control_tab.run_if(resource_changed::<SelectedTab>),
    );
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
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                max_width: Val::Px(240.0),
                display: Display::Block,
                ..default()
            },
            // BackgroundColor(GRAY.into()),
        ))
        .with_children(|builder| {
            // keyboard
            builder
                .spawn(Node {
                    width: Val::Px(240.0),

                    ..default()
                })
                .with_children(|builder| {
                    // 切换按钮
                    builder
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(0.0)),
                                padding: UiRect::axes(Val::Px(6.0), Val::Px(1.0)),
                                ..Default::default()
                            },
                            BackgroundColor(*DARK_BLACK),
                            ChangeTab(ControlTab::Normal),
                            BorderRadius::left(Val::Px(3.0)),
                            // BorderColor(WHITE_COLOR),
                        ))
                        .with_child((
                            Text::new("Normal"),
                            TextFont {
                                font: font.clone(),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(WHITE_COLOR),
                        ))
                        .observe(
                            |trigger: Trigger<Pointer<Click>>,
                             mut selected_tab: ResMut<SelectedTab>| {
                                selected_tab.0 = ControlTab::Normal;
                            },
                        );

                    builder
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(140.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                padding: UiRect::axes(Val::Px(6.0), Val::Px(1.0)),
                                ..Default::default()
                            },
                            BackgroundColor(WHITE_COLOR),
                            ChangeTab(ControlTab::Candidate),
                            BorderRadius::right(Val::Px(3.0)),
                            BorderColor(*LIGHT_GRAY),
                        ))
                        .with_child((
                            Text::new("Candidate"),
                            TextFont {
                                font: font.clone(),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(*DARK_GRAY),
                        ))
                        .observe(
                            |trigger: Trigger<Pointer<Click>>,
                             mut selected_tab: ResMut<SelectedTab>| {
                                selected_tab.0 = ControlTab::Candidate;
                            },
                        );
                });
        });
}

fn update_control_tab(
    selected_tab: Res<SelectedTab>,
    // mut normal_tab: Single<(&mut Node), (With<NormalTab>, Without<CandidateTab>)>,
    // mut candidate_tab: Single<(&mut Node), (Without<NormalTab>, With<CandidateTab>)>,
    mut tab_query: Query<(
        &ChangeTab,
        &mut Node,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    )>,
    mut text_color: Query<&mut TextColor>,
) {
    // let (mut normal_node) = normal_tab.into_inner();
    // let (mut candidate_node) = candidate_tab.into_inner();

    for (change_tab, mut node, mut bg, mut border_color, children) in tab_query.iter_mut() {
        if change_tab.0 == selected_tab.0 {
            bg.0 = *DARK_BLACK;
            border_color.0 = WHITE_COLOR;
            for child in children {
                if let Ok(mut text_color) = text_color.get_mut(*child) {
                    text_color.0 = WHITE_COLOR;
                }
            }
        } else {
            bg.0 = WHITE_COLOR;
            border_color.0 = *LIGHT_GRAY;
            for child in children {
                if let Ok(mut text_color) = text_color.get_mut(*child) {
                    text_color.0 = *DARK_GRAY;
                }
            }
        }

        // normal tab selected
        if selected_tab.0 == ControlTab::Normal {
            if change_tab.0 == ControlTab::Normal {
                node.border = UiRect::all(Val::Px(0.0));
            } else {
                node.border = UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(1.0),
                    top: Val::Px(1.0),
                    bottom: Val::Px(1.0),
                }
            }
        } else {
            if change_tab.0 == ControlTab::Candidate {
                node.border = UiRect::all(Val::Px(0.0));
            } else {
                node.border = UiRect {
                    left: Val::Px(1.0),
                    right: Val::Px(0.0),
                    top: Val::Px(1.0),
                    bottom: Val::Px(1.0),
                }
            }
        }
    }
}
