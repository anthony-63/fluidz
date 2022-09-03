use math::round;

pub const N: i32 = 64;
pub const SCALE: i32 = 14;
pub const ITER: i32 = 1;

pub fn diffuse(b: i32, x: &mut [f32], x0: &mut [f32], diff: f32, dt: f32) {
    let a = dt * diff * (N as f32 - 2.) * (N as f32 - 2.);
    lin_solve(b, x, x0, a, 1. + 6. * a);
}

pub fn lin_solve(b: i32, x: &mut [f32], x0: &mut [f32], a: f32, c: f32) {
    let c_recip = 1.0 / c;
    for _k in 0..ITER {
        for j in 1..(N - 1) {
            for i in 1..(N - 1) {
                x[((i + j * N) as usize)] = (x0[((i + j * N) as usize)]
                    + a * (x[(((i + 1) + j * N) as usize)]
                        + x[(((i - 1) + j * N) as usize)]
                        + x[((i + (j + 1) * N) as usize)]
                        + x[((i + (j - 1) * N) as usize)]))
                    * c_recip;
            }
        }

        set_bnd(b, x);
    }
}

pub fn set_bnd(b: i32, x: &mut [f32]) {
    for i in 1..(N - 1) {
        x[((i + 0 * N) as usize)] = if b == 2 {
            -x[((i + 1 * N) as usize)]
        } else {
            x[((i + 1 * N) as usize)]
        };
        x[((i + (N - 1) * N) as usize)] = if b == 2 {
            -x[((i + (N - 2) * N) as usize)]
        } else {
            x[((i + (N - 2) * N) as usize)]
        };
    }
    for j in 1..(N - 1) {
        x[((0 + j * N) as usize)] = if b == 1 {
            -x[((1 + j * N) as usize)]
        } else {
            x[((1 + j * N) as usize)]
        };
        x[(((N - 1) + j * N) as usize)] = if b == 1 {
            -x[(((N - 2) + j * N) as usize)]
        } else {
            x[(((N - 2) + j * N) as usize)]
        };
    }

    x[((0 + 0 * N) as usize)] = 0.5 * (x[((1 + 0 * N) as usize)] + x[((0 + 1 * N) as usize)]);
    x[((0 + (N - 1) * N) as usize)] =
        0.5 * (x[((1 + (N - 1) * N) as usize)] + x[((0 + (N - 2) * N) as usize)]);
    x[(((N - 1) + 0 * N) as usize)] =
        0.5 * (x[(((N - 2) + 0 * N) as usize)] + x[(((N - 1) + 1 * N) as usize)]);
    x[(((N - 1) + (N - 1) * N) as usize)] =
        0.5 * (x[(((N - 2) + (N - 1) * N) as usize)] + x[(((N - 1) + (N - 2) * N) as usize)]);
}

pub fn advect(
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
            tmp1 = dtx * velocity_x[((i + j * N) as usize)];
            tmp2 = dty * velocity_y[((i + j * N) as usize)];
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
            d[((i + j * N) as usize) % ((N * N) - 1) as usize] = s0
                * (t0 * d0[((i0i + j0i * N) as usize) % ((N * N) - 1) as usize]
                    + t1 * d0[((i0i + j1i * N) as usize) % ((N * N) - 1) as usize])
                + s1 * (t0 * d0[((i1i + j0i * N) as usize) % ((N * N) - 1) as usize]
                    + t1 * d0[((i1i + j1i * N) as usize) % ((N * N) - 1) as usize]);
        }
    }

    set_bnd(b, d);
}

pub fn project(velocity_x: &mut [f32], velocity_y: &mut [f32], p: &mut [f32], div: &mut [f32]) {
    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            div[((i + j * N) as usize)] = -0.5
                * (velocity_x[(((i + 1) + j * N) as usize)]
                    - velocity_x[(((i - 1) + j * N) as usize)]
                    + velocity_y[((i + (j + 1) * N) as usize)]
                    - velocity_y[((i + (j - 1) * N) as usize)])
                / N as f32;
            p[((i + j * N) as usize)] = 0.;
        }
    }

    set_bnd(0, div);
    set_bnd(0, p);
    lin_solve(0, p, div, 1., 4.);

    for j in 1..(N - 1) {
        for i in 1..(N - 1) {
            velocity_x[((i + j * N) as usize)] -= 0.5
                * (p[(((i + 1) + j * N) as usize)] - p[(((i - 1) + j * N) as usize)])
                * N as f32;
            velocity_y[((i + j * N) as usize)] -= 0.5
                * (p[((i + (j + 1) * N) as usize)] - p[((i + (j - 1) * N) as usize)])
                * N as f32;
        }
    }
    set_bnd(1, velocity_x);
    set_bnd(2, velocity_y);
}
