use async_scoped::*;

#[async_std::main]
async fn main() {
    scoped_futures().await;
}

async fn scoped_futures() {
    let not_copy = String::from("hello");
    let not_copy_ref = &not_copy;

    let ((), vals) = Scope::scope_and_block(|s| {
        for _ in 0..10 {
            let proc = || async {
                println!("Running a task! {:?}", not_copy_ref);
            };
            s.spawn(proc());
        }
    });

    assert_eq!(vals.len(), 10);
}
