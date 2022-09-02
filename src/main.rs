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
    let mut fluid_thick = Fluid::new(0.15, 0.0, 0.0001, (0xFF, 0x00, 0x00));
    let mut fluid_thin = Fluid::new(0.15, 0.0, 0.000005, (0x00, 0xFF, 0x00));
    loop {
        cap_fps(60);

        fluid_thick.step();
        fluid_thin.step();
        if pmouse == (0, 0) {
            pmouse = (mouse_position().0 as i32, mouse_position().1 as i32);
        }
        if is_mouse_button_down(MouseButton::Left) {
            fluid_thick.add_density(
                mouse_position().0 as i32 / fluid::SCALE,
                mouse_position().1 as i32 / fluid::SCALE,
                300.,
            );
            fluid_thick.add_velocity(
                mouse_position().0 as i32 / fluid::SCALE,
                mouse_position().1 as i32 / fluid::SCALE,
                ((mouse_position().0 as i32 / fluid::SCALE) - (pmouse.0 / fluid::SCALE)) as f32,
                ((mouse_position().1 as i32 / fluid::SCALE) - (pmouse.1 / fluid::SCALE)) as f32,
            );
        } else if is_mouse_button_down(MouseButton::Right) {
            fluid_thin.add_density(
                mouse_position().0 as i32 / fluid::SCALE,
                mouse_position().1 as i32 / fluid::SCALE,
                300.,
            );
            fluid_thin.add_velocity(
                mouse_position().0 as i32 / fluid::SCALE,
                mouse_position().1 as i32 / fluid::SCALE,
                ((mouse_position().0 as i32 / fluid::SCALE) - (pmouse.0 / fluid::SCALE)) as f32,
                ((mouse_position().1 as i32 / fluid::SCALE) - (pmouse.1 / fluid::SCALE)) as f32,
            );
        }
        fluid_thick.render_d();
        fluid_thin.render_d();
        draw_fps(&mut fps_counter, &mut fps_text);
        pmouse = (mouse_position().0 as i32, mouse_position().1 as i32);
        next_frame().await
    }
}
