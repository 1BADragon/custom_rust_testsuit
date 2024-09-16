use std::process::Command;

pub type ChildSpawn<T> = fn(&T) -> Command;

pub enum Spawner<T> {
    Attached(ChildSpawn<T>),
    Detached(ChildSpawn<T>),
}
