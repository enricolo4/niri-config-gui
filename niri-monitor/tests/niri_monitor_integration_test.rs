use niri_monitor::*;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_OUTPUT: &str = r#"Output "LG Electronics 34GL750 0x01010101" (DP-2)
  Current mode: 2560x1080 @ 144.001 Hz (preferred)
  Variable refresh rate: supported, disabled
  Physical size: 800x340 mm
  Logical position: 1920, 0
  Logical size: 2560x1080
  Scale: 1
  Transform: normal
  Available modes:
    2560x1080@144.001 (current)
    2560x1080@120.000 (preferred)


Output "Samsung Electric Company LF24T35 HX5T502871" (HDMI-A-1)
  Current mode: 1920x1080 @ 60.000 Hz
  Variable refresh rate: not supported
  Physical size: 530x300 mm
  Logical position: 0, 0
  Logical size: 1920x1080
  Scale: 1
  Transform: normal
  Available modes:
    1920x1080@60.000 (preferred)
    1920x1080@74.973
    1920x1080@60.000 (current)"#;

    #[test]
    fn test_parse_niri_outputs_sucess() {
        let response = parse_niri_outputs(SAMPLE_OUTPUT).expect("Could not parse outputs");

        assert_eq!(response.len(), 2);

        assert_eq!(
            response[0].name,
            Some("LG Electronics 34GL750 0x01010101".to_string())
        );
        assert_eq!(response[0].output_name, "DP-2".to_string());
        assert_eq!(response[0].current_mode.width, 2560);
        assert_eq!(response[0].current_mode.height, 1080);
        assert_eq!(response[0].current_mode.refresh_rate, 144.001);
        assert_eq!(response[0].available_modes.len(), 2);
        assert_eq!(response[0].available_modes[0].width, 2560);
        assert_eq!(response[0].available_modes[0].height, 1080);
        assert_eq!(response[0].available_modes[0].refresh_rate, 144.001);
        assert!(response[0].available_modes[0].is_current);
        assert!(!response[0].available_modes[0].is_preferred);
        assert_eq!(response[0].available_modes[1].width, 2560);
        assert_eq!(response[0].available_modes[1].height, 1080);
        assert_eq!(response[0].available_modes[1].refresh_rate, 120.000);
        assert!(!response[0].available_modes[1].is_current);
        assert!(response[0].available_modes[1].is_preferred);
        assert_eq!(response[0].physical_size.width_mm, 800);
        assert_eq!(response[0].physical_size.height_mm, 340);
        assert_eq!(response[0].logical_position.x, 1920);
        assert_eq!(response[0].logical_position.y, 0);
        assert_eq!(response[0].logical_size.width, 2560);
        assert_eq!(response[0].logical_size.height, 1080);
        assert_eq!(response[0].current_scale, 1.0);
        assert!(response[0].variable_refresh_rate.is_supported);
        assert!(!response[0].variable_refresh_rate.is_enabled);

        assert_eq!(
            response[1].name,
            Some("Samsung Electric Company LF24T35 HX5T502871".to_string())
        );
        assert_eq!(response[1].output_name, "HDMI-A-1".to_string());
        assert_eq!(response[1].current_mode.width, 1920);
        assert_eq!(response[1].current_mode.height, 1080);
        assert_eq!(response[1].current_mode.refresh_rate, 60.0);
        assert_eq!(response[1].available_modes.len(), 2);
        assert_eq!(response[1].available_modes[0].width, 1920);
        assert_eq!(response[1].available_modes[0].height, 1080);
        assert_eq!(response[1].available_modes[0].refresh_rate, 60.0);
        assert!(!response[1].available_modes[0].is_current);
        assert!(response[1].available_modes[0].is_preferred);
        assert_eq!(response[1].available_modes[1].width, 1920);
        assert_eq!(response[1].available_modes[1].height, 1080);
        assert_eq!(response[1].available_modes[1].refresh_rate, 60.0);
        assert!(response[1].available_modes[1].is_current);
        assert!(!response[1].available_modes[1].is_preferred);
        assert_eq!(response[1].physical_size.width_mm, 530);
        assert_eq!(response[1].physical_size.height_mm, 300);
        assert_eq!(response[1].logical_position.x, 0);
        assert_eq!(response[1].logical_position.y, 0);
        assert_eq!(response[1].logical_size.width, 1920);
        assert_eq!(response[1].logical_size.height, 1080);
        assert_eq!(response[1].current_scale, 1.0);
        assert!(response[1].variable_refresh_rate.is_supported);
        assert!(!response[1].variable_refresh_rate.is_enabled);
    }
}
