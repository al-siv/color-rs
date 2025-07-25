#[cfg(test)]
mod lch_gradient_test {
    use palette::Lab;
    use crate::color_distance_strategies::{ColorDistanceStrategy, DeltaE2000Strategy, LchStrategy};
    use crate::gradient::calculator::GradientCalculator;
    use crate::color_utils::LegacyColorUtils as ColorUtils;

    #[test]
    fn test_lch_gradient_generation() {
        // Start color: #457FB3
        let start_lab = Lab::new(51.49, -2.846, -33.140);
        // End color: #FF4612  
        let end_lab = Lab::new(57.74, 67.595, 65.177);
        
        let delta_e_strategy = DeltaE2000Strategy;
        let lch_strategy = LchStrategy;
        
        // Generate gradients with both strategies
        let delta_e_stops = GradientCalculator::calculate_unified_gradient_with_strategy(
            start_lab, end_lab, 20, 80, 0.65, 0.35, 5, false, &delta_e_strategy
        );
        
        let lch_stops = GradientCalculator::calculate_unified_gradient_with_strategy(
            start_lab, end_lab, 20, 80, 0.65, 0.35, 5, false, &lch_strategy
        );
        
        println!("\n=== Gradient Comparison: Delta E 2000 vs LCH Strategy ===");
        
        println!("\n--- Delta E 2000 Gradient ---");
        for (i, stop) in delta_e_stops.iter().enumerate() {
            let hex = ColorUtils::lab_to_hex(stop.lab_color);
            let distance = delta_e_strategy.calculate_distance(start_lab, stop.lab_color);
            
            // Calculate LCH coordinates for display
            let c = (stop.lab_color.a * stop.lab_color.a + stop.lab_color.b * stop.lab_color.b).sqrt();
            let h = stop.lab_color.b.atan2(stop.lab_color.a).to_degrees();
            
            println!("Stop {}: pos={}%, hex={}, lab=({:.2}, {:.3}, {:.3}), lch=({:.2}, {:.3}, {:.1}), distance={:.6}",
                     i, stop.position, hex, stop.lab_color.l, stop.lab_color.a, stop.lab_color.b,
                     stop.lab_color.l, c, h, distance);
        }
        
        println!("\n--- LCH Strategy Gradient ---");
        for (i, stop) in lch_stops.iter().enumerate() {
            let hex = ColorUtils::lab_to_hex(stop.lab_color);
            let distance = lch_strategy.calculate_distance(start_lab, stop.lab_color);
            
            // Calculate LCH coordinates for display
            let c = (stop.lab_color.a * stop.lab_color.a + stop.lab_color.b * stop.lab_color.b).sqrt();
            let h = stop.lab_color.b.atan2(stop.lab_color.a).to_degrees();
            
            println!("Stop {}: pos={}%, hex={}, lab=({:.2}, {:.3}, {:.3}), lch=({:.2}, {:.3}, {:.1}), distance={:.6}",
                     i, stop.position, hex, stop.lab_color.l, stop.lab_color.a, stop.lab_color.b,
                     stop.lab_color.l, c, h, distance);
        }
        
        // Compare step uniformity
        println!("\n=== Step Uniformity Comparison ===");
        
        let mut delta_e_distances = vec![0.0];
        let mut lch_distances = vec![0.0];
        
        for stop in &delta_e_stops[1..] {
            delta_e_distances.push(delta_e_strategy.calculate_distance(start_lab, stop.lab_color));
        }
        
        for stop in &lch_stops[1..] {
            lch_distances.push(lch_strategy.calculate_distance(start_lab, stop.lab_color));
        }
        
        println!("\nDelta E 2000 steps:");
        for i in 1..delta_e_distances.len() {
            let step = delta_e_distances[i] - delta_e_distances[i-1];
            println!("  Step {}: {:.6}", i, step);
        }
        
        println!("\nLCH Strategy steps:");
        for i in 1..lch_distances.len() {
            let step = lch_distances[i] - lch_distances[i-1];
            println!("  Step {}: {:.6}", i, step);
        }
    }
}
