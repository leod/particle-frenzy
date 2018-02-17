#[macro_use]
extern crate gfx;

mod shaders;

use gfx::traits::FactoryExt;

// As in the ggez gfx example:
//     https://github.com/ggez/ggez/blob/master/examples/cube.rs
// ColorFormat and DepthFormat are hardwired into ggez's drawing code,
// and there isn't a way to easily change them, so for the moment we just have
// to know what they are and use the same settings.
type ColorFormat = gfx::format::Srgba8;

// Based on the gfx example code for particles:
//     https://github.com/gfx-rs/gfx/blob/master/examples/support/particle/main.rs

gfx_defines! {
    // A `Vertex` stores the lifetime and the initial state of one particle.
    vertex Particle {
        spawn_time: f32 = "a_SpawnTime",
        lifetime: f32 = "a_DeathTime",

        pos: [f32; 2] = "a_Pos",
        vel: [f32; 2] = "a_Vel",
        angle: f32 = "a_Angle",
        angular_vel: f32 = "a_AngularVel",
        color: [f32; 3] = "a_Color",
        size: f32 = "a_Size",
    }

    constant Globals {
        time: f32 = "u_Time",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Particle> = (),
        globals: gfx::ConstantBuffer<Globals> = "Locals",
        out_color: gfx::BlendTarget<ColorFormat> = (
            "Target0",
            gfx::state::ColorMask::all(),
            gfx::preset::blend::ALPHA,
        ),
    }
}

/// Keeps a buffer of particle vertices.
struct Buffer<R: gfx::Resources> {
    /// GPU-side particle state.
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,

    /// Time at which our most long-living particle dies.
    max_death_time: f32,
}

/// Manages multiple buffers to store particles and allows for rendering them.
pub struct ParticleSystem<R: gfx::Resources> {
    buffer_size: usize,

    target: gfx::handle::RenderTargetView<R, ColorFormat>,
    pso: gfx::PipelineState<R, pipe::Meta>,
    globals: gfx::handle::Buffer<R, Globals>,

    /// Ring buffer of particles.
    buffers: Vec<Buffer<R>>,

    /// New particles that will be inserted with the next render call.
    new_particles: Vec<Particle>,

    /// Index into `buffers` at which the next particle will be inserted.
    next_index: (usize, usize),
}

impl<R: gfx::Resources> ParticleSystem<R> {
    /// Create a new particle system, pre-allocating the specified number of buffers.
    pub fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        target: gfx::handle::RenderTargetView<R, ColorFormat>,
        buffer_size: usize,
        num_initial_buffers: usize,
    ) -> Self {
        assert!(
            num_initial_buffers > 0,
            "ParticleSystem must be initialized with at least one buffer"
        );

        let vs = factory.create_shader_vertex(shaders::VERTEX).unwrap();
        let gs = factory.create_shader_geometry(shaders::GEOMETRY).unwrap();
        let ps = factory.create_shader_pixel(shaders::PIXEL).unwrap();
        let shader_set = gfx::ShaderSet::Geometry(vs, gs, ps);

        let pso = factory
            .create_pipeline_state(
                &shader_set,
                gfx::Primitive::PointList,
                gfx::state::Rasterizer::new_fill(),
                pipe::new(),
            )
            .unwrap();
        let globals = factory.create_constant_buffer(1);

        let mut system = ParticleSystem {
            target,
            pso,
            globals,
            buffer_size,
            buffers: Vec::new(),
            new_particles: Vec::new(),
            next_index: (0, 0),
        };

        for _ in 0..num_initial_buffers {
            system.new_buffer(factory);
        }

        system
    }

    /// Add a new particle to be created with the next render call.
    pub fn spawn(&mut self, particle: &Particle) {
        self.new_particles.push(particle.clone());
    }

    pub fn render<C: gfx::CommandBuffer<R>, F: gfx::Factory<R>>(
        &mut self,
        _factory: &mut F,
        encoder: &mut gfx::Encoder<R, C>,
        cur_time: f32,
    ) {
        // Create new particles, filling up the ring buffer
        let mut i = 0;
        while i < self.new_particles.len() {
            // We always point at a buffer that has some space left
            assert!(self.next_index.1 < self.buffer_size);

            // Contiguously fill up the current buffer as much as possible
            let buffer_space = self.buffer_size - self.next_index.1;
            let need_space = self.new_particles.len() - i;
            let num_copy = buffer_space.min(need_space);

            let copy_slice = &self.new_particles[i..i + num_copy];

            encoder
                .update_buffer(
                    &self.buffers[self.next_index.0].data.vbuf,
                    copy_slice,
                    self.next_index.1,
                )
                .unwrap();

            // Keep track of how alive the buffers are, so that we can ignore dead ones in
            // rendering
            self.buffers[self.next_index.0].max_death_time = copy_slice
                .iter()
                .map(|particle| particle.spawn_time + particle.lifetime)
                .fold(0.0, f32::max);

            self.next_index = if num_copy == buffer_space {
                // We have filled up this buffer completely, move on to the next one

                // TODO: Should create new buffers here if the demand is too large.
                //       For example, create a new buffer if we wraparound to a buffer
                //       that is active.

                let buffer_idx = (self.next_index.0 + 1) % self.buffers.len();
                (buffer_idx, 0)
            } else {
                // There weren't enough particles to fill up the current buffer
                (self.next_index.0, self.next_index.1 + num_copy)
            };

            i += num_copy;
        }

        // Draw all the active buffers
        for buffer in &self.buffers {
            if buffer.max_death_time > cur_time {
                encoder.draw(&buffer.slice, &self.pso, &buffer.data);
            }
        }
    }

    fn new_buffer<F: gfx::Factory<R>>(&mut self, factory: &mut F) {
        let vbuf = factory
            .create_buffer(
                self.buffer_size,
                gfx::buffer::Role::Vertex,
                gfx::memory::Usage::Dynamic,
                gfx::memory::Bind::empty(),
            )
            .unwrap();
        let data = pipe::Data {
            vbuf,
            globals: self.globals.clone(),
            out_color: self.target.clone(),
        };
        let slice = gfx::Slice::new_match_vertex_buffer(&data.vbuf);

        let buffer = Buffer {
            data,
            slice,
            max_death_time: 0.0,
        };

        self.buffers.push(buffer);
    }
}
