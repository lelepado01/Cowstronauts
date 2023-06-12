use bevy::prelude::Vec2;

#[derive(Clone, Copy)]
pub struct AABB {
    pub min : Vec2,
    pub max : Vec2,
}

impl AABB {
    pub fn new(min : Vec2, max : Vec2) -> AABB {
        AABB {
            min : min,
            max : max,
        }
    }

    pub fn contains(&self, point : Vec2) -> bool {
        point.x >= self.min.x && point.x <= self.max.x && point.y >= self.min.y && point.y <= self.max.y
    }

    pub fn intersects(&self, other : AABB) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x && self.min.y <= other.max.y && self.max.y >= other.min.y
    }
}