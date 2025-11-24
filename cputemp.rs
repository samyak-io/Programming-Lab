use sysinfo::Components;

fn get_cpu_temp_celsius() -> Option<f32> {
    // Components::new_with_refreshed_list() scans hardware sensors (like CPU, GPU, etc.)
    let components = Components::new_with_refreshed_list();

    let mut fallback: Option<f32> = None;

    for component in &components {
        // temperature() returns Option<f32> according to latest docs
        if let Some(temp) = component.temperature() {
            let label = component.label().to_lowercase();

            // Prefer components whose label looks like "CPU", "Package", etc.
            if label.contains("cpu") || label.contains("package") {
                return Some(temp);
            }

            // If we haven't chosen anything yet, remember this as a fallback
            if fallback.is_none() {
                fallback = Some(temp);
            }
        }
    }

    fallback
}
