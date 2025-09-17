pub mod monitor;

use crate::monitor::{Mode, Monitor, PhysicalSize, Position, Size, Transform, VrrInfo};
use anyhow::{Context, Result};

pub fn parse_niri_outputs(outputs: &str) -> Result<Vec<Monitor>> {
    let mut monitors = Vec::new();
    let mut current_monitor: Option<Monitor> = None;
    let mut available_modes: Vec<Mode> = Vec::new();

    for line in outputs.lines() {
        let line_trimmed = line.trim();

        if line_trimmed.starts_with(r#"Output ""#) {
            if let Some(mut monitor) = current_monitor.take() {
                monitor.available_modes = available_modes.clone();
                monitors.push(monitor);
            }

            current_monitor = Some(parse_monitor_header(line_trimmed)?);
            available_modes.clear();
        } else if line_trimmed.starts_with("Current mode:") {
            if let Some(ref mut monitor) = current_monitor {
                monitor.current_mode = parse_mode(line_trimmed, true)?;
            }
        } else if line_trimmed.starts_with("Variable refresh rate:") {
            if let Some(ref mut monitor) = current_monitor {
                monitor.variable_refresh_rate = parse_vrr_info(line_trimmed)?;
            }
        } else if line_trimmed.starts_with("Physical size:") {
            if let Some(ref mut monitor) = current_monitor {
                monitor.physical_size = parse_physical_size(line_trimmed)?;
            }
        } else if line_trimmed.starts_with("Logical position:") {
            if let Some(ref mut monitor) = current_monitor {
                monitor.logical_position = parse_logical_position(line_trimmed)?;
            }
        } else if line_trimmed.starts_with("Logical size:") {
            if let Some(ref mut monitor) = current_monitor {
                monitor.logical_size = parse_logical_size(line_trimmed)?;
            }
        } else if line_trimmed.starts_with("Scale:") {
            if let Some(ref mut monitor) = current_monitor {
                monitor.current_scale = line.split(": ").nth(1).unwrap().parse::<f32>()?;
            }
        } else if line_trimmed.starts_with("Transform: ") {
            if let Some(ref mut monitor) = current_monitor {
                monitor.transform = parse_transform(line_trimmed)?;
            }
        } else if line.contains("@") && (line.contains("current") || line.contains("preferred")) {
            if let Some(ref mut monitor) = current_monitor {
                let response = parse_available_modes(line_trimmed)?;
                available_modes.push(response)
            }
        }
    }

    if let Some(mut monitor) = current_monitor {
        monitor.available_modes = available_modes;
        monitors.push(monitor);
    }

    Ok(monitors)
}

fn parse_monitor_header(header: &str) -> Result<Monitor> {
    let parts: Vec<&str> = header.split('"').collect();

    let name = Some(
        parts
            .get(1)
            .context("Could not parse monitor name")?
            .to_string(),
    );

    let output_name = header
        .split('(')
        .nth(1)
        .and_then(|s| s.split(')').next())
        .context("Could not parse output name")?
        .to_string();

    let response = Monitor {
        name,
        output_name,
        current_mode: Mode::default(),
        available_modes: Vec::new(),
        physical_size: PhysicalSize::default(),
        logical_position: Position::default(),
        logical_size: Size::default(),
        current_scale: 1.0,
        transform: Transform::default(),
        variable_refresh_rate: VrrInfo::default(),
    };

    Ok(response)
}

fn parse_mode(line: &str, is_current: bool) -> Result<Mode> {
    let mode_part = line
        .split(": ")
        .nth(1)
        .context("Could not parse current mode")?;
    let resolution_part = mode_part
        .split(" @ ")
        .next()
        .context("Could not parse resolution")?;
    let refresh_rate_part = mode_part
        .split(" @ ")
        .nth(1)
        .and_then(|s| s.split(" Hz").next())
        .context("Could not parse refresh rate")?;

    let (width, height) = parse_resolution(resolution_part)?;
    let refresh_rate = refresh_rate_part.parse::<f32>()?;
    let is_preferred = line.contains("preferred");

    Ok(Mode {
        width,
        height,
        refresh_rate,
        is_current,
        is_preferred,
    })
}

fn parse_vrr_info(line: &str) -> Result<VrrInfo> {
    let vrr_info_part = line
        .split(": ")
        .nth(1)
        .context("Could not parse vrr info")?;
    let is_supported = vrr_info_part.contains("supported");
    let is_enabled = vrr_info_part.contains("enabled");

    Ok(VrrInfo {
        is_supported,
        is_enabled,
    })
}

fn parse_physical_size(size: &str) -> Result<PhysicalSize> {
    let physical_size_part = size
        .split(": ")
        .nth(1)
        .and_then(|s| s.split(" mm").next())
        .context("Could not parse physical size")?;

    let (width, height) = parse_resolution(physical_size_part)?;

    Ok(PhysicalSize {
        width_mm: width,
        height_mm: height,
    })
}

fn parse_logical_position(position: &str) -> Result<Position> {
    let logical_position_part = position
        .split(": ")
        .nth(1)
        .context("Could not parse logical position")?;

    let (x, y) = logical_position_part
        .split_once(", ")
        .context("Invalid logical position format")?;

    Ok(Position {
        x: x.parse::<i32>()?,
        y: y.parse::<i32>()?,
    })
}

fn parse_logical_size(size: &str) -> Result<Size> {
    let logical_size_part = size
        .split(": ")
        .nth(1)
        .context("Could not parse logical size")?;

    let (width, height) = parse_resolution(logical_size_part)?;

    Ok(Size { width, height })
}

fn parse_transform(transform: &str) -> Result<Transform> {
    let transform_part = transform
        .split(": ")
        .nth(1)
        .context("Could not parse transform")?;

    // TODO verify correctness of transform
    Ok(match transform_part {
        "normal" => Transform::Normal,
        "Rotated 90" => Transform::Rotated90,
        "Rotated 180" => Transform::Rotated180,
        "Rotated 270" => Transform::Rotated270,
        "Flipped" => Transform::Flipped,
        "Flipped 90" => Transform::Flipped90,
        "Flipped 180" => Transform::Flipped180,
        "Flipped 270" => Transform::Flipped270,
        _ => Transform::Normal,
    })
}

fn parse_available_modes(line: &str) -> Result<Mode> {
    let resolution_refresh = line.split('(').next().context("Invalid mode line")?.trim();

    let flags = line
        .split('(')
        .nth(1)
        .and_then(|s| s.split(')').next())
        .unwrap_or("");

    let is_current = flags.contains("current");
    let is_preferred = flags.contains("preferred");

    let (resolution, refresh_str) = resolution_refresh
        .split_once('@')
        .context("Invalid mode format")?;

    let (width, height) = parse_resolution(resolution)?;

    let refresh_rate: f32 = refresh_str
        .parse()
        .context("Could not parse refresh rate")?;

    let mode = Mode {
        width,
        height,
        refresh_rate,
        is_current,
        is_preferred,
    };

    Ok(mode)
}

fn parse_resolution(resolution: &str) -> Result<(u32, u32)> {
    let (width_str, height_str) = resolution
        .split_once("x")
        .context("Invalid resolution format")?;

    let width = width_str.parse::<u32>().context("Invalid width")?;

    let height = height_str.parse::<u32>().context("Invalid height")?;

    Ok((width, height))
}
