
#[macro_use]
extern crate specs;

use specs::*;

#[derive(Default)]
pub struct Other;
impl<T> Metadata<T> for Other { }

#[derive(Debug, Default)]
struct TestComp;
impl Component for TestComp {
    type Storage = DenseVecStorage<Self>;
    type Metadata = metadata! [ Flagged, Other ];
}

fn main() {
    let mut world = World::new();
    world.register::<TestComp>();

    let e1 = world.create_entity()
        .with(TestComp)
        .build();
    let _e2 = world.create_entity()
        .with(TestComp)
        .build();
    let _e3 = world.create_entity()
        .with(TestComp)
        .build();
    let e4 = world.create_entity()
        .with(TestComp)
        .build();
    {
        let mut storage = world.write::<TestComp>();
        storage.find_mut::<Flagged, _>().clear_flags();
        storage.get_mut(e1);
        storage.get_mut(e4);
    }

    {
        let storage = world.read::<TestComp>();

        // Grab the metadata.
        let flagged = storage.find::<Flagged, _>();

        for (entity, comp, _) in (&*world.entities(), &storage, flagged).join() {
            println!("({:?}, {:?}): {:?}", entity.id(), entity.gen().id(), comp);
        }
    }

    {
        let mut storage = world.write::<TestComp>();
        storage.find::<Flagged, _>();
        let _inner_flagged: &mut Flagged = storage.find_mut::<Other, _>().find_mut();
    }

    {
        let mut storage = world.write::<TestComp>();

        {
            // Grab the metadata and clone it to avoid borrow issues
            let flagged = storage.find::<Flagged, _>().clone();

            // Iterate over all flagged components
            for (entity, comp, _) in (&*world.entities(), &mut storage, flagged).join() {
                println!("({:?}, {:?}): {:?}", entity.id(), entity.gen().id(), comp);
            }
        }
    }
}
