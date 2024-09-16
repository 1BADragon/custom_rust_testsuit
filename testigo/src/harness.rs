use crate::types::Spawner;
use std::{fmt::Display, marker::PhantomData, panic, sync::Arc, thread};
use testigo_types::Test;

pub struct Harness<T: Default + Sync + Send + 'static, E: Display + Send + Sync + 'static> {
    process_spawners: Vec<Spawner<T>>,

    suite_setup: Option<fn() -> T>,
    suite_teardown: Option<fn(&T) -> ()>,

    post_spawn_setup: Option<fn(&mut T) -> ()>,
    pre_join_teardown: Option<fn(&T) -> ()>,

    per_test_setup: Option<fn(&T) -> ()>,
    per_test_teardown: Option<fn(&T) -> ()>,
    _marker: PhantomData<E>,
}

impl<T: Default + Sync + Send + 'static, E: Display + Send + Sync + 'static> Harness<T, E> {
    pub(crate) fn new() -> Self {
        Self {
            process_spawners: Vec::new(),
            suite_setup: None,
            suite_teardown: None,
            post_spawn_setup: None,
            pre_join_teardown: None,
            per_test_setup: None,
            per_test_teardown: None,
            _marker: PhantomData {},
        }
    }

    pub(crate) fn add_child_spawners(&mut self, c: Vec<Spawner<T>>) {
        self.process_spawners = c
    }

    pub(crate) fn add_setup(&mut self, c: fn() -> T) {
        self.suite_setup = Some(c)
    }

    pub(crate) fn add_teardown(&mut self, c: fn(&T) -> ()) {
        self.suite_teardown = Some(c)
    }

    pub(crate) fn add_post_spawn_setup(&mut self, c: fn(&mut T) -> ()) {
        self.post_spawn_setup = Some(c)
    }

    pub(crate) fn add_pre_join_teardown(&mut self, c: fn(&T) -> ()) {
        self.pre_join_teardown = (Some(c))
    }

    pub(crate) fn add_test_setup(&mut self, c: fn(&T) -> ()) {
        self.per_test_setup = Some(c)
    }

    pub(crate) fn add_test_teardown(&mut self, c: fn(&T) -> ()) {
        self.per_test_teardown = Some(c)
    }

    pub fn execute(self) {
        let mut ctx = if let Some(setup) = self.suite_setup {
            setup()
        } else {
            T::default()
        };

        let mut child_processes = Vec::new();

        for spawner in self.process_spawners.iter() {
            match spawner {
                Spawner::Attached(p) => {
                    child_processes.push(p(&ctx).spawn().expect("Failed to spawn process"))
                }
                Spawner::Detached(p) => {
                    p(&ctx).spawn().expect("Failed to spawn process");
                }
            };
        }

        if let Some(post_spawn_setup) = self.post_spawn_setup {
            post_spawn_setup(&mut ctx)
        }

        let ctx = Arc::new(ctx);

        for test in inventory::iter::<Test<T, E>> {
            print!("Running test '{}'...", test.name);

            if let Some(test_setup) = self.per_test_setup {
                test_setup(ctx.as_ref());
            }

            let ctx_clone = Arc::clone(&ctx);
            let test_thread = thread::Builder::new()
                .name(format!("runner {}", test.name))
                .spawn(move || {
                    panic::set_hook(Box::new(|_| {}));
                    test.call(ctx_clone.as_ref())
                })
                .expect("Failed to spawn test thread");

            let res = test_thread.join();

            if let Some(test_teardown) = self.per_test_teardown {
                test_teardown(ctx.as_ref());
            }

            match res {
                Ok(res) => match res {
                    Ok(_) => println!("pass"),
                    Err(s) => println!("fail: {}", s),
                },
                Err(e) => println!("panic: {:?}", e),
            }
        }

        if let Some(pre_join_teardown) = self.pre_join_teardown {
            pre_join_teardown(ctx.as_ref())
        }

        child_processes.iter_mut().for_each(|c| {
            let _ = c.kill();
        });

        child_processes.into_iter().for_each(|mut c| {
            c.wait().expect("Failed to wait");
        });

        if let Some(teardown) = self.suite_teardown {
            teardown(ctx.as_ref());
        }
    }
}
