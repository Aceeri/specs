
extern crate specs;
#[macro_use]
extern crate specs_derive;

#[cfg(feature="serialize")]
extern crate serde;
#[cfg(feature="serialize")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature="serialize")]
extern crate serde_json;

macro_rules! call {
    // Top level calls
    /*
    ( $type:ident : $group:ty =>
        fn $method:ident
        [ $( $before:ty ),* ] in [ $( $after:ty ),* ]
        ( $( $args:expr ),* )
    ) => {
        call!( $type : $group =>
            fn $method
            [ $( $before ),* ] in [ $( $after ),* ]
            ( $( $args ),* );
    }
    */
    ( local: $group:ty => 
        fn $method:ident
        [ $( $before:ty ),* ] in [ $( $after:ty ),* ]
        ( $( $args:expr ),* )
    ) => {
        call!(
            $group : GroupLocals =>
            fn $method
            [ $( $before ),* ] in [ $( $after ),* ]
            ( $( $args ),* )
        );
    };

    // Helper methods
    ($group:ty : $group_trait:ty =>
        fn $method:ident
        [ $( $before:ty ),* ] in [ $( $after:ty ),* ]
        ( $( $args:expr ),* )
    ) => {
        call!(
            $group : $group_trait =>
            fn $method
            [ $( $before ),* ] in [ $( $after ),* ]
            ( $( $args ),* )
            { A B C D E F G H I J K L M N O P Q R S T U V W X Y Z } // Associated items
        )
    };
    ($group:ty : $group_trait:ty =>
        fn $method:ident
        [ $( $before:ty ),* ] in [ $( $after:ty ),* ]
        ( $( $args:expr ),* )
        { $( $left:tt )* }
    ) => {
        let mut counter = 0;
        $(
            if count < <$group as $group_trait>::used() {
                $method::<$( $before , )*, <$group as $group_trait>::$left $( , $after )*>( $( $args )* );   
                counter += 1;
            }
        )*
    };
}

#[cfg(feature="serialize")]
fn main() {
    use specs::prelude::*;
    use specs::{WorldDeserializer, WorldSerializer};
    use specs::entity::{ComponentGroup, SerializeGroup, GroupLocals};
    use serde::{Deserialize, Serialize};
    use serde::de::DeserializeSeed;

    #[derive(Debug, Serialize, Deserialize)]
    struct Comp1(String);
    impl Component for Comp1 {
        type Storage = VecStorage<Comp1>;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Comp2(f32);
    impl Component for Comp2 {
        type Storage = VecStorage<Comp2>;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Comp3(u32);
    impl Component for Comp3 {
        type Storage = VecStorage<Comp3>;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Comp4(u32);
    impl Component for Comp4 {
        type Storage = VecStorage<Comp4>;
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Comp5(u32);
    impl Component for Comp5 {
        type Storage = VecStorage<Comp5>;
    }

    #[derive(ComponentGroup)]
    #[allow(dead_code)]
    struct SomeGroup {
        #[group(serialize)]
        field1: Comp1,

        #[group(serialize)]
        field2: Comp2,

        field3: Comp3,

        #[group(subgroup)]
        test_sub: TestSub,
    }

    #[derive(ComponentGroup)]
    #[allow(dead_code)]
    struct TestSub {
        #[group(serialize)]
        field4: Comp4,

        field5: Comp5,
    }

    struct RemovalSystem;
    impl<'a, C> System<'a, C> for RemovalSystem {
        type SystemData = (
            Entities<'a>,
            WriteStorage<'a, Comp1>,
            WriteStorage<'a, Comp2>,
            WriteStorage<'a, Comp3>,
            WriteStorage<'a, Comp4>,
            WriteStorage<'a, Comp5>,
        );

        fn work(&mut self, mut data: Self::SystemData, _: C) {
            // Remove all components
            for (entity, _) in (&*data.0, &data.1.check()).join() {
                data.1.remove(entity);
            }
            for (entity, _) in (&*data.0, &data.2.check()).join() {
                data.2.remove(entity);
            }
            for (entity, _) in (&*data.0, &data.3.check()).join() {
                data.3.remove(entity);
            }
            for (entity, _) in (&*data.0, &data.4.check()).join() {
                data.4.remove(entity);
            }
            for (entity, _) in (&*data.0, &data.5.check()).join() {
                data.5.remove(entity);
            }
        }
    }

    // Running
    let mut world = World::new();
    world.register_group::<SomeGroup>();

    world.create_entity().with(Comp1("Nice".to_owned())).with(Comp4(500)).with(Comp5(501)).build();
    world.create_entity().with(Comp1("Nice".to_owned())).with(Comp2(5.0)).with(Comp3(1)).build();
    world.create_entity().with(Comp1("Nice".to_owned())).with(Comp3(2)).build();
    world.create_entity().with(Comp4(0)).with(Comp2(3.14159265358979)).build();

    let serialized = {
        let world_serializer = WorldSerializer::<SomeGroup>::new(&world);
        let serialized = serde_json::to_string_pretty(&world_serializer).unwrap();
        println!("{}", serialized);
        serialized
    };

    {
        let mut dispatcher = DispatcherBuilder::new()
            .add(RemovalSystem, "removal", &[])
            .build();

        dispatcher.dispatch(&mut world.res, ());
    }

    {
        let world_serializer = WorldSerializer::<SomeGroup>::new(&world);
        let serialized = serde_json::to_string_pretty(&world_serializer).unwrap();
        println!("before: {}", serialized);
    }

    {
        let entity_list: Vec<_> = {
            let entities = world.read_resource::<specs::Entities>();
            entities.join().collect()
        };

        let world_deserializer = WorldDeserializer::<SomeGroup>::new(&mut world, entity_list.as_slice());
        let mut json_deserializer = serde_json::Deserializer::from_str(&serialized);
        world_deserializer.deserialize(&mut json_deserializer);
    }

    {
        let world_serializer = WorldSerializer::<SomeGroup>::new(&world);
        let serialized = serde_json::to_string_pretty(&world_serializer).unwrap();
        println!("after: {}", serialized);
    }

    {
        use specs::entity::ComponentGroup;

        println!("locals:");
        for local in SomeGroup::local_components() {
            println!("{}", local);
        }
        println!("all:");
        for element in SomeGroup::components() {
            println!("{}", element);
        }
        println!("subgroups:");
        for subgroup in SomeGroup::subgroups() {
            println!("{}", subgroup);
        }
    }

    fn call_method<T>(s: &str) -> u32 {
        println!("Nice {}", s);
        42
    }
    
    fn call_other<T>(s: &str) -> u32 {
        println!("Static {}", s);
        3
    }

    println!("used: {:?}", <SomeGroup as GroupLocals>::used());
    call!(local: SomeGroup =>
        fn method  [ G1, G2 ] in [ G3, G4 ] ( arg1, arg2, arg3)
    );
}

#[cfg(not(feature="serialize"))]
fn main() {
    println!("Requires `serialize` flag to run");
}
