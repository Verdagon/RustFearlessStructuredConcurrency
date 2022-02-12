// This program shows the error we get when the child task tries to capture any non-Sync
// data from the parent scope.
//
// The borrow checker is right; for all we know, Firefly contains a RefCell or something else
// that's not thread-safe.

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

    // This trait object happens to not implement Sync, which causes the below error.
    // One might ask, "Can't we just refactor the program so that the entire World is Sync?"
    // which is only sometimes possible; we might contain third-party data which we cannot
    // make Sync.
    ship: Box<dyn Spaceship>

    // ... plus many other fields, which contain any number
    // of trait objects that may or may not have + Sync
}
impl World {
    fn new() -> World {
        World {
            some_int: 0,
            ship: Box::new(Firefly { })
        }
    }
}

async fn scoped_futures() {
    let not_copy = World::new();
    let not_copy_ref = &not_copy;

    let ((), vals) = async_scoped::AsyncScope::scope_and_block(|s| {
        for _ in 0..10 {
            let proc = || async {
                println!("Running a task! {:?}", not_copy_ref.some_int);
            };
            s.spawn(proc());
//            ^^^^^ `(dyn Spaceship + 'static)` cannot be shared between threads safely
//               = help: the trait `Sync` is not implemented for `(dyn Spaceship + 'static)`
//               = note: required because of the requirements on the impl of `Sync` for `Unique<(dyn Spaceship + 'static)>`
//               = note: required because it appears within the type `Box<(dyn Spaceship + 'static)>`
//            note: required because it appears within the type `World`
//              --> src/main.rs:13:8
        }
    });

    assert_eq!(vals.len(), 10);
}