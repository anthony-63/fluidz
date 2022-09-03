// use emitter::Emitter;
use fluid::Fluid;
use fluid_math::*;
use macroquad::prelude::*;

pub mod emitter;
pub mod fluid;
pub mod fluid_math;

fn win_conf() -> Conf {
    Conf {
        window_title: String::from("Fluid Simulation"),
        window_width: N * SCALE,
        window_height: N * SCALE,
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
        *f = format!("{}", get_fps());
        *c = 0;
    }
    draw_text((*f).as_str(), 5., 15., 25., YELLOW);
    *c += 1;
}

#[macroquad::main(win_conf)]
async fn main() {
    let mut fps_counter = 0;
    let mut fps_text = "FPS: ".to_string();
    let mut pmouse = (0, 0);
    let mut visc_slider = 0.0000001;
    let mut diffusion_slider = 0.0000001;
    let mut speed_slider: f32 = 0.15;
    let mut density_slider = 100.;
    let mut fluid_red: u8 = 0xff;
    let mut fluid_green: u8 = 0xff;
    let mut fluid_blue: u8 = 0xff;
    // let mut visc_slider_emitter = 0.0000001;
    // let mut diffusion_slider_emitter = 0.0000001;
    // let mut density_slider_emitter = 100.;
    // let mut speed_slider_emitter: f32 = 0.15;
    // let mut fluid_red_emitter: u8 = 0xff;
    // let mut fluid_green_emitter: u8 = 0xff;
    // let mut fluid_blue_emitter: u8 = 0xff;
    let mut fluid = Fluid::new(
        0.15,
        diffusion_slider,
        visc_slider,
        (fluid_red, fluid_green, fluid_blue),
    );
    // let emitter: &mut Emitter = &mut Emitter::empty();
    loop {
        cap_fps(60);

        // emitter.step();
        fluid.step();
        if pmouse == (0, 0) {
            pmouse = (mouse_position().0 as i32, mouse_position().1 as i32);
        }
        if is_mouse_button_down(MouseButton::Left) {
            fluid.add_density(
                mouse_position().0 as i32 / SCALE,
                mouse_position().1 as i32 / SCALE,
                density_slider,
            );
            fluid.add_velocity(
                mouse_position().0 as i32 / SCALE,
                mouse_position().1 as i32 / SCALE,
                ((mouse_position().0 as i32 / SCALE) - (pmouse.0 / SCALE)) as f32,
                ((mouse_position().1 as i32 / SCALE) - (pmouse.1 / SCALE)) as f32,
            );
        }
        // if is_mouse_button_pressed(MouseButton::Right) {
        //     *emitter = Emitter::new(
        //         mouse_position().0 as i32,
        //         mouse_position().1 as i32,
        //         mouse_position().0 as i32,
        //         mouse_position().1 as i32 - 100,
        //         (fluid_red_emitter, fluid_green_emitter, fluid_blue_emitter),
        //         diffusion_slider_emitter,
        //         visc_slider_emitter,
        //         0.15,
        //         density_slider_emitter,
        //     );
        // }
        if is_key_pressed(KeyCode::Backspace) {
            fluid = Fluid::new(0.15, 0.0, visc_slider, (fluid_red, fluid_green, fluid_blue));
        }
        fluid.render_d();
        // emitter.draw();
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("User").show(egui_ctx, |ui| {
                ui.add(egui::Slider::new(&mut visc_slider, 0.0..=0.00005).text("Viscosity"));
                ui.add(egui::Slider::new(&mut diffusion_slider, 0.0..=0.00005).text("Diffusion"));
                ui.add(egui::Slider::new(&mut density_slider, 0.0..=2000.).text("Amount"));
                ui.add(egui::Slider::new(&mut speed_slider, 0.0..=1.).text("Speed"));
                ui.add(egui::Slider::new(&mut fluid_red, 0..=255).text("R"));
                ui.add(egui::Slider::new(&mut fluid_green, 0..=255).text("G"));
                ui.add(egui::Slider::new(&mut fluid_blue, 0..=255).text("B"));
                if ui.button("Clear").clicked() {
                    fluid =
                        Fluid::new(0.15, 0.0, visc_slider, (fluid_red, fluid_green, fluid_blue));
                }
            });
            // egui::Window::new("Emitter").show(egui_ctx, |ui| {
            //     ui.add(
            //         egui::Slider::new(&mut visc_slider_emitter, 0.0..=0.00005).text("Viscosity"),
            //     );
            //     ui.add(
            //         egui::Slider::new(&mut diffusion_slider_emitter, 0.0..=0.00005)
            //             .text("Diffusion"),
            //     );
            //     ui.add(egui::Slider::new(&mut speed_slider_emitter, 0.0..=1.).text("Speed"));
            //     ui.add(
            //         egui::Slider::new(&mut density_slider_emitter, 0.0..=2000.).text("Emission"),
            //     );
            //     ui.add(egui::Slider::new(&mut fluid_red_emitter, 0..=255).text("R"));
            //     ui.add(egui::Slider::new(&mut fluid_green_emitter, 0..=255).text("G"));
            //     ui.add(egui::Slider::new(&mut fluid_blue_emitter, 0..=255).text("B"));
            //     if ui.button("Remove").clicked() {
            //         *emitter = Emitter::empty();
            //     }
            // });
        });
        fluid.set_viscosity(visc_slider);
        fluid.set_diffusion(diffusion_slider);
        fluid.set_color((fluid_red, fluid_green, fluid_blue));
        fluid.set_speed(speed_slider);

        // emitter.fluid.set_viscosity(visc_slider_emitter);
        // emitter.fluid.set_diffusion(diffusion_slider_emitter);
        // emitter
        //     .fluid
        //     .set_color((fluid_red_emitter, fluid_green_emitter, fluid_blue_emitter));
        // emitter.set_density(density_slider_emitter);
        // emitter.fluid.set_speed(speed_slider_emitter);
        egui_macroquad::draw();
        draw_fps(&mut fps_counter, &mut fps_text);
        pmouse = (mouse_position().0 as i32, mouse_position().1 as i32);
        next_frame().await
    }
}
