/// Format utilities for display

/// Format temperature based on unit preference
pub fn format_temperature(celsius: f64, unit: &str) -> String {
    if unit == "F" {
        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
        format!("{:.1}°F", fahrenheit)
    } else {
        format!("{:.1}°C", celsius)
    }
}

/// Format length based on unit preference
pub fn format_length(cm: f64, unit: &str) -> String {
    if unit == "in" {
        let inches = cm / 2.54;
        format!("{:.1} in", inches)
    } else {
        format!("{:.1} cm", cm)
    }
}

/// Format weight based on unit preference
pub fn format_weight(kg: f64, unit: &str) -> String {
    if unit == "lb" {
        let lb = kg * 2.20462;
        format!("{:.2} lb", lb)
    } else {
        format!("{:.2} kg", kg)
    }
}

/// Format pressure based on unit preference
pub fn format_pressure(hpa: f64, unit: &str) -> String {
    match unit {
        "mmHg" => {
            let mmhg = hpa * 0.75006;
            format!("{:.0} mmHg", mmhg)
        }
        "inHg" => {
            let inhg = hpa * 0.02953;
            format!("{:.2} inHg", inhg)
        }
        _ => format!("{:.0} hPa", hpa),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_temperature_celsius() {
        assert_eq!(format_temperature(20.0, "C"), "20.0°C");
    }

    #[test]
    fn test_format_temperature_fahrenheit() {
        let f = format_temperature(0.0, "F");
        assert!(f.starts_with("32.0"));
    }

    #[test]
    fn test_format_length_cm() {
        assert_eq!(format_length(10.0, "cm"), "10.0 cm");
    }

    #[test]
    fn test_format_length_inches() {
        let inches = format_length(25.4, "in");
        assert!(inches.starts_with("10."));
    }

    #[test]
    fn test_format_weight_kg() {
        assert_eq!(format_weight(1.0, "kg"), "1.00 kg");
    }
}
