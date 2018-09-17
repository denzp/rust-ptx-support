use core::cmp::PartialEq;
use core::ops::Deref;

pub struct Context;

#[derive(Debug)]
pub struct Block {
    dimensions: Size,
    index: Size,
}

#[derive(Debug)]
pub struct Thread {
    index: Size,
}

#[derive(Debug)]
pub struct Size {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl Context {
    pub fn block() -> Block {
        unsafe {
            Block {
                dimensions: Size {
                    x: nvptx_block_dim_x() as u64,
                    y: nvptx_block_dim_y() as u64,
                    z: nvptx_block_dim_z() as u64,
                },
                index: Size {
                    x: nvptx_block_idx_x() as u64,
                    y: nvptx_block_idx_y() as u64,
                    z: nvptx_block_idx_z() as u64,
                },
            }
        }
    }

    pub fn thread() -> Thread {
        unsafe {
            Thread {
                index: Size {
                    x: nvptx_thread_idx_x() as u64,
                    y: nvptx_thread_idx_y() as u64,
                    z: nvptx_thread_idx_z() as u64,
                },
            }
        }
    }
}

impl Block {
    pub fn index(&self) -> &Size {
        &self.index
    }

    pub fn dims(&self) -> &Size {
        &self.dimensions
    }
}

impl Deref for Thread {
    type Target = Size;

    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

impl Thread {
    pub fn index(&self) -> &Size {
        &self.index
    }
}

impl PartialEq<(u64, u64, u64)> for Size {
    fn eq(&self, other: &(u64, u64, u64)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == other.2
    }
}

impl PartialEq<(u64, u64, u64)> for &Size {
    fn eq(&self, other: &(u64, u64, u64)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == other.2
    }
}

impl PartialEq<(u64, u64)> for Size {
    fn eq(&self, other: &(u64, u64)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == 0
    }
}

impl PartialEq<(u64, u64)> for &Size {
    fn eq(&self, other: &(u64, u64)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == 0
    }
}

impl PartialEq<(u64)> for Size {
    fn eq(&self, other: &(u64)) -> bool {
        self.x == *other && self.y == 0 && self.z == 0
    }
}

impl PartialEq<(u64)> for &Size {
    fn eq(&self, other: &(u64)) -> bool {
        self.x == *other && self.y == 0 && self.z == 0
    }
}

extern "platform-intrinsic" {
    pub fn nvptx_block_idx_x() -> i32;
    pub fn nvptx_block_idx_y() -> i32;
    pub fn nvptx_block_idx_z() -> i32;

    pub fn nvptx_block_dim_x() -> i32;
    pub fn nvptx_block_dim_y() -> i32;
    pub fn nvptx_block_dim_z() -> i32;

    pub fn nvptx_thread_idx_x() -> i32;
    pub fn nvptx_thread_idx_y() -> i32;
    pub fn nvptx_thread_idx_z() -> i32;
}
