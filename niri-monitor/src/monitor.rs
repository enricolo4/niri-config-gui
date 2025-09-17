use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub name: Option<String>,
    pub output_name: String,
    pub current_mode: Mode,
    pub variable_refresh_rate: VrrInfo,
    pub physical_size: PhysicalSize,
    pub logical_position: Position,
    pub logical_size: Size,
    pub current_scale: f32,
    pub transform: Transform,
    pub available_modes: Vec<Mode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f32,
    pub is_current: bool,
    pub is_preferred: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalSize {
    pub width_mm: u32,
    pub height_mm: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrrInfo {
    pub is_supported: bool,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transform {
    Normal,
    Rotated90,
    Rotated180,
    Rotated270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            refresh_rate: 0.0,
            is_current: false,
            is_preferred: false,
        }
    }
}

impl Default for PhysicalSize {
    fn default() -> Self {
        Self {
            width_mm: 0,
            height_mm: 0,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}

impl Default for VrrInfo {
    fn default() -> Self {
        Self {
            is_supported: false,
            is_enabled: false,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::Normal
    }
}
