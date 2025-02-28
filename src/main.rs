use colors_transform::{Color, Hsl};
use lerp::Lerp;
use macroquad::{prelude::*, rand::RandomRange};

#[derive(Default, Debug)]
struct ADeltafallLogo {
    rotation: f32,
    position: Vec2,
    size: f32,
    speed: f32
}

async fn render_bg_particles(texture: &Texture2D, particles: &mut Vec<ADeltafallLogo>) {
    for particle in particles.iter_mut() {
        let mut drawparams = DrawTextureParams::default();
        drawparams.dest_size = Some(vec2(particle.size, particle.size));
        drawparams.rotation = particle.rotation.to_radians();
        particle.rotation += 50.0 * get_frame_time();
        particle.position.y += particle.speed * get_frame_time();
        if particle.position.y > screen_height() {
            particle.position.y = 0.0;
            particle.position.x = RandomRange::gen_range(0.0, screen_width());
        };
        draw_texture_ex(&texture, particle.position.x, particle.position.y, macroquad::color::WHITE, drawparams);
    }
}

#[macroquad::main("Deltafall Clicker")]
async fn main() {

    let deltafall_logo = load_texture("deltafall.png").await.unwrap();
    
    let mut elapsed: f32 = 0.0;
    let title_texts: [String; 6] = ["Deltafall Clicker".to_string (), "Click the Deltafall!".to_string (), "We love clicking the Deltafall".to_string (),
    "Hell Yeah Click Deltafall".to_string (), "Wow clicking the Deltafall Amazing!".to_string (), "Deltafall by Hipxel clicking experience".to_string ()];
    let mut current_title_index: usize = 0;

    let mut current_cookie_rotation: f32 = 0.0;
    let mut current_cookie_to_rotation: f32 = 0.0;

    let mut click_count: u64 = 0;

    let mut bg_particles: Vec<ADeltafallLogo> = Vec::new();

    loop {
        elapsed += get_frame_time();
        let hsl_color = Hsl::from((elapsed / 8.0 % 1.0) * 360.0,10.0, 50.0);
        let color_rgba = macroquad::color::Color::from_rgba(hsl_color.get_red() as u8, hsl_color.get_green() as u8, hsl_color.get_blue() as u8, 255);
        clear_background(color_rgba);
        
        let df_logo_size: f32 = 150.0 * (1.0 + (elapsed/2.0).sin().abs()) ;
        let mut drawparams = DrawTextureParams::default();
        drawparams.dest_size = Some(vec2(df_logo_size, df_logo_size));
        drawparams.rotation = current_cookie_rotation.to_radians();
        draw_texture_ex(&deltafall_logo, (screen_width() - drawparams.dest_size.unwrap().x) / 2.0, (screen_height() - drawparams.dest_size.unwrap().y) / 2.0, macroquad::color::WHITE, drawparams);

        if is_mouse_button_pressed(MouseButton::Left) {
            current_cookie_to_rotation += 47.0;
            click_count += 1;
            
            let deltafall_logo_obj: ADeltafallLogo = ADeltafallLogo {
                position: Vec2 {
                    x: RandomRange::gen_range(0, screen_width() as i32) as f32,
                    y: 0.0
                },
                rotation: (RandomRange::gen_range(0, 360) as f32).to_radians(),
                size: RandomRange::gen_range(0, 50) as f32,
                speed: RandomRange::gen_range(50, 300) as f32
            };
            bg_particles.push(deltafall_logo_obj);
        }
        render_bg_particles(&deltafall_logo, &mut bg_particles).await;

        current_cookie_rotation = Lerp::lerp(current_cookie_rotation, current_cookie_to_rotation, get_frame_time() * 5.0);

        draw_rectangle(0.0, 0.0, screen_width(), 66.0, WHITE);
        let title_index_normal: f32 = (1.0 + elapsed.sin()).clamp(0.001, 1.0);
        if title_index_normal == 0.001{
            if current_title_index + 1 == title_texts.len() {
                current_title_index = 0;
            } else { current_title_index += 1 }
        }
        
        let current_title = &title_texts[current_title_index];
        let textdimen = draw_text(&current_title[0..(title_index_normal *current_title.len() as f32) as usize], 20.0, 50.0, 64.0, DARKGRAY);
        draw_line(19.0+textdimen.width, 50.0 / 1.5, 20.0+textdimen.width, 50.0 / 1.5, 64.0, macroquad::color::Color::from_rgba(0, 0, 0, ((elapsed*4.0).sin().abs() * 255.0) as u8));
        draw_text(&format!("click count: {}", click_count.to_string()), 20.0, 100.0, 24.0, WHITE);

        next_frame().await
    }
}