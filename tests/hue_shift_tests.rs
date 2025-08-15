//! Tests for minimal signed hue shift computation ensuring wrap-around correctness.
use color_rs::command_execution::hue_analysis::compute_hue_shift;

#[test]
fn hue_shift_none_for_first() {
    assert_eq!(compute_hue_shift(None, 10.0), None);
}

#[test]
fn hue_shift_simple_forward() {
    assert_eq!(compute_hue_shift(Some(10.0), 30.0), Some(20.0));
}

#[test]
fn hue_shift_wrap_forward_across_zero() {
    // 350 -> 10 should yield +20, not -340
    assert_eq!(compute_hue_shift(Some(350.0), 10.0), Some(20.0));
}

#[test]
fn hue_shift_wrap_backward_across_zero() {
    // 10 -> 350 should yield -20, not +340
    assert_eq!(compute_hue_shift(Some(10.0), 350.0), Some(-20.0));
}

#[test]
fn hue_shift_boundary_positive_small() {
    // 179 -> 181 diff should be +2 not -358
    assert_eq!(compute_hue_shift(Some(179.0), 181.0), Some(2.0));
}

#[test]
fn hue_shift_boundary_negative_small() {
    // 181 -> 179 diff should be -2 not +358
    assert_eq!(compute_hue_shift(Some(181.0), 179.0), Some(-2.0));
}

#[test]
fn hue_shift_exact_opposite_positive() {
    // 0 -> 180 = +180 (allowed edge)
    assert_eq!(compute_hue_shift(Some(0.0), 180.0), Some(180.0));
}

#[test]
fn hue_shift_exact_opposite_negative() {
    // 180 -> 0 = -180 (allowed edge)
    assert_eq!(compute_hue_shift(Some(180.0), 0.0), Some(-180.0));
}

#[test]
fn hue_shift_identity() {
    assert_eq!(compute_hue_shift(Some(42.0), 42.0), Some(0.0));
}

#[test]
fn hue_shift_magnitude_never_exceeds_180() {
    // Random sampling of pairs that would exceed 180 raw diff
    let samples = vec![(0.0, 300.0), (300.0, 0.0), (45.0, 225.0), (225.0, 45.0)];
    for (a,b) in samples { 
        if let Some(shift) = compute_hue_shift(Some(a), b) { 
            assert!(shift.abs() <= 180.0, "shift {shift} exceeds 180 for pair ({a},{b})"); 
        }
    }
}
