use math::round;

pub const N: i32 = 64;
pub const SCALE: i32 = 8;

const ITER: i32 = 10;

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

    color: (u8, u8, u8),
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
        let idx = ix(x, y);
        self.density[idx] += amount;
    }

    pub fn add_velocity(&mut self, x: i32, y: i32, ax: f32, ay: f32) {
        let idx = ix(x, y);
        self.vx[idx] += ax;
        self.vy[idx] += ay;
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
                        self.density[ix(i, j)] as u8,
                    ),
                );
            }
        }
    }
}

fn ix(x: i32, y: i32) -> usize {
    (x + y * N) as usize
}

fn diffuse(b: i32, x: &mut [f32], x0: &mut [f32], diff: f32, dt: f32) {
    let a = dt * diff * (N as f32 - 2.) * (N as f32 - 2.);
    lin_solve(b, x, x0, a, 1. + 6. * a);
}

fn lin_solve(b: i32, x: &mut [f32], x0: &mut [f32], a: f32, c: f32) {
    let c_recip = 1.0 / c;
    for _k in 0..ITER {
        for j in 1..(N - 1) {
            for i in 1..(N - 1) {
                x[ix(i, j)] = (x0[ix(i, j)]
                    + a * (x[ix(i + 1, j)] + x[ix(i - 1, j)] + x[ix(i, j + 1)] + x[ix(i, j - 1)]))
                    * c_recip;
            }
        }

        set_bnd(b, x);
    }
}

fn set_bnd(b: i32, x: &mut [f32]) {
    for i in 1..(N - 1) {
        x[ix(i, 0)] = if b == 2 { -x[ix(i, 1)] } else { x[ix(i, 1)] };
        x[ix(i, N - 1)] = if b == 2 {
            -x[ix(i, N - 2)]
        } else {
            x[ix(i, N - 2)]
        };
    }
    for j in 1..(N - 1) {
        x[ix(0, j)] = if b == 1 { -x[ix(1, j)] } else { x[ix(1, j)] };
        x[ix(N - 1, j)] = if b == 1 {
            -x[ix(N - 2, j)]
        } else {
            x[ix(N - 2, j)]
        };
    }

    x[ix(0, 0)] = 0.5 * (x[ix(1, 0)] + x[ix(0, 1)]);
    x[ix(0, N - 1)] = 0.5 * (x[ix(1, N - 1)] + x[ix(0, N - 2)]);
    x[ix(N - 1, 0)] = 0.5 * (x[ix(N - 2, 0)] + x[ix(N - 1, 1)]);
    x[ix(N - 1, N - 1)] = 0.5 * (x[ix(N - 2, N - 1)] + x[ix(N - 1, N - 2)]);
}

fn advect(
    b: i32,
    d: &mut [f32],
    d0: &mut [f32],
    velocity_x: &mut [f32],
    velocity_y: &mut [f32],
    dt: f32,
) {
    let mut i0: f32;
    let mut i1: f32;
    let mut j0: f32;
    let mut j1: f32;

    let dtx = dt * (N - 2) as f32;
    let dty = dt * (N - 2) as f32;

    let mut s0: f32;
    let mut s1: f32;
    let mut t0: f32;
    let mut t1: f32;
    let mut tmp1: f32;
    let mut tmp2: f32;
    let mut x: f32;
    let mut y: f32;

    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            tmp1 = dtx * velocity_x[ix(i, j)];
            tmp2 = dty * velocity_y[ix(i, j)];
            x = i as f32 - tmp1;
            y = j as f32 - tmp2;

            if x < 0.5 {
                x = 0.5
            };
            if x > N as f32 + 0.5 {
                x = N as f32 + 0.5
            };
            i0 = round::floor(x as f64, 0) as f32;
            i1 = i0 + 1.0;
            if y < 0.5 {
                y = 0.5
            };
            if y > N as f32 + 0.5 {
                y = N as f32 + 0.5
            };
            j0 = round::floor(y as f64, 0) as f32;
            j1 = j0 + 1.0;

            s1 = x - i0;
            s0 = 1.0 - s1;
            t1 = y - j0;
            t0 = 1.0 - t1;

            let i0i: i32 = i0 as i32;
            let i1i: i32 = i1 as i32;
            let j0i: i32 = j0 as i32;
            let j1i: i32 = j1 as i32;
            d[ix(i, j) % ((N * N) - 1) as usize] = s0
                * (t0 * d0[ix(i0i, j0i) % ((N * N) - 1) as usize]
                    + t1 * d0[ix(i0i, j1i) % ((N * N) - 1) as usize])
                + s1 * (t0 * d0[ix(i1i, j0i) % ((N * N) - 1) as usize]
                    + t1 * d0[ix(i1i, j1i) % ((N * N) - 1) as usize]);
        }
    }

    set_bnd(b, d);
}

fn project(velocity_x: &mut [f32], velocity_y: &mut [f32], p: &mut [f32], div: &mut [f32]) {
    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            div[ix(i, j)] = -0.5
                * (velocity_x[ix(i + 1, j)] - velocity_x[ix(i - 1, j)] + velocity_y[ix(i, j + 1)]
                    - velocity_y[ix(i, j - 1)])
                / N as f32;
            p[ix(i, j)] = 0.;
        }
    }

    set_bnd(0, div);
    set_bnd(0, p);
    lin_solve(0, p, div, 1., 4.);

    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            velocity_x[ix(i, j)] -= 0.5 * (p[ix(i + 1, j)] - p[ix(i - 1, j)]) * N as f32;
            velocity_y[ix(i, j)] -= 0.5 * (p[ix(i, j + 1)] - p[ix(i, j - 1)]) * N as f32;
        }
    }
    set_bnd(1, velocity_x);
    set_bnd(2, velocity_y);
}
