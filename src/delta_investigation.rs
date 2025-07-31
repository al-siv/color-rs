#[cfg(test)]
mod delta_e_investigation {
    use crate::color_distance_strategies::{DistanceAlgorithm, calculate_distance};
    use palette::{Lab, color_difference::ImprovedCiede2000};

    #[test]
    fn test_delta_e_2000_detailed() {
        // Start color: #457FB3
        let start = Lab::new(51.49, -2.846, -33.140);

        // Stop 2: #637DA8 -> lch(51.95, 26.109, -85.0)
        let stop2 = Lab::new(51.95, 2.262, -26.010);

        println!("=== Detailed Delta E 2000 Investigation ===");
        println!("Start: lab({:.3}, {:.3}, {:.3})", start.l, start.a, start.b);
        println!("Stop2: lab({:.3}, {:.3}, {:.3})", stop2.l, stop2.a, stop2.b);

        // Test direct palette function
        let delta_palette = start.improved_difference(stop2);
        println!("Direct palette improved_difference: {:.6}", delta_palette);

        // Test our functional API
        let delta_strategy = calculate_distance(DistanceAlgorithm::DeltaE2000, start, stop2);
        println!("Our functional calculate_distance: {:.6}", delta_strategy);

        // Manual calculation verification
        let dl = stop2.l - start.l;
        let da = stop2.a - start.a;
        let db = stop2.b - start.b;
        println!("\nDifferences:");
        println!("ΔL: {:.3}", dl);
        println!("Δa: {:.3}", da);
        println!("Δb: {:.3}", db);

        // Simple euclidean for comparison
        let euclidean = (dl * dl + da * da + db * db).sqrt() as f32;
        println!("Euclidean distance: {:.6}", euclidean);

        // Convert to LCH to see differences
        let start_c = (start.a * start.a + start.b * start.b).sqrt() as f32;
        let start_h = start.b.atan2(start.a).to_degrees() as f32;
        let stop2_c = (stop2.a * stop2.a + stop2.b * stop2.b).sqrt() as f32;
        let stop2_h = stop2.b.atan2(stop2.a).to_degrees() as f32;

        println!("\nLCH comparison:");
        println!(
            "Start LCH: ({:.2}, {:.3}, {:.1})",
            start.l, start_c, start_h
        );
        println!(
            "Stop2 LCH: ({:.2}, {:.3}, {:.1})",
            stop2.l, stop2_c, stop2_h
        );
        println!(
            "ΔL: {:.3}, ΔC: {:.3}, ΔH: {:.1}",
            stop2.l - start.l,
            stop2_c - start_c,
            stop2_h - start_h
        );
    }

    #[test]
    fn test_multiple_stops_investigation() {
        let start = Lab::new(51.49, -2.846, -33.140);

        let test_colors = vec![
            ("Stop2", Lab::new(51.95, 2.262, -26.010)), // Expected: 5.408635
            ("Stop3", Lab::new(52.52, 8.771, -16.925)), // Expected: 10.832061
            ("Stop4", Lab::new(53.66, 21.561, 0.926)),  // Expected: 16.24578
            ("End", Lab::new(57.74, 67.595, 65.177)),   // Expected: 21.672974
        ];

        println!("\n=== Multiple Stops Delta E Investigation ===");
        for (name, color) in test_colors {
            let distance = calculate_distance(DistanceAlgorithm::DeltaE2000, start, color);
            println!("{}: {:.6}", name, distance);

            // Show LCH coordinates for context
            let c = (color.a * color.a + color.b * color.b).sqrt() as f32;
            let h = color.b.atan2(color.a).to_degrees() as f32;
            println!("  LCH: ({:.2}, {:.3}, {:.1})", color.l, c, h);
        }
    }
}
