use bevy::{
    asset::{AssetServer, Assets, Handle},
    gltf::Gltf,
    log::debug,
    math::Vec3,
    prelude::{Commands, Res, ResMut, Resource},
    scene::SceneBundle,
    state::state::NextState,
    transform::components::Transform,
    utils::HashSet,
};

use bevy_rapier3d::prelude::*;

use crate::{player::Player, states::GameState};

#[derive(Resource)]
pub struct WorldAsset(Handle<Gltf>);

#[derive(Resource)]
pub struct MainCharacter(Handle<Gltf>);

#[derive(Resource)]
pub struct WaitForResource(pub HashSet<String>);

pub fn load_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let mut wait_for_resource = HashSet::new();

    let gltf = ass.load("gltf/World.glb");
    commands.insert_resource(WorldAsset(gltf));
    wait_for_resource.insert("World".to_string());

    debug!("Loaded GLTF World");

    let gltf = ass.load("gltf/library/MainCharacter.glb");
    commands.insert_resource(MainCharacter(gltf));
    wait_for_resource.insert("MainCharacter".to_string());

    debug!("Loaded Main Character");

    commands.insert_resource(WaitForResource(wait_for_resource));
}

pub fn spawn_gltf_objects(
    mut commands: Commands,
    world_asset: Res<WorldAsset>,
    main_character_asset: Res<MainCharacter>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut wait_for_resource: ResMut<WaitForResource>,
) {
    debug!("Trying to spawn scenes");

    if let Some(gltf) = assets_gltf.get(&world_asset.0) {
        commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                ..Default::default()
            },
            AsyncSceneCollider {
                shape: Some(ComputedColliderShape::ConvexHull),
                ..Default::default()
            },
        ));

        debug!("Spawned World");

        wait_for_resource.0.remove("World");
    }

    if let Some(gltf) = assets_gltf.get(&main_character_asset.0) {
        commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 3.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::Dynamic,
            AsyncSceneCollider {
                shape: Some(ComputedColliderShape::ConvexHull),
                ..Default::default()
            },
            Restitution::coefficient(0.7),
            Player,
        ));

        debug!("Spawned Main Character");

        wait_for_resource.0.remove("MainCharacter");
    }

    if wait_for_resource.0.is_empty() {
        next_state.set(GameState::Playing);
    }
}
