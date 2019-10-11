//! TODO: doc
//!

pub mod bundle;
pub mod sync;

pub use legion::{prelude::*, *};
pub use sync::{LegionSystems as Systems, LegionWorld};

pub trait Consume {
    fn consume(
        self: Box<Self>,
        world: &mut legion::world::World,
        resources: &mut legion::resource::Resources,
        systems: &mut Systems,
    ) -> Result<(), amethyst_error::Error>;
}

pub trait SystemDesc: 'static {
    fn build(
        self,
        world: &mut legion::world::World,
        resources: &mut legion::resource::Resources,
    ) -> Box<dyn legion::system::Schedulable>;
}

pub trait SystemBundle {
    fn build(
        self,
        world: &mut legion::world::World,
        resources: &mut legion::resource::Resources,
        systems: &mut Systems,
    ) -> Result<(), amethyst_error::Error>;
}

pub struct SystemDescWrapper<B>(B)
where
    B: SystemDesc;

impl<B: SystemDesc> Consume for SystemDescWrapper<B> {
    fn consume(
        self: Box<Self>,
        world: &mut legion::world::World,
        resources: &mut legion::resource::Resources,
        systems: &mut Systems,
    ) -> Result<(), amethyst_error::Error> {
        // TODO: Stages enum
        systems.game.push(self.0.build(world, resources));
        Ok(())
    }
}

pub struct SystemBundleWrapper<B>(B)
where
    B: SystemBundle;

impl<B: SystemBundle> Consume for SystemBundleWrapper<B> {
    fn consume(
        self: Box<Self>,
        world: &mut legion::world::World,
        resources: &mut legion::resource::Resources,
        systems: &mut Systems,
    ) -> Result<(), amethyst_error::Error> {
        self.0.build(world, resources, systems)
    }
}
