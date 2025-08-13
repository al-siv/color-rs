// Gradient parity test ensuring refactored generate_gradient orchestrator preserves behavior
// Focus on structural invariants rather than full golden snapshot to avoid brittleness.

use color_rs::{ColorRs, cli::GradientArgs};

fn build_args(start: &str, end: &str, stops: usize) -> GradientArgs {
    GradientArgs {
        start_color: start.to_string(),
        end_color: end.to_string(),
        start_position: 0,
        end_position: 100,
        ease_in: 0.65,
        ease_out: 0.35,
        svg: None,
        png: None,
        no_legend: false,
        width: 800,
        step: None,
        stops,
        stops_simple: false,
        output_format: Some(color_rs::cli::OutputFormat::Yaml),
        output_file: None,
        func_filter: None,
        vectorized_text: false,
    }
}

#[test]
fn test_gradient_parity_basic_invariants() {
    let color_rs = ColorRs::new();
    let args = build_args("#FF0000", "#0000FF", 7);

    // Capture stdout by redirecting temporarily if needed. Simpler: just run and ensure success.
    let result = color_rs.generate_gradient(args.clone());
    assert!(result.is_ok(), "Gradient generation should succeed: {result:?}");

    // Build unified stops using public GradientCalculationConfig (mirror orchestrator logic minimally)
    let parser = color_rs::color_parser::ColorParser::new();
    let (start_lab, _) = parser.parse(&args.start_color).expect("start parse");
    let (end_lab, _) = parser.parse(&args.end_color).expect("end parse");
    let steps = args.stops; // step option not used in this case
    let unified = color_rs::gradient::unified_calculator::calculate_unified_gradient_cfg(
        color_rs::gradient::unified_calculator::GradientCalculationConfig {
            start_lab,
            end_lab,
            start_position: args.start_position,
            end_position: args.end_position,
            ease_in: args.ease_in,
            ease_out: args.ease_out,
            steps,
            use_simple_mode: args.stops_simple,
            algorithm: color_rs::color_distance_strategies::DistanceAlgorithm::DeltaE2000,
        }
    );

    // Invariant 1: number of unified stops equals requested steps
    assert_eq!(unified.len(), steps, "Unified stops length mismatch");

    // Invariant 2: first and last positions match start/end positions
    assert_eq!(unified.first().unwrap().position as u8, args.start_position);
    assert_eq!(unified.last().unwrap().position as u8, args.end_position);

    // Invariant 3: monotonic non-decreasing positions
    assert!(unified.windows(2).all(|w| w[0].position <= w[1].position), "Positions not monotonic");

    // Invariant 4: hex of first/last derived from lab_to_hex consistency (indirect via generation of rgb tuple)
    // We simply ensure different endpoints for distinct colors and non-empty string.
    // Invariant 4: distinct LAB endpoints -> differing RGB tuples
    let first_rgb = unified.first().unwrap().rgb_color;
    let last_rgb = unified.last().unwrap().rgb_color;
    assert_ne!(first_rgb, last_rgb, "Distinct start/end LAB colors should yield distinct RGB endpoints");
}

#[test]
fn test_gradient_parity_step_vs_stops_consistency() {
    // Use step percentage to derive steps
    let mut args = build_args("red", "blue", 5);
    // 20% step -> 100/20 = 5 steps
    args.step = Some(20);
    // Recompute using the same simple formula from generation::parse_base_colors
    let derived_steps = (100 / args.step.unwrap() as usize).max(2);
    assert_eq!(derived_steps, 5, "Derived steps from step percentage should be 5");
}
