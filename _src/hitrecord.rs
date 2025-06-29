use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SideEnum {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

#[derive(PartialEq, Debug, Clone)]
pub struct HitRecord {
    pub hitpoint: Vec3f,
    pub key: Vec3i,
    pub tile_key: Vec3i,
    pub distance: f32,
    pub normal: Vec3f,
    pub uv: Vec3f,
    pub value: (u8, u8),
    pub side: SideEnum,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            hitpoint: Vec3f::zero(),
            key: Vec3i::zero(),
            tile_key: Vec3i::zero(),
            distance: 0.0,
            normal: Vec3f::zero(),
            uv: Vec3f::zero(),
            value: (0, 0),
            side: SideEnum::Top,
        }
    }

    pub fn compute_side(&mut self) {
        if self.normal.y > 0.5 {
            self.side = SideEnum::Bottom;
        } else if self.normal.y < -0.5 {
            self.side = SideEnum::Top;
        } else if self.normal.x > 0.5 {
            self.side = SideEnum::Left;
        } else if self.normal.x < -0.5 {
            self.side = SideEnum::Right;
        } else if self.normal.z > 0.5 {
            self.side = SideEnum::Back;
        } else if self.normal.z < -0.5 {
            self.side = SideEnum::Front;
        }
    }

    /*
    pub fn get_hitpoint(&mut self) -> ScriptVec3f {
        ScriptVec3f::from_vec3f(self.hitpoint)
    }

    pub fn get_key(&mut self) -> ScriptVec3i {
        ScriptVec3i::from_vec3i(self.key)
    }

    pub fn get_tile_key(&mut self) -> ScriptVec3i {
        ScriptVec3i::from_vec3i(self.tile_key)
    }

    pub fn get_normal(&mut self) -> ScriptVec3f {
        ScriptVec3f::from_vec3f(self.normal)
    }*/

    // pub fn get_value(&mut self) -> i32 {
    //     self.value as i32
    // }

    pub fn get_side(&mut self) -> SideEnum {
        self.side.clone()
    }

    /*
    /// Register to the engine
    pub fn register(engine: &mut Engine) {
        engine.register_type_with_name::<HitRecord>("HitRecord")
            .register_get("hitpoint", HitRecord::get_hitpoint)
            .register_get("key", HitRecord::get_key)
            .register_get("tile_key", HitRecord::get_tile_key)
            .register_get("normal", HitRecord::get_normal)
            //.register_get("value", HitRecord::get_value)
            .register_get("side", HitRecord::get_side);
    }*/
}

/*
impl FuncArgs for HitRecord {
    fn parse<C: Extend<rhai::Dynamic>>(self, container: &mut C) {
        container.extend(once(rhai::Dynamic::from(self)));
    }
}*/
