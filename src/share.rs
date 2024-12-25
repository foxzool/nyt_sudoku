use crate::loading::FontAssets;
use bevy::prelude::*;

/// 顶部标题栏
pub fn title_bar(font_assets: &Res<FontAssets>, builder: &mut ChildBuilder) {
    builder
        .spawn((
            Name::new("title-wrapper"),
            Node {
                display: Display::Flex,
                height: Val::Px(114.0),
                margin: UiRect::axes(Val::Auto, Val::Px(0.0)),
                padding: UiRect {
                    left: Val::Px(24.0),
                    right: Val::Px(24.0),
                    top: Val::Px(26.0),
                    bottom: Val::Px(20.0),
                },
                max_width: Val::Px(1280.0),
                width: Val::Px(1280.0),
                align_items: AlignItems::Baseline,
                ..default()
            },
            // BackgroundColor(GAME_YELLOW),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Name::new("game-title"),
                    Node {
                        margin: UiRect {
                            // top: Val::Px(10.0),
                            right: Val::Px(16.0),
                            ..default()
                        },
                        // padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                        ..default()
                    },
                    // BackgroundColor(GRAY2),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Sudoku"),
                        TextFont {
                            font_size: 42.0,
                            font: font_assets.karnak.clone(),
                            ..default()
                        },
                        TextColor::BLACK,
                    ));
                });

            builder
                .spawn((
                    Name::new("game-date"),
                    Node {
                        bottom: Val::Px(6.0),
                        // padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                        ..default()
                    },
                    // BackgroundColor(GRAY),
                ))
                .with_children(|p| {
                    let date_str = chrono::Local::now().format("%B %d, %Y").to_string();
                    p.spawn((
                        Text::new(date_str),
                        TextFont {
                            font_size: 28.0,
                            font: font_assets.franklin_500.clone(),
                            ..default()
                        },
                        TextColor::BLACK,
                    ));
                });
        });
}
