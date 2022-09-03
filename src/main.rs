use fluid::Fluid;
use macroquad::prelude::*;

pub mod fluid;

fn win_conf() -> Conf {
    Conf {
        window_title: String::from("Fluid Simulation"),
        window_width: fluid::N * fluid::SCALE,
        window_height: fluid::N * fluid::SCALE,
        fullscreen: false,
        window_resizable: false,

        ..Default::default()
    }
}

fn cap_fps(fps: i32) {
    let min_frame_time = 1. / fps as f32;
    let frame_time = get_frame_time();
    if frame_time < min_frame_time {
        let sleep_time = (min_frame_time - frame_time) * 1000.;
        std::thread::sleep(std::time::Duration::from_millis(sleep_time as u64));
    }
}

fn draw_fps(c: &mut i32, f: &mut String) {
    if *c > 10 {
        *f = format!("FPS: {}", get_fps());
        *c = 0;
    }
    draw_text((*f).as_str(), 5., 25., 40., RED);
    *c += 1;
}

#[macroquad::main(win_conf)]
async fn main() {
    let mut fps_counter = 0;
    let mut fps_text = "FPS: ".to_string();
    let mut pmouse = (0, 0);
    let mut visc_slider = 0.0000001;
    let mut diffusion_slider = 0.0000001;
    let mut density_slider = 100.;
    let mut fluid_red: u8 = 0xff;
    let mut fluid_green: u8 = 0xff;
    let mut fluid_blue: u8 = 0xff;
    let mut fluid = Fluid::new(0.15, diffusion_slider, visc_slider, (fluid_red, fluid_green, fluid_blue));

    loop {
        cap_fps(60);

        fluid.step();
        if pmouse == (0, 0) {
            pmouse = (mouse_position().0 as i32, mouse_position().1 as i32);
        }
        if is_mouse_button_down(MouseButton::Left) {
            fluid.add_density(
                mouse_position().0 as i32 / fluid::SCALE,
                mouse_position().1 as i32 / fluid::SCALE,
                density_slider,
            );
            fluid.add_velocity(
                mouse_position().0 as i32 / fluid::SCALE,
                mouse_position().1 as i32 / fluid::SCALE,
                ((mouse_position().0 as i32 / fluid::SCALE) - (pmouse.0 / fluid::SCALE)) as f32,
                ((mouse_position().1 as i32 / fluid::SCALE) - (pmouse.1 / fluid::SCALE)) as f32,
            );
        }
        if is_key_pressed(KeyCode::Backspace) {
            fluid = Fluid::new(0.15, 0.0, visc_slider, (fluid_red, fluid_green, fluid_blue));
        }
        fluid.render_d();
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Variables")
                .show(egui_ctx, |ui| {  
                    ui
                        .add(egui::Slider::new(&mut visc_slider, 0.0..=0.00005).text("Viscosity"));
                    ui
                        .add(egui::Slider::new(&mut diffusion_slider, 0.0..=0.00005).text("Diffusion"));
                    ui
                        .add(egui::Slider::new(&mut density_slider, 0.0..=1000.).text("Density"));
                    ui
                        .add(egui::Slider::new(&mut fluid_red, 0..=255).text("R"));
                    ui
                        .add(egui::Slider::new(&mut fluid_green, 0..=255).text("G"));
                    ui
                        .add(egui::Slider::new(&mut fluid_blue, 0..=255).text("B"));
                    if ui.button("Clear").clicked() {
                        fluid = Fluid::new(0.15, 0.0, visc_slider, (fluid_red, fluid_green, fluid_blue));
                    }
                    
                });
        });
        fluid.set_viscosity(visc_slider);
        fluid.set_diffusion(diffusion_slider);
        fluid.set_color((fluid_red, fluid_green, fluid_blue));
        egui_macroquad::draw();
        draw_fps(&mut fps_counter, &mut fps_text);
        pmouse = (mouse_position().0 as i32, mouse_position().1 as i32);
        next_frame().await
    }
}
