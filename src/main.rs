use async_scoped::*;

#[async_std::main]
async fn main() {
    scoped_futures().await;
}

trait Spaceship { }

struct Firefly { }
impl Spaceship for Firefly { }

struct World {
    some_int: i64,
    // ship: Box<dyn Spaceship>
    // ... plus many other fields, which contain any number
    // of trait objects that may or may not have + Sync
}
impl World {
    fn new() -> World {
        World {
            some_int: 0,
            // ship: Box::new(Firefly { })
        }
    }
}

async fn scoped_futures() {
    let not_copy = World::new();
    let not_copy_ref = &not_copy;

    let ((), vals) = Scope::scope_and_block(|s: &mut async_scoped::Scope<'_, (), async_scoped::spawner::use_async_std::AsyncStd>| {
        for _ in 0..10 {
            let proc = || async {
                println!("Running a task! {:?}", not_copy_ref.some_int);
            };
            s.spawn(proc());
        }
    });

    assert_eq!(vals.len(), 10);
}
