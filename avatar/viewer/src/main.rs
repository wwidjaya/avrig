use bevy::{
    animation::graph::AnimationGraph,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::WindowResolution,
    world_serialization::WorldInstanceReady,
};

const DEFAULT_MODEL: &str = "avatars/bulb.glb";

fn main() {
    let model_path =
        std::env::var("AVRIG_AVATAR_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_owned());

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.055, 0.075, 0.11)))
        .insert_resource(ViewerModelPath(model_path))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "avrig-ai · Bulb model viewer".into(),
                resolution: WindowResolution::new(1280, 900),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (load_avatar, setup_scene, setup_hud))
        .add_systems(Update, (orbit_camera, idle_float))
        .run();
}

#[derive(Resource)]
struct ViewerModelPath(String);

#[derive(Component)]
struct AvatarScene;

#[derive(Component)]
struct AnimationToPlay {
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}

#[derive(Component)]
struct OrbitCamera {
    target: Vec3,
    yaw: f32,
    pitch: f32,
    distance: f32,
    auto_rotate: bool,
}

fn load_avatar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    model_path: Res<ViewerModelPath>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("Loading PET model from assets/{}", model_path.0);

    // The first animation in the GLB should be the looping Idle clip.
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(model_path.0.clone())),
    );
    let graph_handle = graphs.add(graph);

    commands
        .spawn((
            AnimationToPlay { graph_handle, index },
            WorldAssetRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(model_path.0.clone())),
            ),
            AvatarScene,
            Transform::default(),
        ))
        .observe(play_idle_when_ready);
}

fn play_idle_when_ready(
    scene_ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let Ok(animation) = animations.get(scene_ready.entity) else {
        return;
    };

    for child in children.iter_descendants(scene_ready.entity) {
        if let Ok(mut player) = players.get_mut(child) {
            player.play(animation.index).repeat();
            commands.entity(child).insert(AnimationGraphHandle(
                animation.graph_handle.clone(),
            ));
        }
    }
}

fn setup_scene(mut commands: Commands, model_path: Res<ViewerModelPath>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 4.2).looking_at(Vec3::new(0.0, 0.9, 0.0), Vec3::Y),
        OrbitCamera {
            target: Vec3::new(0.0, 0.9, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            distance: 4.2,
            auto_rotate: false,
        },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 9_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 5.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 220_000.0,
            range: 12.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(2.0, 2.5, 2.0),
    ));

    // Keep the viewer honest when the reviewed GLB has not been supplied yet.
    info!(
        "Expected model: avatar/viewer/assets/{} (override with AVRIG_AVATAR_MODEL)",
        model_path.0
    );
}

fn setup_hud(mut commands: Commands, model_path: Res<ViewerModelPath>) {
    commands.spawn((
        Text::new(format!(
            "AVRIG · PET MODEL REVIEW\n             Model: {}\n             Drag: orbit   Scroll: zoom   R: reset   Space: auto-rotate\n             GLB animation 0 is played as the idle loop.",
            model_path.0
        )),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.93, 0.98)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(22.0),
            top: Val::Px(18.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new(
            "No stand-in character is generated. Add the reviewed, rigged Bulb GLB at the model path above.",
        ),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.72, 0.78, 0.87)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(22.0),
            bottom: Val::Px(18.0),
            ..default()
        },
    ));
}

fn orbit_camera(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut cameras: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    for (mut transform, mut orbit) in &mut cameras {
        if keys.just_pressed(KeyCode::KeyR) {
            orbit.yaw = 0.0;
            orbit.pitch = 0.0;
            orbit.distance = 4.2;
        }
        if keys.just_pressed(KeyCode::Space) {
            orbit.auto_rotate = !orbit.auto_rotate;
        }

        for motion in mouse_motion.read() {
            if buttons.pressed(MouseButton::Left) {
                orbit.yaw -= motion.delta.x * 0.008;
                orbit.pitch = (orbit.pitch + motion.delta.y * 0.006).clamp(-1.15, 1.15);
                orbit.auto_rotate = false;
            }
        }

        for wheel in mouse_wheel.read() {
            orbit.distance = (orbit.distance * (1.0 - wheel.y * 0.08)).clamp(1.6, 10.0);
        }

        if orbit.auto_rotate {
            orbit.yaw += time.delta_secs() * 0.35;
        }

        let horizontal = orbit.distance * orbit.pitch.cos();
        transform.translation = orbit.target
            + Vec3::new(
                orbit.yaw.sin() * horizontal,
                orbit.distance * orbit.pitch.sin(),
                orbit.yaw.cos() * horizontal,
            );
        transform.look_at(orbit.target, Vec3::Y);
    }
}

fn idle_float(time: Res<Time>, mut avatars: Query<&mut Transform, With<AvatarScene>>) {
    let bob = (time.elapsed_secs() * 1.5).sin() * 0.025;
    for mut transform in &mut avatars {
        transform.translation.y = bob;
    }
}
