#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn sk_release(handle: *mut c_void);

    // --- Scene ---
    pub fn sk_scene_new_with_size(width: f64, height: f64) -> *mut c_void;
    pub fn sk_scene_get_size_w(scene: *mut c_void) -> f64;
    pub fn sk_scene_get_size_h(scene: *mut c_void) -> f64;
    pub fn sk_scene_set_size(scene: *mut c_void, width: f64, height: f64);
    pub fn sk_scene_get_scale_mode(scene: *mut c_void) -> i32;
    pub fn sk_scene_set_scale_mode(scene: *mut c_void, mode: i32);
    pub fn sk_scene_set_background_color(scene: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn sk_scene_get_anchor_x(scene: *mut c_void) -> f64;
    pub fn sk_scene_get_anchor_y(scene: *mut c_void) -> f64;
    pub fn sk_scene_set_anchor_point(scene: *mut c_void, x: f64, y: f64);
    pub fn sk_scene_physics_world(scene: *mut c_void) -> *mut c_void;

    // --- Node ---
    pub fn sk_node_new() -> *mut c_void;
    pub fn sk_node_add_child(parent: *mut c_void, child: *mut c_void);
    pub fn sk_node_remove_from_parent(node: *mut c_void);
    pub fn sk_node_remove_all_children(node: *mut c_void);
    pub fn sk_node_copy_name(node: *mut c_void) -> *mut c_char;
    pub fn sk_node_set_name(node: *mut c_void, name: *const c_char);
    pub fn sk_node_get_position_x(node: *mut c_void) -> f64;
    pub fn sk_node_get_position_y(node: *mut c_void) -> f64;
    pub fn sk_node_set_position(node: *mut c_void, x: f64, y: f64);
    pub fn sk_node_get_z_position(node: *mut c_void) -> f64;
    pub fn sk_node_set_z_position(node: *mut c_void, z: f64);
    pub fn sk_node_get_z_rotation(node: *mut c_void) -> f64;
    pub fn sk_node_set_z_rotation(node: *mut c_void, rotation: f64);
    pub fn sk_node_get_x_scale(node: *mut c_void) -> f64;
    pub fn sk_node_set_x_scale(node: *mut c_void, scale: f64);
    pub fn sk_node_get_y_scale(node: *mut c_void) -> f64;
    pub fn sk_node_set_y_scale(node: *mut c_void, scale: f64);
    pub fn sk_node_set_scale(node: *mut c_void, scale: f64);
    pub fn sk_node_get_alpha(node: *mut c_void) -> f64;
    pub fn sk_node_set_alpha(node: *mut c_void, alpha: f64);
    pub fn sk_node_get_hidden(node: *mut c_void) -> bool;
    pub fn sk_node_set_hidden(node: *mut c_void, hidden: bool);
    pub fn sk_node_get_paused(node: *mut c_void) -> bool;
    pub fn sk_node_set_paused(node: *mut c_void, paused: bool);
    pub fn sk_node_get_speed(node: *mut c_void) -> f64;
    pub fn sk_node_set_speed(node: *mut c_void, speed: f64);
    pub fn sk_node_get_physics_body(node: *mut c_void) -> *mut c_void;
    pub fn sk_node_set_physics_body(node: *mut c_void, body: *mut c_void);
    pub fn sk_node_run_action(node: *mut c_void, action: *mut c_void);
    pub fn sk_node_remove_all_actions(node: *mut c_void);

    // --- SpriteNode ---
    pub fn sk_sprite_node_new_with_texture(texture: *mut c_void) -> *mut c_void;
    pub fn sk_sprite_node_new_with_color(
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    pub fn sk_sprite_node_get_texture(node: *mut c_void) -> *mut c_void;
    pub fn sk_sprite_node_set_texture(node: *mut c_void, texture: *mut c_void);
    pub fn sk_sprite_node_get_size_w(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_get_size_h(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_set_size(node: *mut c_void, width: f64, height: f64);
    pub fn sk_sprite_node_get_anchor_x(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_get_anchor_y(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_set_anchor_point(node: *mut c_void, x: f64, y: f64);
    pub fn sk_sprite_node_set_color(node: *mut c_void, r: f32, g: f32, b: f32, a: f32);
    pub fn sk_sprite_node_get_color_blend_factor(node: *mut c_void) -> f64;
    pub fn sk_sprite_node_set_color_blend_factor(node: *mut c_void, factor: f64);
    pub fn sk_sprite_node_get_blend_mode(node: *mut c_void) -> i32;
    pub fn sk_sprite_node_set_blend_mode(node: *mut c_void, mode: i32);

    // --- Texture ---
    pub fn sk_texture_image_named(name: *const c_char) -> *mut c_void;
    pub fn sk_texture_from_cg_image(image: *mut c_void) -> *mut c_void;
    pub fn sk_texture_from_rgba_bytes(
        bytes: *const u8,
        length: usize,
        width: usize,
        height: usize,
    ) -> *mut c_void;
    pub fn sk_texture_subrect(
        texture: *mut c_void,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> *mut c_void;
    pub fn sk_texture_get_size_w(texture: *mut c_void) -> f64;
    pub fn sk_texture_get_size_h(texture: *mut c_void) -> f64;
    pub fn sk_texture_get_filtering_mode(texture: *mut c_void) -> i32;
    pub fn sk_texture_set_filtering_mode(texture: *mut c_void, mode: i32);
    pub fn sk_texture_get_uses_mipmaps(texture: *mut c_void) -> bool;
    pub fn sk_texture_set_uses_mipmaps(texture: *mut c_void, uses: bool);

    // --- Action ---
    pub fn sk_action_move_by(dx: f64, dy: f64, duration: f64) -> *mut c_void;
    pub fn sk_action_move_to(x: f64, y: f64, duration: f64) -> *mut c_void;
    pub fn sk_action_rotate_by(angle: f64, duration: f64) -> *mut c_void;
    pub fn sk_action_rotate_to(angle: f64, duration: f64) -> *mut c_void;
    pub fn sk_action_scale_by(scale: f64, duration: f64) -> *mut c_void;
    pub fn sk_action_scale_to(scale: f64, duration: f64) -> *mut c_void;
    pub fn sk_action_resize_to(width: f64, height: f64, duration: f64) -> *mut c_void;
    pub fn sk_action_fade_in(duration: f64) -> *mut c_void;
    pub fn sk_action_fade_out(duration: f64) -> *mut c_void;
    pub fn sk_action_fade_to(alpha: f64, duration: f64) -> *mut c_void;
    pub fn sk_action_fade_alpha_by(alpha: f64, duration: f64) -> *mut c_void;
    pub fn sk_action_hide() -> *mut c_void;
    pub fn sk_action_unhide() -> *mut c_void;
    pub fn sk_action_set_texture(texture: *mut c_void) -> *mut c_void;
    pub fn sk_action_animate_with_textures(
        textures: *mut c_void,
        count: usize,
        time_per_frame: f64,
    ) -> *mut c_void;
    pub fn sk_action_wait(duration: f64) -> *mut c_void;
    pub fn sk_action_sequence(actions: *mut c_void, count: usize) -> *mut c_void;
    pub fn sk_action_group(actions: *mut c_void, count: usize) -> *mut c_void;
    pub fn sk_action_repeat(action: *mut c_void, count: usize) -> *mut c_void;
    pub fn sk_action_repeat_forever(action: *mut c_void) -> *mut c_void;

    // --- PhysicsBody ---
    pub fn sk_physics_body_circle(radius: f64) -> *mut c_void;
    pub fn sk_physics_body_rect(width: f64, height: f64) -> *mut c_void;
    pub fn sk_physics_body_edge_loop_rect(x: f64, y: f64, width: f64, height: f64) -> *mut c_void;
    pub fn sk_physics_body_texture(texture: *mut c_void, size_w: f64, size_h: f64) -> *mut c_void;
    pub fn sk_physics_body_get_dynamic(body: *mut c_void) -> bool;
    pub fn sk_physics_body_set_dynamic(body: *mut c_void, dynamic: bool);
    pub fn sk_physics_body_get_allows_rotation(body: *mut c_void) -> bool;
    pub fn sk_physics_body_set_allows_rotation(body: *mut c_void, allows: bool);
    pub fn sk_physics_body_get_precise_collision(body: *mut c_void) -> bool;
    pub fn sk_physics_body_set_precise_collision(body: *mut c_void, precise: bool);
    pub fn sk_physics_body_get_pinned(body: *mut c_void) -> bool;
    pub fn sk_physics_body_set_pinned(body: *mut c_void, pinned: bool);
    pub fn sk_physics_body_get_friction(body: *mut c_void) -> f64;
    pub fn sk_physics_body_set_friction(body: *mut c_void, friction: f64);
    pub fn sk_physics_body_get_restitution(body: *mut c_void) -> f64;
    pub fn sk_physics_body_set_restitution(body: *mut c_void, restitution: f64);
    pub fn sk_physics_body_get_linear_damping(body: *mut c_void) -> f64;
    pub fn sk_physics_body_set_linear_damping(body: *mut c_void, damping: f64);
    pub fn sk_physics_body_get_angular_damping(body: *mut c_void) -> f64;
    pub fn sk_physics_body_set_angular_damping(body: *mut c_void, damping: f64);
    pub fn sk_physics_body_get_density(body: *mut c_void) -> f64;
    pub fn sk_physics_body_set_density(body: *mut c_void, density: f64);
    pub fn sk_physics_body_get_mass(body: *mut c_void) -> f64;
    pub fn sk_physics_body_set_mass(body: *mut c_void, mass: f64);
    pub fn sk_physics_body_get_gravity_scale(body: *mut c_void) -> f64;
    pub fn sk_physics_body_set_affected_by_gravity(body: *mut c_void, affected: bool);
    pub fn sk_physics_body_get_category_bitmask(body: *mut c_void) -> u32;
    pub fn sk_physics_body_set_category_bitmask(body: *mut c_void, mask: u32);
    pub fn sk_physics_body_get_contact_test_bitmask(body: *mut c_void) -> u32;
    pub fn sk_physics_body_set_contact_test_bitmask(body: *mut c_void, mask: u32);
    pub fn sk_physics_body_get_collision_bitmask(body: *mut c_void) -> u32;
    pub fn sk_physics_body_set_collision_bitmask(body: *mut c_void, mask: u32);
    pub fn sk_physics_body_get_velocity_dx(body: *mut c_void) -> f64;
    pub fn sk_physics_body_get_velocity_dy(body: *mut c_void) -> f64;
    pub fn sk_physics_body_set_velocity(body: *mut c_void, dx: f64, dy: f64);
    pub fn sk_physics_body_apply_force(body: *mut c_void, dx: f64, dy: f64);
    pub fn sk_physics_body_apply_impulse(body: *mut c_void, dx: f64, dy: f64);

    // --- PhysicsWorld ---
    pub fn sk_physics_world_get_gravity_dx(world: *mut c_void) -> f64;
    pub fn sk_physics_world_get_gravity_dy(world: *mut c_void) -> f64;
    pub fn sk_physics_world_set_gravity(world: *mut c_void, dx: f64, dy: f64);
    pub fn sk_physics_world_get_speed(world: *mut c_void) -> f64;
    pub fn sk_physics_world_set_speed(world: *mut c_void, speed: f64);

    // --- EffectNode ---
    pub fn sk_effect_node_new() -> *mut c_void;
    pub fn sk_effect_node_get_should_enable_effects(node: *mut c_void) -> bool;
    pub fn sk_effect_node_set_should_enable_effects(node: *mut c_void, enable: bool);
    pub fn sk_effect_node_get_should_rasterize(node: *mut c_void) -> bool;
    pub fn sk_effect_node_set_should_rasterize(node: *mut c_void, rasterize: bool);
    pub fn sk_effect_node_get_blend_mode(node: *mut c_void) -> i32;
    pub fn sk_effect_node_set_blend_mode(node: *mut c_void, mode: i32);

    // --- Renderer ---
    pub fn sk_render_pass_descriptor_new_for_texture(
        texture: *mut c_void,
        clear_r: f64,
        clear_g: f64,
        clear_b: f64,
        clear_a: f64,
        load_action: i32,
        store_action: i32,
    ) -> *mut c_void;
    pub fn sk_renderer_new(device: *mut c_void) -> *mut c_void;
    pub fn sk_renderer_set_scene(renderer: *mut c_void, scene: *mut c_void);
    pub fn sk_renderer_update_at_time(renderer: *mut c_void, time: f64);
    pub fn sk_renderer_render(
        renderer: *mut c_void,
        vp_x: f64,
        vp_y: f64,
        vp_w: f64,
        vp_h: f64,
        command_buffer: *mut c_void,
        pass_descriptor: *mut c_void,
    );
    pub fn sk_texture_copy_bytes(
        texture: *mut c_void,
        out_bytes: *mut c_void,
        bytes_per_row: usize,
    ) -> bool;
}
