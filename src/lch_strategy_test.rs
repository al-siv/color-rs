#[cfg(test)]
mod lch_strategy_test {
    use crate::color_distance_strategies::{
        DistanceAlgorithm, calculate_distance,
    };
    use palette::Lab;

    #[test]
    fn test_lch_vs_delta_e_2000_comparison() {
        let start = Lab::new(51.49, -2.846, -33.140);

        let test_colors = vec![
            ("Stop2", Lab::new(51.95, 2.262, -26.010)), // lch(51.95, 26.109, -85.0)
            ("Stop3", Lab::new(52.52, 8.771, -16.925)), // lch(52.52, 19.063, -62.6)
            ("Stop4", Lab::new(53.66, 21.561, 0.926)),  // lch(53.66, 21.581, 2.5)
            ("End", Lab::new(57.74, 67.595, 65.177)),   // lch(57.74, 93.900, 44.0)
        ];

        println!("\n=== Delta E 2000 vs LCH Algorithm Comparison ===");
        println!("Start: lab({:.2}, {:.3}, {:.3})", start.l, start.a, start.b);

        for (name, color) in &test_colors {
            let distance_de2000 = calculate_distance(DistanceAlgorithm::DeltaE2000, start, *color);
            let distance_lch = calculate_distance(DistanceAlgorithm::Lch, start, *color);

            // Show LCH coordinates for context
            let c = (color.a * color.a + color.b * color.b).sqrt() as f32;
            let h = color.b.atan2(color.a).to_degrees() as f32;

            println!(
                "\n{}: lab({:.2}, {:.3}, {:.3}) -> lch({:.2}, {:.3}, {:.1})",
                name, color.l, color.a, color.b, color.l, c, h
            );
            println!("  Delta E 2000: {:.6}", distance_de2000);
            println!("  LCH Strategy: {:.6}", distance_lch);
            println!(
                "  Ratio (LCH/DE2000): {:.3}",
                distance_lch / distance_de2000
            );
        }

        // Calculate step differences for LCH algorithm
        println!("\n=== LCH Algorithm Step Differences ===");
        let mut lch_distances: Vec<f64> = vec![0.0]; // Start distance is always 0

        for (_, color) in &test_colors {
            let distance = calculate_distance(DistanceAlgorithm::Lch, start, *color);
            lch_distances.push(distance);
        }

        for i in 1..lch_distances.len() {
            let step_diff = lch_distances[i] - lch_distances[i - 1];
            println!("Step {}: {:.6}", i, step_diff);
        }

        // Check if LCH distances would be more uniform
        let total_lch_distance = lch_distances.last().unwrap();
        let expected_lch_step = total_lch_distance / 4.0;
        println!("\nExpected LCH step size: {:.6}", expected_lch_step);

        println!("\n=== LCH Step Uniformity Analysis ===");
        for i in 1..lch_distances.len() {
            let step_diff = lch_distances[i] - lch_distances[i - 1];
            let deviation = (step_diff - expected_lch_step).abs();
            let deviation_percent = (deviation / expected_lch_step) * 100.0;
            println!(
                "Step {}: {:.6} (deviation: {:.2}%)",
                i, step_diff, deviation_percent
            );
        }
    }
}
