use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader, switch_shader, fragment_shader2, venus_shader, jupiter_shader, saturn_shader, mars_shader, earth_shader, uranus_shader, neptune_shader};
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
}

fn create_noise() -> FastNoiseLite {
    create_cloud_noise()
}

fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}


fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_sol(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = fragment_shader2(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);

            framebuffer.set_emission_color(0xFFFF00); // Emisión amarilla brillante

            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_venus(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterización y procesamiento de fragmentos
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Shader específico de Venus
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            // Aplicar el shader específico para Venus
            let shaded_color = venus_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_jupiter(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = jupiter_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_saturn(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = saturn_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_mars(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = mars_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_earth(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = earth_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_uranus(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = uranus_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}


fn render_neptune(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = neptune_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn render_point(framebuffer: &mut Framebuffer, position: Vec3, radius: usize) {
    let x = position.x as isize;
    let y = position.y as isize;

    let radius_squared = (radius as isize).pow(2);

    for dx in -(radius as isize)..=(radius as isize) {
        for dy in -(radius as isize)..=(radius as isize) {
            if dx * dx + dy * dy <= radius_squared {
                let px = x + dx;
                let py = y + dy;

                if px >= 0 && py >= 0 && (px as usize) < framebuffer.width && (py as usize) < framebuffer.height {
                    framebuffer.set_current_color(0xFFFFFF); // Blanco
                    framebuffer.point(px as usize, py as usize, position.z);
                }
            }
        }
    }
}






fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Sistema solar",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(000000);

    let sun_translation = Vec3::new(0.0, 0.0, 0.0);
    let sun_scale = 2.0; // Escala del sol

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 20.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 3.0, 0.0)
    );

    let planet_obj = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");
    let nave_obj = Obj::load("assets/models/Nave.obj").expect("Failed to load obj");
    let sol_obj = Obj::load("assets/models/sol.obj").expect("Failed to load obj");
    let mut time = 0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Cambia el shader cuando se presiona la tecla "Space"
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            switch_shader();
        }

        time += 1;
        handle_input(&window, &mut camera);

        framebuffer.clear();

        // Renderizar el Sol
        let sun_model_matrix = create_model_matrix(sun_translation, sun_scale, Vec3::new(0.0, 0.0, 0.0));
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        let sun_uniforms = Uniforms {
            model_matrix: sun_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };




        framebuffer.set_current_color(0xFFDD44); // Color para el Sol
        render_sol(&mut framebuffer, &sun_uniforms, &planet_obj.get_vertex_array());
        framebuffer.apply_emission();

        // Planeta Mercurio orbitando alrededor del Sol
        let planet1_distance = 2.1;
        let planet1_translation = Vec3::new(
            planet1_distance * (time as f32 * 0.08).cos(),
            0.0,
            planet1_distance * (time as f32 * 0.08).sin(),
        );

        let planet1_scale = 0.7;
        let planet1_model_matrix = create_model_matrix(planet1_translation, planet1_scale, Vec3::new(0.0, 0.0, 0.0));

        let planet1_uniforms = Uniforms {
            model_matrix: planet1_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        render(&mut framebuffer, &planet1_uniforms, &planet_obj.get_vertex_array());

    // Crear rastros para el planeta
    let trail_length = 50; // Número de puntos en el rastro

    for i in 0..trail_length {
        // Calcula un desfase temporal
        let trail_time = time as f32 - (i as f32 * 0.2);

        // Posición del punto basado en el tiempo desfaseado
        let trail_translation = Vec3::new(
            planet1_distance * (trail_time * 0.08).cos(),
            0.0,
            planet1_distance * (trail_time * 0.08).sin() - 0.05 * i as f32, // Desfase gradual en Z
        );

        // Escala pequeña para los puntos
        let trail_scale = 0.1;

        // Matriz de transformación para el "mini-planeta"
        let trail_model_matrix = create_model_matrix(trail_translation, trail_scale, Vec3::new(0.0, 0.0, 0.0));

        // Uniforms para el rastro
        let trail_uniforms = Uniforms {
            model_matrix: trail_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        // Renderiza el punto como un mini-planeta
        render(&mut framebuffer, &trail_uniforms, &planet_obj.get_vertex_array());
    }



        // Planeta Venus orbitando alrededor del Sol
        let planet2_distance = 3.3;
        let planet2_translation = Vec3::new(
            planet2_distance * (time as f32 * 0.05).cos(),
            0.0,
            planet2_distance * (time as f32 * 0.05).sin(),
        );
        let planet2_scale = 0.85;
        let planet2_model_matrix = create_model_matrix(planet2_translation, planet2_scale, Vec3::new(0.0, 0.0, 0.0));

        let planet2_uniforms = Uniforms {
            model_matrix: planet2_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        render_venus(&mut framebuffer, &planet2_uniforms, &planet_obj.get_vertex_array());


            // Crear rastros para el planeta
    let trail_length = 50; // Número de puntos en el rastro

    for i in 0..trail_length {
        // Calcula un desfase temporal
        let trail_time = time as f32 - (i as f32 * 0.2);

        // Posición del punto basado en el tiempo desfaseado
        let trail_translation = Vec3::new(
            planet2_distance * (trail_time * 0.05).cos(),
            0.0,
            planet2_distance * (trail_time * 0.05).sin() - 0.05 * i as f32, // Desfase gradual en Z
        );

        // Escala pequeña para los puntos
        let trail_scale = 0.1;

        // Matriz de transformación para el "mini-planeta"
        let trail_model_matrix = create_model_matrix(trail_translation, trail_scale, Vec3::new(0.0, 0.0, 0.0));

        // Uniforms para el rastro
        let trail_uniforms = Uniforms {
            model_matrix: trail_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        // Renderiza el punto como un mini-planeta
        render(&mut framebuffer, &trail_uniforms, &planet_obj.get_vertex_array());
    }

        // Planeta Tierra orbitando alrededor del Sol
        let planet3_distance = 5.1;
        let planet3_translation = Vec3::new(
            planet3_distance * (time as f32 * 0.045).cos(),
            0.0,
            planet3_distance * (time as f32 * 0.045).sin(),
        );
        let planet3_scale = 1.0;
        let planet3_model_matrix = create_model_matrix(planet3_translation, planet3_scale, Vec3::new(0.0, 0.0, 0.0));

        let planet3_uniforms = Uniforms {
            model_matrix: planet3_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        render_earth(&mut framebuffer, &planet3_uniforms, &planet_obj.get_vertex_array());

        // Planeta Marte orbitando alrededor del Sol
        let planet4_distance = 6.4;
        let planet4_translation = Vec3::new(
            planet4_distance * (time as f32 * 0.04).cos(),
            0.0,
            planet4_distance * (time as f32 * 0.04).sin(),
        );
        let planet4_scale = 0.7;
        let planet4_model_matrix = create_model_matrix(planet4_translation, planet4_scale, Vec3::new(0.0, 0.0, 0.0));

        let planet4_uniforms = Uniforms {
            model_matrix: planet4_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        render_mars(&mut framebuffer, &planet4_uniforms, &planet_obj.get_vertex_array());

        // Planeta Júpiter orbitando alrededor del Sol
        let planet5_distance = 7.9;
        let planet5_translation = Vec3::new(
            planet5_distance * (time as f32 * 0.035).cos(),
            0.0,
            planet5_distance * (time as f32 * 0.035).sin(),
        );
        let planet5_scale = 2.1;
        let planet5_model_matrix = create_model_matrix(planet5_translation, planet5_scale, Vec3::new(0.0, 0.0, 0.0));

        let planet5_uniforms = Uniforms {
            model_matrix: planet5_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        render_jupiter(&mut framebuffer, &planet5_uniforms, &planet_obj.get_vertex_array());

        // Planeta Saturno orbitando alrededor del Sol
        let planet6_distance = 9.9;
        let planet6_translation = Vec3::new(
            planet6_distance * (time as f32 * 0.03).cos(),
            0.0,
            planet6_distance * (time as f32 * 0.03).sin(),
        );
        let planet6_scale = 1.8;
        let planet6_model_matrix = create_model_matrix(planet6_translation, planet6_scale, Vec3::new(0.0, 0.0, 0.0));

        let planet6_uniforms = Uniforms {
            model_matrix: planet6_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        render_saturn(&mut framebuffer, &planet6_uniforms, &planet_obj.get_vertex_array());

        // Planeta Urano orbitando alrededor del Sol
        let planet7_distance = 12.1;
        let planet7_translation = Vec3::new(
            planet7_distance * (time as f32 * 0.025).cos(),
            0.0,
            planet7_distance * (time as f32 * 0.025).sin(),
        );
        let planet7_scale = 1.6;
        let planet7_model_matrix = create_model_matrix(planet7_translation, planet7_scale, Vec3::new(0.0, 0.0, 0.0));

        let planet7_uniforms = Uniforms {
            model_matrix: planet7_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        render_uranus(&mut framebuffer, &planet7_uniforms, &planet_obj.get_vertex_array());

        // Planeta Neptuno orbitando alrededor del Sol
        let planet8_distance = 15.2;
        let planet8_translation = Vec3::new(
            planet8_distance * (time as f32 * 0.02).cos(),
            0.0,
            planet8_distance * (time as f32 * 0.02).sin(),
        );
        let planet8_scale = 1.6;
        let planet8_model_matrix = create_model_matrix(planet8_translation, planet8_scale, Vec3::new(0.0, 0.0, 0.0));

        let planet8_uniforms = Uniforms {
            model_matrix: planet8_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: create_noise(),
        };

        render_neptune(&mut framebuffer, &planet8_uniforms, &planet_obj.get_vertex_array());


        // Movimiento orbital de la nave espacial
        let spaceship_distance = 3.0; 
        let spaceship_translation = Vec3::new(
            spaceship_distance * (time as f32 * -0.016).cos(), // Movimiento en X
            -5.0, // Movimiento en Y 
            spaceship_distance * (time as f32 * -0.016).sin(), // Movimiento en Z
        );

        // Escala de la nave 
        let spaceship_scale = 0.6;
        let spaceship_model_matrix = create_model_matrix(spaceship_translation, spaceship_scale, Vec3::new(0.0, 0.0, 0.0));

        let spaceship_uniforms = Uniforms {
            model_matrix: spaceship_model_matrix, // Matriz de modelo actualizada con movimiento orbital
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up), // Matriz de vista
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32), 
            viewport_matrix: create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32), 
            time,
            noise: create_noise(),
        };

        render(&mut framebuffer, &spaceship_uniforms, &nave_obj.get_vertex_array());


        // Nave espacial mas pequeña.
        let navecita_distance = 3.0; 
        let navecita_translation = Vec3::new(
            navecita_distance * (time as f32 * -0.016).cos(), // Movimiento en X
            5.0, // Movimiento en Y 
            navecita_distance * (time as f32 * -0.016).sin(), // Movimiento en Z
        );

        let navecita_scale = 0.3;
        let navecita_model_matrix = create_model_matrix(navecita_translation, navecita_scale, Vec3::new(0.0, 0.0, 0.0));

        let navecita_uniforms = Uniforms {
            model_matrix: navecita_model_matrix, // Matriz de modelo actualizada con movimiento orbital
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up), // Matriz de vista
            projection_matrix: create_perspective_matrix(window_width as f32, window_height as f32), 
            viewport_matrix: create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32), 
            time,
            noise: create_noise(),
        };

        render(&mut framebuffer, &navecita_uniforms, &nave_obj.get_vertex_array());


        // Actualizar la ventana y dormir un poco
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }


}

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;

    //  camera orbit controls
    if window.is_key_down(Key::Left) {
      camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
      camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
      camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
      camera.orbit(0.0, rotation_speed);
    }

    // Camera movement controls
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
      movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
      movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
      movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
      movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
      camera.move_center(movement);
    }

    // Camera zoom controls
    if window.is_key_down(Key::Up) {
      camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
      camera.zoom(-zoom_speed);
    }
}
