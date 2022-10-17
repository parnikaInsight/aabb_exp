use bevy::{prelude::*, render::primitives::Aabb,};

fn main() {
    println!("Hello, world!");
    let mut app = bevy::app::App::new(); 
    app.add_plugins(DefaultPlugins);
    app
    .add_startup_system(setup)
    .add_system(sizer)
    .run();


}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_handle2: Handle<Scene> = asset_server.load("tiger.glb#Scene0");
    commands.spawn_bundle(SceneBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            //scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        scene: player_handle2.clone(),
        ..default()
    });

    // Light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    let translation = Vec3::new(-2.0, 2.5, 5.0);
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
}

// with aabb
pub fn sizer(
    mut visible_aabb_query: Query<(Entity, &Aabb, &GlobalTransform)>,
) {
    visible_aabb_query.par_for_each_mut(1024, |(entity, model_aabb, transform)| {
        let model = transform.compute_matrix(); // model
        let world_center = model.transform_point3(Vec3::from(model_aabb.center)); // center of aabb in world space
        let world_half_extents = model.transform_point3(Vec3::from(model_aabb.half_extents)); // half-extents of aabb in world space
        println!("world_center: {:?}", world_center);
        println!("world_half_extents: {:?}", world_half_extents);
    });
}