use crate::fluid::Fluid;
use crate::fluid_math::*;
use macroquad::prelude::*;
pub struct Emitter {
    pub posx: i32,
    pub posy: i32,
    pub dirx: i32,
    pub diry: i32,
    pub amount: f32,
    pub fluid: Fluid,
    empty: bool,
}

impl Emitter {
    pub fn new(
        posx: i32,
        posy: i32,
        dirx: i32,
        diry: i32,
        color: (u8, u8, u8),
        diffusion: f32,
        viscosity: f32,
        dt: f32,
        amount: f32,
    ) -> Self {
        Self {
            posx,
            posy,
            dirx,
            diry,
            fluid: Fluid::new(dt, diffusion, viscosity, color),
            amount,
            empty: false,
        }
    }
    pub fn empty() -> Self {
        Self {
            posx: 0,
            posy: 0,
            dirx: 0,
            diry: 0,
            fluid: Fluid::new(0., 0., 0., (0, 0, 0)),
            amount: 0.,
            empty: true,
        }
    }
    pub fn step(&mut self) {
        if !self.empty {
            self.fluid.step();
            self.fluid
                .add_density(self.posx / SCALE, self.posy / SCALE, self.amount);
            self.fluid.add_velocity(
                self.posx / SCALE,
                self.posy / SCALE,
                (self.posx / SCALE) as f32 - (self.dirx / SCALE) as f32,
                (self.posy / SCALE) as f32 - (self.diry / SCALE) as f32,
            );
        }
    }
    pub fn draw(&mut self) {
        if !self.empty {
            self.fluid.render_d();
        }
    }
    pub fn set_density(&mut self, density: f32) {
        self.amount = density;
    }
}
