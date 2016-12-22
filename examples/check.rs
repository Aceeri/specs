
extern crate specs;

use specs::*;

pub struct ExampleComp1 {
    field: bool,
}

impl Component for ExampleComp1 {
    type Storage = VecStorage<ExampleComp1>;
}


pub struct ExampleSystem;
impl System<()> for ExampleSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, mut comps) = arg.fetch(|w| {
            (w.entities(), w.write::<ExampleComp1>()) 
        });

        let checkjoin = &(&comps).check().clone();

        for (entity, _) in (&entities, checkjoin).iter() {
            println!("{:?}", entity);
            let comp = comps.get_mut(entity);
        }
    }
}

fn main() {
    let mut world = specs::World::new();
    world.register::<ExampleComp1>();

    world.create_now()
        .with::<ExampleComp1>(ExampleComp1 { field: true })
        .build();
    world.create_now()
        .build();
    world.create_now()
        .with::<ExampleComp1>(ExampleComp1 { field: true })
        .build();

    let mut planner = specs::Planner::new(world, 1);
    planner.add_system(ExampleSystem, "example", 1);

    planner.dispatch(());
    planner.wait();
}
