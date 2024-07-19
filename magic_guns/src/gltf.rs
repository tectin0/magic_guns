use bevy::{
    asset::{AssetServer, Assets, Handle},
    gltf::Gltf,
    log::debug,
    prelude::{Commands, Res, ResMut, Resource},
    scene::SceneBundle,
    state::state::NextState,
};

use crate::states::GameState;

#[derive(Resource)]
pub struct BaseAssetPack(Handle<Gltf>);

pub fn load_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let gltf = ass.load("gltf/base.glb");
    commands.insert_resource(BaseAssetPack(gltf));

    debug!("Loaded GLTF base");
}

pub fn spawn_gltf_objects(
    mut commands: Commands,
    base_asset_pack: Res<BaseAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    debug!("Trying to spawn scene");

    if let Some(gltf) = assets_gltf.get(&base_asset_pack.0) {
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..Default::default()
        });

        next_state.set(GameState::Playing);

        debug!("Spawned Scene");
    }
}
