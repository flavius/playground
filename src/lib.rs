use std::ops::{Deref, DerefMut};
use std::any::{Any, TypeId};

pub mod plugin;
mod application;
pub use application::Application;

pub use plugin::{MyPlugin, Plugin, PluginList};
