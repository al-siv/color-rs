#[cfg(test)]
mod distance_debug_tests {
    use palette::Lab;
    use crate::color_distance_strategies::{ColorDistanceStrategy, DeltaE2000Strategy};

    #[test]
    fn test_gradient_distance_verification() {
        let strategy = DeltaE2000Strategy;
        
        // Start color: #457FB3
        let start = Lab::new(51.49, -2.846, -33.140);
        
        // From gradient output:
        // Stop 2: lch(51.95, 26.109, -85.0) -> distance: 5.408635
        let stop2 = Lab::new(51.95, 2.262, -26.010); // #637DA8
        
        // Stop 3: lch(52.52, 19.063, -62.6) -> distance: 10.832061  
        let stop3 = Lab::new(52.52, 8.771, -16.925); // #7E7A9A
        
        // Stop 4: lch(53.66, 21.581, 2.5) -> distance: 16.24578
        let stop4 = Lab::new(53.66, 21.561, 0.926); // #A4737F
        
        // End color: #FF4612
        let end = Lab::new(57.74, 67.595, 65.177);
        
        println!("=== Delta E 2000 Distance Verification ===");
        println!("Start color: lab({:.2}, {:.3}, {:.3})", start.l, start.a, start.b);
        
        let d2 = strategy.calculate_distance(start, stop2);
        println!("Start -> Stop2: {:.6} (expected: 5.408635)", d2);
        
        let d3 = strategy.calculate_distance(start, stop3);
        println!("Start -> Stop3: {:.6} (expected: 10.832061)", d3);
        
        let d4 = strategy.calculate_distance(start, stop4);
        println!("Start -> Stop4: {:.6} (expected: 16.24578)", d4);
        
        let d_end = strategy.calculate_distance(start, end);
        println!("Start -> End:   {:.6} (expected: 21.672974)", d_end);
        
        println!("\n=== Step Differences ===");
        println!("Step 1: {:.6}", d2);
        println!("Step 2: {:.6}", d3 - d2);
        println!("Step 3: {:.6}", d4 - d3);
        println!("Step 4: {:.6}", d_end - d4);
        
        // Check if differences are roughly equal (smart algorithm should produce ~5.418 each)
        let expected_step = d_end / 4.0;
        println!("\nExpected step size: {:.6}", expected_step);
        
        assert!((d2 - expected_step).abs() < 0.1, "Step 1 should be ~{}", expected_step);
        assert!(((d3 - d2) - expected_step).abs() < 0.1, "Step 2 should be ~{}", expected_step);
        assert!(((d4 - d3) - expected_step).abs() < 0.1, "Step 3 should be ~{}", expected_step);
        assert!(((d_end - d4) - expected_step).abs() < 0.1, "Step 4 should be ~{}", expected_step);
    }
}
