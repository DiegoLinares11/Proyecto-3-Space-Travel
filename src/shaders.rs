
use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;


pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub static mut SHADER_INDEX: u8 = 0;

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  unsafe {
    match SHADER_INDEX {
        5 => black_and_white(fragment, uniforms),
        1 => dalmata_shader(fragment, uniforms),
        2 => cloud_shader(fragment, uniforms),
        3 => cellular_shader(fragment, uniforms),
        4 => lava_shader(fragment, uniforms),
        0 => earth_shader(fragment, uniforms),
        6 => moon_shader(fragment, uniforms), 
        _ => cellular_shader(fragment, uniforms), // Default
    }
  }
}
pub fn fragment_shader2(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    return emissive_shader(fragment, uniforms);  
}

fn emissive_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let intensity = fragment.intensity;

  // Color base del material, un amarillo cálido
  let base_color = Color::new(255, 223, 0); // Amarillo cálido

  // Color de emisión aún más brillante, amarillo intenso
  let emission_color = Color::new(255, 255, 102) * (intensity * 2.0); // Amarillo más claro y brillante

  // Mezclamos el color base y el color de emisión
  let final_color = base_color.lerp(&emission_color, 0.9); // Mayor peso del color de emisión

  final_color
}


// Función para cambiar el índice del shader activo
pub fn switch_shader() {
  unsafe {
      SHADER_INDEX = (SHADER_INDEX + 1) % 7; // Cambia al siguiente shader, volviendo a 0 si supera 4
  }
}

fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 50.0; // Escala para definir detalles en la superficie
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.1; // Tiempo para simular ligera rotación

  // Valor de ruido para la superficie de la luna
  let surface_noise = uniforms.noise.get_noise_2d(x * zoom + t, y * zoom + t);

  // Colores base para la luna
  let gray_color = Color::new(200, 200, 200);  // Color gris para la luna
  let crater_color = Color::new(150, 150, 150); // Color más oscuro para las áreas de cráteres

  // Umbral para simular cráteres
  let crater_threshold = 0.4;

  // Determinación del color de la superficie
  let base_color = if surface_noise > crater_threshold {
      gray_color // Área clara
  } else {
      crater_color // Área más oscura (cráter)
  };

  // Ajustar la intensidad del color final para simular la iluminación
  base_color * fragment.intensity
}

fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.5; // Tiempo para simular movimiento dinámico del brillo

  // Gradiente central hacia los bordes
  let distance_from_center = (x * x + y * y).sqrt();
  let glow_intensity = (1.0 - distance_from_center).clamp(0.0, 1.0); // Más brillante en el centro

  // Color base del Sol (amarillo brillante)
  let core_color = Color::new(255, 223, 0); // Amarillo intenso
  let glow_color = Color::new(255, 140, 0); // Amarillo-naranja para el borde

  // Simulación de destellos dinámicos usando ruido
  let noise_value = uniforms.noise.get_noise_2d(x * 100.0 + t, y * 100.0 + t);
  let flicker = (noise_value * 0.5 + 0.5).clamp(0.5, 1.0); // Ruido suavizado para destellos

  // Color final mezclando gradiente y destellos
  let base_color = core_color.lerp(&glow_color, 1.0 - glow_intensity); // Gradiente centro-borde
  let final_color = base_color * (glow_intensity * flicker);

  final_color
}



fn earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 80.0; // Escala del mapa de ruido para definir detalles de tierra y océanos
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.1; // Tiempo para simular ligera rotación

  // Valor de ruido para la superficie de la Tierra
  let surface_noise = uniforms.noise.get_noise_2d(x * zoom + t, y * zoom);

  // Colores para diferentes zonas
  let ocean_color = Color::new(0, 105, 148);        // Azul para océanos
  let land_color = Color::new(34, 139, 34);         // Verde para tierra
  let desert_color = Color::new(210, 180, 140);     // Arena para desiertos
  let snow_color = Color::new(255, 250, 250);       // Blanco para nieve en zonas polares
  let cloud_color = Color::new(255, 255, 255); // Blanco semitransparente para nubes

  // Umbrales para diferenciar áreas geográficas
  let snow_threshold = 0.7;
  let land_threshold = 0.4;
  let desert_threshold = 0.3;

  // Determinación de color de la superficie
  let base_color = if y.abs() > snow_threshold {
      snow_color // Zonas polares
  } else if surface_noise > land_threshold {
      land_color // Zonas verdes
  } else if surface_noise > desert_threshold {
      desert_color // Desiertos
  } else {
      ocean_color // Océanos
  };

  // Agregar nubes (ruido extra sobre el océano y la tierra)
  let cloud_noise = uniforms.noise.get_noise_2d(x * zoom * 0.5 + t * 0.5, y * zoom * 0.5 + t * 0.5);
  let final_color = if cloud_noise > 0.6 {
      // Si hay nubes, mezcla el color base con el color de las nubes
      base_color.lerp(&cloud_color, 0.3)
  } else {
      base_color
  };

  // Ajusta la intensidad del color final para simular la iluminación
  final_color * fragment.intensity
}


fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
  
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
  
    let random_number = rng.gen_range(0..=100);
  
    let black_or_white = if random_number < 50 {
      Color::new(0, 0, 0)
    } else {
      Color::new(255, 255, 255)
    };
  
    black_or_white * fragment.intensity
}
  
fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    let noise_value = uniforms.noise.get_noise_2d(
      (x + ox) * zoom,
      (y + oy) * zoom,
    );
  
    let spot_threshold = 0.5;
    let spot_color = Color::new(255, 255, 255); // White
    let base_color = Color::new(0, 0, 0); // Black
  
    let noise_color = if noise_value < spot_threshold {
      spot_color
    } else {
      base_color
    };
  
    noise_color * fragment.intensity
}
  
fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;  // to move our values 
    let ox = 100.0; // offset x in the noise map
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;
  
    let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);
  
    // Define cloud threshold and colors
    let cloud_threshold = 0.5; // Adjust this value to change cloud density
    let cloud_color = Color::new(255, 255, 255); // White for clouds
    let sky_color = Color::new(30, 97, 145); // Sky blue
  
    // Determine if the pixel is part of a cloud or sky
    let noise_color = if noise_value > cloud_threshold {
      cloud_color
    } else {
      sky_color
    };
  
    noise_color * fragment.intensity
}
  
fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0;  // Zoom factor to adjust the scale of the cell pattern
    let ox = 50.0;    // Offset x in the noise map
    let oy = 50.0;    // Offset y in the noise map
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    // Use a cellular noise function to create the plant cell pattern
    let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();
  
    // Define different shades of green for the plant cells
    let cell_color_1 = Color::new(85, 107, 47);   // Dark olive green
    let cell_color_2 = Color::new(124, 252, 0);   // Light green
    let cell_color_3 = Color::new(34, 139, 34);   // Forest green
    let cell_color_4 = Color::new(173, 255, 47);  // Yellow green
  
    // Use the noise value to assign a different color to each cell
    let final_color = if cell_noise_value < 0.15 {
      cell_color_1
    } else if cell_noise_value < 0.7 {
      cell_color_2
    } else if cell_noise_value < 0.75 {
      cell_color_3
    } else {
      cell_color_4
    };
  
    // Adjust intensity to simulate lighting effects (optional)
    final_color * fragment.intensity
}
  
fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base colors for the lava effect
    let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
    let dark_color = Color::new(130, 20, 0);   // Darker red-orange
  
    // Get fragment position
    let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth
    );
  
    // Base frequency and amplitude for the pulsating effect
    let base_frequency = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.01;
  
    // Pulsate on the z-axis to change spot size
    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;
  
    // Apply noise to coordinates with subtle pulsating on z-axis
    let zoom = 1000.0; // Constant zoom factor
    let noise_value1 = uniforms.noise.get_noise_3d(
      position.x * zoom,
      position.y * zoom,
      (position.z + pulsate) * zoom
    );
    let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x + 1000.0) * zoom,
      (position.y + 1000.0) * zoom,
      (position.z + 1000.0 + pulsate) * zoom
    );
    let noise_value = (noise_value1 + noise_value2) * 0.5;  // Averaging noise for smoother transitions
  
    // Use lerp for color blending based on noise value
    let color = dark_color.lerp(&bright_color, noise_value);
  
    color * fragment.intensity
}