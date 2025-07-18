use color_rs::color_formatter::ColorFormatter;
use palette::Lab;

fn main() {
    // Create a test Lab color (orange-ish)
    let lab_color = Lab::new(60.0, 30.0, 50.0);
    
    // Use the new function that demonstrates OutputUtils integration
    match ColorFormatter::format_color_with_output_utils(lab_color, "Orange Color") {
        Ok(output) => {
            println!("=== OutputUtils Integration Demo ===");
            println!("{}", output);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
