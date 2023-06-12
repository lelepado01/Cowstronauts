use bevy::prelude::{Vec2, Entity};

use super::aabb::AABB;

#[derive(Clone, Copy)]
pub struct EntityData {
    pub position : Vec2,
    pub scale : f32,
    pub entity : Entity,
}

pub struct CowQuadTreeNode {
    pub depth : u32,
    pub children : Vec<CowQuadTreeNode>,
    pub cows : Vec<EntityData>,
    pub bounds : AABB,
}

const MAX_DEPTH : u32 = 5;

impl CowQuadTreeNode {
    pub fn new(bounds : AABB, depth : u32) -> CowQuadTreeNode {
        CowQuadTreeNode {
            depth : depth,
            children : Vec::new(),
            cows : Vec::new(),
            bounds : bounds,
        }
    }

    pub fn insert(&mut self, cow : EntityData) {
        if self.children.len() > 0 {
            let index = self.get_index(cow);
            if index != -1 {
                self.children[index as usize].insert(cow);
                return;
            }
        }

        self.cows.push(cow);

        if self.cows.len() > 4 && self.depth < MAX_DEPTH {
            if self.children.len() == 0 {
                self.split();
            }

            let mut i = 0;
            while i < self.cows.len() {
                let index = self.get_index(self.cows[i]);
                if index != -1 {
                    self.children[index as usize].insert(self.cows.remove(i));
                } else {
                    i += 1;
                }
            }
        }
    }

    pub fn split(&mut self) {
        let half_width = self.bounds.max.x - self.bounds.min.x;
        let half_height = self.bounds.max.y - self.bounds.min.y;

        let x = self.bounds.min.x;
        let y = self.bounds.min.y;

        self.children.push(CowQuadTreeNode::new(AABB::new(Vec2::new(x + half_width, y), Vec2::new(x + half_width * 2.0, y + half_height)), self.depth + 1));
        self.children.push(CowQuadTreeNode::new(AABB::new(Vec2::new(x, y), Vec2::new(x + half_width, y + half_height)), self.depth + 1));
        self.children.push(CowQuadTreeNode::new(AABB::new(Vec2::new(x, y + half_height), Vec2::new(x + half_width, y + half_height * 2.0)), self.depth + 1));
        self.children.push(CowQuadTreeNode::new(AABB::new(Vec2::new(x + half_width, y + half_height), Vec2::new(x + half_width * 2.0, y + half_height * 2.0)), self.depth + 1));
    }

    pub fn get_index(&self, cow : EntityData) -> i32 {
        let mut index = -1;

        let vertical_midpoint = self.bounds.min.x + (self.bounds.max.x - self.bounds.min.x) / 2.0;
        let horizontal_midpoint = self.bounds.min.y + (self.bounds.max.y - self.bounds.min.y) / 2.0;

        let mut top_quadrant = false;
        let mut bottom_quadrant = false;
        let mut left_quadrant = false;
        let mut right_quadrant = false;

        if cow.position.x > vertical_midpoint {
            right_quadrant = true;
        } else if cow.position.x < vertical_midpoint {
            left_quadrant = true;
        }

        if cow.position.y > horizontal_midpoint {
            top_quadrant = true;
        } else if cow.position.y < horizontal_midpoint {
            bottom_quadrant = true;
        }

        if top_quadrant {
            if right_quadrant {
                index = 0;
            } else if left_quadrant {
                index = 1;
            }
        } else if bottom_quadrant {
            if left_quadrant {
                index = 2;
            } else if right_quadrant {
                index = 3;
            }
        }

        index
    }

    pub fn query(&self, result : &mut Vec<(Vec2, f32)>, bounds : AABB) {
        if self.children.len() > 0 {
            
            for child in &self.children {
                if bounds.intersects(child.bounds){
                    child.query(result, bounds);
                }
            }
        } else {
            for cow in &self.cows {
                if bounds.contains(cow.position) {
                    result.push((cow.position, cow.scale));
                }
            }
        }
    }

    pub fn update(&mut self, cow : EntityData) {
        if self.children.len() > 0 {
            let index = self.get_index(cow);
            if index != -1 {
                self.children[index as usize].update(cow);
                return;
            }
        }

        for i in 0..self.cows.len() {
            if self.cows[i].entity == cow.entity {
                self.cows[i] = cow;
                return;
            }
        }
    }


}