use inventory::{Collect, Registry};

pub struct Test<C: 'static, E: 'static> {
    pub name: &'static str,
    pub func: fn(&C) -> Result<(), E>,
}

impl<C: 'static, E: 'static> Test<C, E> {
    pub const fn new(name: &'static str, func: fn(&C) -> Result<(), E>) -> Self {
        Test { name, func }
    }

    pub fn call(&self, ctx: &C) -> Result<(), E> {
        (self.func)(ctx)
    }
}

impl<T: 'static, E: 'static> Collect for Test<T, E> {
    #[inline]
    fn registry() -> &'static Registry {
        static REGISTRY: Registry = Registry::new();
        &REGISTRY
    }
}
