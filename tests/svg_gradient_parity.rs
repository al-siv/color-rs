//! Parity test for create_svg_content to lock current SVG structure before refactor.
use color_rs::cli::GradientArgs;
use color_rs::color_parser::ColorParser;
use color_rs::image_core::create_svg_content;

// Simple stable hash (FNV-1a 64) to avoid asserting entire large string inline.
fn fnv1a64(data: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325; // offset basis
    for b in data.as_bytes() {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn test_args() -> GradientArgs {
    GradientArgs {
        start_color: "FF0000".to_string(),
        end_color: "00FFFF".to_string(),
        start_position: 0,
        end_position: 100,
        ease_in: 0.2,
        ease_out: 0.8,
        svg: None,
        png: None,
        no_legend: false,
        width: 420,
        step: None,
        stops: 6,
        stops_simple: false,
        output_format: None,
        output_file: None,
        func_filter: None,
        vectorized_text: false,
    }
}

#[test]
fn svg_gradient_parity_snapshot() {
    let parser = ColorParser::new();
    let args = test_args();
    let (start_lab, _) = parser.parse(&args.start_color).expect("parse start");
    let (end_lab, _) = parser.parse(&args.end_color).expect("parse end");
    let svg = create_svg_content(&args, start_lab, end_lab).expect("svg generation");

    // Structural assertions (cheap guards)
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("linearGradient"));
    assert!(svg.contains("<rect x=\"0\" y=\"0\""));
    assert!(svg.ends_with("</svg>"));

    // Hash-based snapshot (update only on intentional structural change)
    let hash = fnv1a64(&svg);
    // If this fails after refactor, verify diff & update expected.
    // Updated expected hash 2025-08-15 after intentional refactors affecting SVG ordering/content.
    // Previous: 0x9a085fb59c164f2c; New: 0x84250c0c7a74cc2c.
    let expected_hash: u64 = 0x84250c0c7a74cc2c; // snapshot hash (update only after auditing diff)
    assert_eq!(hash, expected_hash, "SVG gradient parity hash changed: {hash:016x}");
}
