use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the game to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<TextureAssets>()
                .load_collection::<FontAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(path = "textures/pause.png")]
    pub pause: Handle<Image>,
    #[asset(path = "textures/back.png")]
    pub back: Handle<Image>,
    #[asset(path = "textures/question.png")]
    pub question: Handle<Image>,
    #[asset(path = "textures/more.png")]
    pub more: Handle<Image>,
    #[asset(path = "textures/setting.png")]
    pub setting: Handle<Image>,
    #[asset(path = "textures/circle.png")]
    pub circle: Handle<Image>,
    #[asset(path = "textures/close.png")]
    pub close: Handle<Image>,
    #[asset(path = "textures/dot.png")]
    pub dot: Handle<Image>,
    #[asset(path = "textures/blank-check-box.png")]
    pub blank_check: Handle<Image>,
    #[asset(path = "textures/check.png")]
    pub check: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/franklin-normal-500.ttf")]
    pub franklin_500: Handle<Font>,
    #[asset(path = "fonts/franklin-normal-600.ttf")]
    pub franklin_600: Handle<Font>,
    #[asset(path = "fonts/franklin-normal-700.ttf")]
    pub franklin_700: Handle<Font>,
    #[asset(path = "fonts/franklin-normal-800.ttf")]
    pub franklin_800: Handle<Font>,
    #[asset(path = "fonts/NYTKarnakCondensed.ttf")]
    pub karnak: Handle<Font>,
}
