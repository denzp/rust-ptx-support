use core::arch::nvptx::*;
use core::cmp::PartialEq;
use core::ops::Deref;

pub struct Context;

#[derive(Debug)]
pub struct Block {
    dimensions: Vec3,
    index: Vec3,
}

#[derive(Debug)]
pub struct Thread {
    index: Vec3,
}

#[derive(Debug)]
pub struct Vec3 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl Context {
    pub fn block() -> Block {
        unsafe {
            Block {
                dimensions: Vec3 {
                    x: _block_dim_x() as u64,
                    y: _block_dim_y() as u64,
                    z: _block_dim_z() as u64,
                },
                index: Vec3 {
                    x: _block_idx_x() as u64,
                    y: _block_idx_y() as u64,
                    z: _block_idx_z() as u64,
                },
            }
        }
    }

    pub fn thread() -> Thread {
        unsafe {
            Thread {
                index: Vec3 {
                    x: _thread_idx_x() as u64,
                    y: _thread_idx_y() as u64,
                    z: _thread_idx_z() as u64,
                },
            }
        }
    }
}

impl Block {
    pub fn index(&self) -> &Vec3 {
        &self.index
    }

    pub fn dims(&self) -> &Vec3 {
        &self.dimensions
    }
}

impl Deref for Thread {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

impl Thread {
    pub fn index(&self) -> &Vec3 {
        &self.index
    }
}

impl PartialEq<(u64, u64, u64)> for Vec3 {
    fn eq(&self, other: &(u64, u64, u64)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == other.2
    }
}

impl PartialEq<(u64, u64, u64)> for &Vec3 {
    fn eq(&self, other: &(u64, u64, u64)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == other.2
    }
}

impl PartialEq<(u64, u64)> for Vec3 {
    fn eq(&self, other: &(u64, u64)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == 0
    }
}

impl PartialEq<(u64, u64)> for &Vec3 {
    fn eq(&self, other: &(u64, u64)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == 0
    }
}

impl PartialEq<(u64)> for Vec3 {
    fn eq(&self, other: &(u64)) -> bool {
        self.x == *other && self.y == 0 && self.z == 0
    }
}

impl PartialEq<(u64)> for &Vec3 {
    fn eq(&self, other: &(u64)) -> bool {
        self.x == *other && self.y == 0 && self.z == 0
    }
}
