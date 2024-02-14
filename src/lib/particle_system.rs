use crate::{Position, Renderable};

use super::{ParticleLifetime, Rltk};
use rltk::RGB;
use specs::prelude::*;
// 粒子系统

// 一个粒子就是一个请求
struct ParticleRequest {
    x: i32,
    y: i32,
    fg: RGB,
    bg: RGB,
    glyph: rltk::FontCharType,
    lifetime: f32,
}

pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder {
            requests: Vec::new(),
        }
    }

    // 粒子服务
    pub fn request(
        &mut self,
        x: i32,
        y: i32,
        fg: RGB,
        bg: RGB,
        glyph: rltk::FontCharType,
        lifetime: f32,
    ) {
        self.requests.push(ParticleRequest {
            x,
            y,
            fg,
            bg,
            glyph,
            lifetime,
        });
    }
}

// 粒子在生命周期结束后消失
pub fn cull_dead_particles(ecs: &mut World, ctx: &Rltk) {
    let mut dead_particles: Vec<Entity> = Vec::new();
    {
        // age out particles
        let mut particles = ecs.write_storage::<ParticleLifetime>();
        let entities = ecs.entities();
        for (entity, mut particle) in (&entities, &mut particles).join() {
            particle.lifetime_ms -= ctx.frame_time_ms;
            if particle.lifetime_ms < 0.0 {
                dead_particles.push(entity);
            }
        }
    }

    for dead in dead_particles.iter() {
        ecs.delete_entity(*dead).expect("Particle will not dead");
    }
}

// 实际的粒子系统
pub struct ParticleSpawnSystem {}
impl<'a> System<'a> for ParticleSpawnSystem {
    #[allow(clippy::type_complexity)]
    // 这个粒子系统需要哪些数据，位置，渲染，生命周期，粒子的Builder
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, ParticleLifetime>,
        WriteExpect<'a, ParticleBuilder>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut positions, mut renderables, mut particles, mut particle_builder) = data;
        // 遍历粒子builder 中的所有的粒子请求
        for new_particle in particle_builder.requests.iter() {
            // 创建一个实体
            let p = entities.create();
            // 将这个实体插入到组件存储器中, 并给组件赋值
            positions
                .insert(
                    p,
                    Position {
                        x: new_particle.x,
                        y: new_particle.y,
                    },
                )
                .expect("Unable to inser position");
            renderables
                .insert(
                    p,
                    Renderable {
                        fg: new_particle.fg,
                        bg: new_particle.bg,
                        glyph: new_particle.glyph,
                        render_order: 0,
                    },
                )
                .expect("Unable to insert renderable");
            particles
                .insert(
                    p,
                    ParticleLifetime {
                        lifetime_ms: new_particle.lifetime,
                    },
                )
                .expect("Unable to insert lifetime");
        }
        // 这是一个非常简单的服务：它迭代请求，并使用请求中的组件参数为每个粒子创建一个实体。然后它会清除构建器列表。
        particle_builder.requests.clear();
    }
}
