//! Color analysis functions
//!
//! Pure functions for analyzing color properties and characteristics.
//! This module has been decomposed into focused submodules:
//!
//! - `conversions` - Type conversion logic and serializable color representations
//! - `core` - Core analysis functions and main logic
//! - `formatting` - Result formatting and comparison functions
//!
//! All original functions and types are re-exported for backward compatibility.

// Re-export all public items from the decomposed submodules
pub use self::{
    conversions::*,
    core::*,
    formatting::*,
};

// Declare the submodules
pub mod conversions;
pub mod core; 
pub mod formatting;
