use std::{fmt::Display, process::Command};

use crate::{harness::Harness, types::Spawner};

pub struct Builder<T: Default + Send + Sync + 'static> {
    processes: Vec<Spawner<T>>,
    setup: Option<fn() -> T>,
    teardown: Option<fn(&T) -> ()>,

    post_spawn_setup: Option<fn(&mut T) -> ()>,
    pre_join_teardown: Option<fn(&T) -> ()>,

    test_setup: Option<fn(&T) -> ()>,
    test_teardown: Option<fn(&T) -> ()>,
}

impl<T: Default + Send + Sync + 'static> Builder<T> {
    pub fn new() -> Builder<T> {
        Builder {
            processes: Vec::new(),
            setup: None,
            teardown: None,
            post_spawn_setup: None,
            pre_join_teardown: None,
            test_setup: None,
            test_teardown: None,
        }
    }

    pub fn with_attached(mut self, f: fn(&T) -> Command) -> Self {
        self.processes.push(Spawner::Attached(f));
        self
    }

    pub fn with_detached(mut self, f: fn(&T) -> Command) -> Self {
        self.processes.push(Spawner::Detached(f));
        self
    }

    pub fn with_setup(mut self, f: fn() -> T) -> Self {
        self.setup = Some(f);
        self
    }

    pub fn with_teardown(mut self, f: fn(&T) -> ()) -> Self {
        self.teardown = Some(f);
        self
    }

    pub fn with_post_spawn_setup(mut self, c: fn(&mut T) -> ()) -> Self {
        self.post_spawn_setup = Some(c);
        self
    }

    pub fn with_pre_join_teardown(mut self, c: fn(&T) -> ()) -> Self {
        self.pre_join_teardown = Some(c);
        self
    }

    pub fn with_test_setup(mut self, f: fn(&T) -> ()) -> Self {
        self.test_setup = Some(f);
        self
    }

    pub fn with_test_teardown(mut self, f: fn(&T) -> ()) -> Self {
        self.test_teardown = Some(f);
        self
    }

    pub fn build<E: Display + Send + Sync + 'static>(self) -> Harness<T, E> {
        let mut h = Harness::<T, E>::new();

        h.add_child_spawners(self.processes);
        if let Some(setup) = self.setup {
            h.add_setup(setup);
        }

        if let Some(teardown) = self.teardown {
            h.add_teardown(teardown);
        }

        if let Some(test_setup) = self.test_setup {
            h.add_test_setup(test_setup)
        }

        if let Some(test_teardown) = self.test_teardown {
            h.add_test_teardown(test_teardown)
        }

        h
    }
}
