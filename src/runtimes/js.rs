use bevy::{
    asset::Asset,
    ecs::{component::Component, entity::Entity, resource::Resource, schedule::ScheduleLabel},
    math::Vec3,
    reflect::TypePath,
};

use boa_engine::Context;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::{
    ENTITY_VAR_NAME, FuncArgs, Runtime, ScriptingError,
    assets::GetExtensions,
    callback::{FromRuntimeValueWithEngine, IntoRuntimeValueWithEngine},
    promise::Promise,
};


type JsEngine = Arc<Mutex<Context>>;


#[derive(Resource)]
pub struct JsRuntime{
    engine:JsEngine
}

#[derive(Debug, Clone, Copy)]
pub struct BevyEntity(pub Entity);

impl BevyEntity {
    pub fn index(&self) -> u32 {
        self.0.index()
    }
}