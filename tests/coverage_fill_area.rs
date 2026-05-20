use std::sync::mpsc;
use std::time::Duration;

use spritekit::{
    Action, Attribute, AttributeType, AttributeValue, CGPoint, CGRect, CGSize, CGVector,
    CameraNode, Color, CropNode, EffectNode, EmitterNode, Event, FieldNode, MutableTexture, Node,
    NodeExt, ParticleRenderOrder, PhysicsBody, PhysicsContact, PhysicsContactDelegate,
    PhysicsJointLimit, ReachConstraints, ReferenceNode, Region, Scene, SceneDelegate, Shader,
    ShapeNode, SpriteNode, Texture, TextureAtlas, TileAdjacencyMask, TileDefinition,
    TileDefinitionRotation, TileGroup, TileGroupRule, TileMapNode, TileSet, TileSetType,
    TransformNode, Transition, TransitionDirection, UniformType, View, ViewDelegate,
    WarpGeometryGrid, WarpableNode,
};

const CALLBACK_TIMEOUT: Duration = Duration::from_secs(5);

#[test]
fn foundation_wrappers_smoke() {
    let attribute = Attribute::new("a_pos", AttributeType::Float).expect("attribute");
    assert_eq!(attribute.name().as_deref(), Some("a_pos"));
    assert_eq!(attribute.attribute_type(), AttributeType::Float);

    let value = AttributeValue::with_float(1.25).expect("attribute value");
    value.set_float_value(2.5);
    assert!((value.float_value() - 2.5).abs() < f32::EPSILON);

    let node = Node::new().expect("node");
    node.set_accessibility_element(true);
    node.set_accessibility_label("play");
    node.set_accessibility_enabled(true);
    assert!(node.is_accessibility_element());
    assert_eq!(node.accessibility_label().as_deref(), Some("play"));
    assert!(node.is_accessibility_enabled());

    let constraints = ReachConstraints::new(-1.0, 1.0).expect("constraints");
    constraints.set_lower_angle_limit(-0.5);
    constraints.set_upper_angle_limit(0.75);
    assert!((constraints.lower_angle_limit() + 0.5).abs() < f64::EPSILON);
    assert!((constraints.upper_angle_limit() - 0.75).abs() < f64::EPSILON);
    node.set_reach_constraints(Some(&constraints));
    assert!(node.reach_constraints().is_some());

    let event = Event::mouse_moved(CGPoint::new(4.0, 6.0)).expect("event");
    let location = event.location_in_node(&node);
    assert!((location.x - 4.0).abs() < 1e-6);
    assert!((location.y - 6.0).abs() < 1e-6);

    let _drag = FieldNode::drag().expect("drag field");
    let _vortex = FieldNode::vortex().expect("vortex field");
    let field = FieldNode::linear_gravity([0.0, -1.0, 0.0]).expect("field");
    let region = Region::with_radius(8.0).expect("region");
    let other_region = Region::with_size(CGSize::new(4.0, 4.0)).expect("other region");
    let union = region.union(&other_region).expect("union");
    let _difference = union.difference(&other_region).expect("difference");
    let _inverse = union.inverse().expect("inverse");
    let intersection = union.intersection(&region).expect("intersection");
    field.set_region(Some(&union));
    field.set_strength(3.0);
    field.set_direction([1.0, 0.0, 0.0]);
    field.set_enabled(true);
    assert!(union.contains_point(CGPoint::new(0.0, 0.0)));
    assert!(intersection.contains_point(CGPoint::new(0.0, 0.0)));
    assert!(field.region().is_some());
    let direction = field.direction();
    assert!((direction[0] - 1.0).abs() < f32::EPSILON);
    assert!(direction[1].abs() < f32::EPSILON);
    assert!(direction[2].abs() < f32::EPSILON);
    assert!((field.strength() - 3.0).abs() < f32::EPSILON);
    assert!(field.is_enabled());
}

#[test]
fn delegates_camera_and_transitions_smoke() {
    let view = View::with_frame(CGRect::new(0.0, 0.0, 160.0, 90.0)).expect("view");
    let scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("scene");
    let next_scene = Scene::with_size(CGSize::new(160.0, 90.0)).expect("next scene");
    let camera = CameraNode::new().expect("camera");

    scene.set_camera(Some(&camera));
    assert!(scene.camera().is_some());
    assert_eq!(camera.contained_node_count(), 0);
    let _ = camera.contains_node(&scene);

    let _unused_scene_delegate = SceneDelegate::new().expect("scene delegate");
    let scene_delegate = SceneDelegate::with_update(|_| {}).expect("scene delegate with update");
    scene.set_delegate(Some(&scene_delegate));
    assert!(scene.has_delegate());

    let _unused_view_delegate = ViewDelegate::new().expect("view delegate");
    let view_delegate =
        ViewDelegate::with_should_render(|_| true).expect("view delegate with callback");
    view.set_delegate(Some(&view_delegate));
    assert!(view.has_delegate());

    let transition = Transition::push(TransitionDirection::Left, 0.25).expect("transition");
    transition.set_pauses_incoming_scene(true);
    transition.set_pauses_outgoing_scene(false);
    assert!(transition.pauses_incoming_scene());
    assert!(!transition.pauses_outgoing_scene());

    view.present_scene(Some(&scene));
    view.present_scene_with_transition(&next_scene, &transition);
    assert!(view.scene().is_some());
    assert!(next_scene.view().is_some());
}

#[test]
fn advanced_nodes_and_shader_smoke() {
    let shape = ShapeNode::with_circle(8.0).expect("shape");
    shape.set_line_width(2.0);
    shape.set_glow_width(1.0);
    shape.set_antialiased(false);
    shape.set_stroke_color(Color::red());
    shape.set_fill_color(Color::blue());
    assert!((shape.line_width() - 2.0).abs() < f64::EPSILON);
    assert!((shape.glow_width() - 1.0).abs() < f64::EPSILON);
    assert!(!shape.is_antialiased());
    let _ = shape.line_length();

    let crop = CropNode::new().expect("crop");
    let mask = Node::new().expect("mask");
    crop.set_mask_node(Some(&mask));
    assert!(crop.mask_node().is_some());

    let transform = TransformNode::new().expect("transform");
    transform.set_x_rotation(0.25);
    transform.set_y_rotation(0.5);
    assert!((transform.x_rotation() - 0.25).abs() < f64::EPSILON);
    assert!((transform.y_rotation() - 0.5).abs() < f64::EPSILON);

    let texture = MutableTexture::with_size(CGSize::new(4.0, 4.0)).expect("mutable texture");
    assert_eq!(texture.size(), CGSize::new(4.0, 4.0));
    let _ = MutableTexture::with_size_pixel_format(CGSize::new(2.0, 2.0), 70);

    let shader = Shader::new().expect("shader");
    shader.add_float_uniform("u_time", 0.5);
    assert_eq!(shader.uniform_count(), 1);
    assert_eq!(
        shader.uniform_type_named("u_time"),
        Some(UniformType::Float)
    );

    let emitter = EmitterNode::new().expect("emitter");
    emitter.set_particle_render_order(ParticleRenderOrder::DontCare);
    assert_eq!(
        emitter.particle_render_order(),
        ParticleRenderOrder::DontCare
    );

    let _ = TextureAtlas::named("MissingAtlas");
    let _ = ReferenceNode::with_file_named("missing-reference.sks");
    if let Some(reference) =
        ReferenceNode::with_url_path("/Users/perjohansson/dev/spritekit-rs/missing-reference.sks")
    {
        reference.resolve_reference_node();
    }
}

#[test]
fn tiles_warps_and_action_categories_smoke() {
    let texture = Texture::from_rgba_bytes(&[255, 255, 255, 255], 1, 1).expect("texture");
    let definition = TileDefinition::with_texture_size(&texture, CGSize::new(16.0, 16.0))
        .expect("tile definition");
    definition.set_name("grass");
    definition.set_size(CGSize::new(16.0, 16.0));
    definition.set_placement_weight(2);
    definition.set_rotation(TileDefinitionRotation::Rotation90);
    definition.set_flip_horizontally(true);
    definition.set_flip_vertically(true);
    assert_eq!(definition.name().as_deref(), Some("grass"));
    assert_eq!(definition.rotation(), TileDefinitionRotation::Rotation90);
    assert!(definition.flip_horizontally());
    assert!(definition.flip_vertically());

    let adjacency = TileAdjacencyMask::UP | TileAdjacencyMask::LEFT;
    let rule = TileGroupRule::new(adjacency, &[&definition]).expect("tile rule");
    rule.set_name("edge");
    assert_eq!(rule.name().as_deref(), Some("edge"));
    assert_eq!(rule.adjacency().bits(), adjacency.bits());

    let group = TileGroup::with_rules(&[&rule]).expect("tile group");
    group.set_name("terrain");
    let _ = TileGroup::with_tile_definition(&definition).expect("tile group from definition");
    let _ = TileGroup::empty().expect("empty tile group");

    let tile_set = TileSet::with_tile_groups_type(&[&group], TileSetType::Grid).expect("tile set");
    tile_set.set_name("terrain-set");
    tile_set.set_default_tile_group(Some(&group));
    tile_set.set_default_tile_size(CGSize::new(16.0, 16.0));
    assert_eq!(tile_set.tile_set_type(), TileSetType::Grid);
    assert!(tile_set.default_tile_group().is_some());

    let map =
        TileMapNode::with_fill(&tile_set, 2, 2, CGSize::new(16.0, 16.0), &group).expect("tile map");
    assert_eq!(map.number_of_columns(), 2);
    assert_eq!(map.number_of_rows(), 2);
    map.set_tile_size(CGSize::new(16.0, 16.0));
    map.set_tile_group(Some(&group), 0, 1);
    assert!(map.tile_group_at(0, 1).is_some());
    assert!(map.tile_definition_at(0, 1).is_some());
    let center = map.center_of_tile_at(0, 1);
    assert_eq!(map.tile_column_index_from_position(center), 0);
    assert_eq!(map.tile_row_index_from_position(center), 1);

    let base_grid = WarpGeometryGrid::with_dimensions(1, 1).expect("warp grid");
    assert_eq!(base_grid.number_of_columns(), 1);
    assert_eq!(base_grid.number_of_rows(), 1);
    assert_eq!(base_grid.vertex_count(), 4);
    let replaced = base_grid
        .replacing_dest_positions(&[
            CGPoint::new(-1.0, -1.0),
            CGPoint::new(1.0, -0.8),
            CGPoint::new(-0.8, 1.0),
            CGPoint::new(1.0, 1.0),
        ])
        .expect("replaced warp grid");
    let _ = base_grid.source_position_at(0);
    let _ = replaced.dest_position_at(0);

    let sprite = SpriteNode::with_color(Color::white(), CGSize::new(32.0, 32.0)).expect("sprite");
    sprite.set_warp_geometry(Some(&replaced));
    sprite.set_subdivision_levels(2);
    assert!(sprite.warp_geometry().is_some());
    assert_eq!(sprite.subdivision_levels(), 2);

    let effect = EffectNode::new().expect("effect");
    effect.set_warp_geometry(Some(&base_grid));
    effect.set_subdivision_levels(1);
    assert!(effect.warp_geometry().is_some());

    assert!(Action::change_charge_to(1.0, 0.2).is_some());
    assert!(Action::apply_force(CGVector::new(1.0, 2.0), 0.2).is_some());
    assert!(Action::play().is_some());
    assert!(Action::change_volume_to(0.5, 0.2).is_some());
    assert!(Action::stereo_pan_to(0.2, 0.2).is_some());
    assert!(Action::warp_to(&base_grid, 0.2).is_some());
    assert!(Action::animate_with_warps(&[&base_grid, &replaced], &[0.0, 1.0]).is_some());
}

#[test]
fn physics_contact_delegate_and_limit_joint_smoke() {
    let scene = Scene::with_size(CGSize::new(64.0, 64.0)).expect("scene");
    let world = scene.physics_world();
    let _unused_delegate = PhysicsContactDelegate::new().expect("empty contact delegate");
    let delegate = PhysicsContactDelegate::from_callbacks(
        Some(|contact: PhysicsContact| {
            let _ = contact.body_a();
            let _ = contact.body_b();
            let _ = contact.contact_point();
            let _ = contact.contact_normal();
            let _ = contact.collision_impulse();
        }),
        None::<fn(PhysicsContact)>,
    )
    .expect("contact delegate");
    world.set_contact_delegate(Some(&delegate));
    assert!(world.has_contact_delegate());

    // Constructing SKPhysicsJointLimit can still crash in headless CI on some macOS setups,
    // so keep the API typechecked without exercising the runtime path unless explicitly opted in.
    if std::env::var_os("SPRITEKIT_RS_RUN_LIMIT_JOINT").is_some() {
        let body_a = PhysicsBody::circle(8.0).expect("body a");
        let body_b = PhysicsBody::circle(8.0).expect("body b");
        let joint = PhysicsJointLimit::new(
            &body_a,
            &body_b,
            CGPoint::new(0.0, 0.0),
            CGPoint::new(16.0, 0.0),
        )
        .expect("limit joint");
        joint.set_max_length(24.0);
        assert!((joint.max_length() - 24.0).abs() < f64::EPSILON);
    }

    world.set_contact_delegate(None);
}

#[test]
fn mutable_texture_modify_pixel_data_smoke() {
    let texture = MutableTexture::with_size(CGSize::new(2.0, 2.0)).expect("mutable texture");
    let (tx, rx) = mpsc::channel();
    texture.modify_pixel_data(move |pixels| {
        for pixel in pixels.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[255, 0, 0, 255]);
        }
        let _ = tx.send(pixels.len());
    });

    assert_eq!(
        rx.recv_timeout(CALLBACK_TIMEOUT)
            .expect("mutable texture callback"),
        16
    );
}

#[test]
fn texture_atlas_preload_callbacks_smoke() {
    let atlas = TextureAtlas::named("MissingAtlas").expect("atlas handle");

    let (instance_tx, instance_rx) = mpsc::channel();
    atlas.preload_with_completion_handler(move || {
        let _ = instance_tx.send(());
    });
    instance_rx
        .recv_timeout(CALLBACK_TIMEOUT)
        .expect("instance preload callback");

    let (class_tx, class_rx) = mpsc::channel();
    TextureAtlas::preload_texture_atlases_with_completion_handler(&[&atlas], move || {
        let _ = class_tx.send(());
    });
    class_rx
        .recv_timeout(CALLBACK_TIMEOUT)
        .expect("class preload callback");

    let (named_tx, named_rx) = mpsc::channel();
    TextureAtlas::preload_texture_atlases_named_with_completion_handler(&[], move |result| {
        let _ = named_tx.send(result.map(|atlases| atlases.len()).map_err(|error| error.to_string()));
    })
    .expect("named preload scheduling");
    assert_eq!(
        named_rx
            .recv_timeout(CALLBACK_TIMEOUT)
            .expect("named preload callback")
            .expect("named preload result"),
        0
    );
}

#[cfg(feature = "async")]
#[test]
fn texture_atlas_async_smoke() {
    let atlas = TextureAtlas::named("MissingAtlas").expect("atlas handle");

    pollster::block_on(async {
        atlas.preload_async().await;
        TextureAtlas::preload_texture_atlases_async(&[&atlas]).await;
        let atlases = TextureAtlas::preload_texture_atlases_named_async(&[])
            .await
            .expect("named async preload");
        assert!(atlases.is_empty());
    });
}
