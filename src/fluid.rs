use crate::fluid_math::*;

use macroquad::prelude::*;
pub struct Fluid {
    dt: f32,
    diff: f32,
    visc: f32,

    s: Vec<f32>,
    density: Vec<f32>,

    vx: Vec<f32>,
    vy: Vec<f32>,
    vx0: Vec<f32>,
    vy0: Vec<f32>,

    pub color: (u8, u8, u8),
}

impl Fluid {
    pub fn new(dt: f32, diffusion: f32, viscosity: f32, color: (u8, u8, u8)) -> Self {
        Self {
            dt,
            diff: diffusion,
            visc: viscosity,

            s: vec![0.; (N * N) as usize],
            density: vec![0.; (N * N) as usize],

            vx: vec![0.; (N * N) as usize],
            vy: vec![0.; (N * N) as usize],

            vx0: vec![0.; (N * N) as usize],
            vy0: vec![0.; (N * N) as usize],

            color,
        }
    }

    pub fn add_density(&mut self, x: i32, y: i32, amount: f32) {
        let idx = (x + y * N) as usize;
        self.density[idx % ((N * N) - 1) as usize] += amount;
    }

    pub fn add_velocity(&mut self, x: i32, y: i32, ax: f32, ay: f32) {
        let idx = (x + y * N) as usize;
        self.vx[idx % ((N * N) - 1) as usize] += ax;
        self.vy[idx % ((N * N) - 1) as usize] += ay;
    }
    pub fn step(&mut self) {
        let vx: &mut [f32] = &mut self.vx;
        let vy: &mut [f32] = &mut self.vy;
        let vx0: &mut Vec<f32> = &mut self.vx0;
        let vy0: &mut Vec<f32> = &mut self.vy0;
        let s: &mut [f32] = &mut self.s;
        let density: &mut [f32] = &mut self.density;

        diffuse(1, vx0, vx, self.visc, self.dt);
        diffuse(2, vy0, vy, self.visc, self.dt);

        project(vx0, vy0, vx, vy);

        advect(1, vx, &mut vx0.clone(), vx0, vy0, self.dt);
        advect(2, vy, &mut vy0.clone(), vx0, vy0, self.dt);

        project(vx, vy, vx0, vy0);

        diffuse(0, s, density, self.diff, self.dt);
        advect(0, density, s, vx, vy, self.dt);
    }
    pub fn render_d(&mut self) {
        for i in 0..N {
            for j in 0..N {
                // self.density[ix(i,j)] as f32
                draw_rectangle(
                    (i * SCALE) as f32,
                    (j * SCALE) as f32,
                    SCALE as f32,
                    SCALE as f32,
                    Color::from_rgba(
                        self.color.0 as u8,
                        self.color.1 as u8,
                        self.color.2 as u8,
                        self.density[((i + j * N) as usize)] as u8,
                    ),
                );
            }
        }
    }
    pub fn set_viscosity(&mut self, viscosity: f32) {
        self.visc = viscosity;
    }
    pub fn set_diffusion(&mut self, diffusion: f32) {
        self.diff = diffusion;
    }
    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }
    pub fn set_speed(&mut self, speed: f32) {
        self.dt = speed;
    }
}
