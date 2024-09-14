use std::panic;
use std::thread;

use test_macro::my_test;

struct Test {
    name: &'static str,
    f: fn() -> Result<(), String>,
}

inventory::collect!(Test);

fn main() {
    for test in inventory::iter::<Test> {
        print!("Running test '{}'...", test.name);
        let test_thread = thread::Builder::new()
            .name(format!("runner {}", test.name))
            .spawn(|| {
                panic::set_hook(Box::new(|_| {}));
                (test.f)()
            })
            .expect("Failed to spawn test thread");

        let res = test_thread.join();

        match res {
            Ok(res) => match res {
                Ok(_) => println!("pass"),
                Err(s) => println!("fail: {}", s),
            },
            Err(e) => println!("panic: {:?}", e),
        }
    }
}

#[my_test(name = "panicing test")]
fn panicing_test() -> Result<(), String> {
    assert!(false);
    Ok(())
}

#[my_test(name = "test1")]
fn test1() -> Result<(), String> {
    Ok(())
}
