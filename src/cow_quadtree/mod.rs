use bevy::prelude::{Vec2, Resource};

use self::{quadtree_node::{CowQuadTreeNode, EntityData}, aabb::AABB};


pub mod quadtree_node;
pub mod aabb; 

#[derive(Resource)]
pub struct CowQuadTree {
    pub root : CowQuadTreeNode,
}

impl CowQuadTree {
    pub fn new(size : f32) -> CowQuadTree {
        let bounds = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(size, size));
        CowQuadTree {
            root : CowQuadTreeNode::new(bounds, 0),
        }
    }

    pub fn insert(&mut self, cow : EntityData) {
        self.root.insert(cow);
    }

    pub fn update(&mut self, cow : EntityData) {
        self.root.update(cow);
    }

    pub fn query(&self, bounds : AABB) -> Vec<(Vec2, f32)> {
        let mut result = Vec::new();
        self.root.query(&mut result, bounds);
        result
    }
}