#![feature(test)]

extern crate test;
extern crate specs;

static NUM: u32 = 10_000_000;

struct CompInt(u32);
struct CompBool(f32);

impl Default for CompInt {
    fn default() -> Self {
        CompInt(0)
    }
}

impl Default for CompBool {
    fn default() -> Self {
        CompBool(true)
    }
}

macro_rules! setup {
    ($num:expr => [ $( $comp:ty = $sparsity:expr ),* ] ) => {
        let mut w = World::new();
        $(
            w.register::<$comp>();
        )*
        let mut eids: Vec<_> = (0..$num)
            .map(|i| {
                let mut builder = w.create_entity();

                $(
                    if i % $sparsity == 0 {
                        builder.with(<$comp>::default());
                    }
                )*
                
                builder.build()
            })
            .collect();
    }
}

macro_rules! tests {
    ($storage:ident [ $( $generics:tt ),* ]) => {
        impl Component for CompInt {
            type Storage = $storage<CompInt $( , $generics )*>;
        }
        impl Component for CompBool {
            type Storage = $storage<CompInt $( , $generics )*>;
        }

        // no gaps
        #[bench]
        fn straight(bencher: &mut Bencher) {
            setup!(NUM => [ CompInt = 1, CompBool = 1 ]);

            bencher.iter({
                
            });
        }
    }
}

#[test]
mod vec {
    tests!(VecStorage [T]);
}
