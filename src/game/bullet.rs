use bevy::{
    prelude::*, 
    window::PrimaryWindow
};
use crate::AppState;

const BULLET_SPEED: f32 = 800.0;
pub const BULLET_SIZE: Vec2 = Vec2::new(6.0, 22.0);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BulletSprite>()
            .add_event::<BulletShotEvent>()
            .add_systems(OnEnter(AppState::InGame), load_resources)
            .add_systems(OnExit(AppState::InGame), destroy_all_bullets)
            .add_systems(Update, (
                spawn_bullet,
                bullet_movement,
                destroy_bullets,
            ).run_if(in_state(AppState::InGame)))
        ;
    }
}

#[derive(Component)]
pub struct Bullet {
    pub instigator: Instigator,
    direction: Vec2,
}

#[derive(Resource, Default)]
struct BulletSprite(Handle<Image>);

#[derive(Event)]
pub struct BulletShotEvent {
    pub instigator: Instigator,
    pub positon: Vec2,
    pub direction: Vec2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Instigator {
    Player,
    Enemy,
}

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(BulletSprite(
        asset_server.load("sprites/bullet.png")
    ));
}

fn spawn_bullet(
    mut commands: Commands,
    mut bullet_shot_event_reader: EventReader<BulletShotEvent>,
    bullet_sprite: Res<BulletSprite>,
) {
    for shot_event in bullet_shot_event_reader.read() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(shot_event.positon.extend(0.0)),
                texture: bullet_sprite.0.clone(),
                ..default()
            },
            Bullet {
                instigator: shot_event.instigator,
                direction: shot_event.direction,
            },
        ));
    }
}

fn bullet_movement(
    mut bullet_query: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut bullet_transform, bullet) in bullet_query.iter_mut() {
        bullet_transform.translation += bullet.direction.extend(0.0) * BULLET_SPEED * time.delta_seconds();
    }
}

fn destroy_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let bullet_translation = bullet_transform.translation;
        if bullet_translation.y > window.height() + 100.0 || bullet_translation.y < -100.0 {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn destroy_all_bullets(
    mut commands: Commands,
    bullet_query: Query<Entity, With<Bullet>>,
) {
    for bullet_entity in bullet_query.iter() {
        commands.entity(bullet_entity).despawn();
    } 
}