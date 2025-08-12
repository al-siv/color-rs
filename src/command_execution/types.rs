//! Command execution types and core data structures
//!
//! This module defines the fundamental types for functional command processing,
//! replacing traditional command pattern with enum-based dispatch.

use crate::cli::GradientArgs;
use crate::logger::{Logger, DEFAULT_LOGGER};
use std::collections::HashMap;

/// Command type using enum dispatch (replaces trait objects)
///
/// NOTE: This enum already serves as the planned "Command" ADT described in the
/// migration plan. No additional wrapping enum is required; sprint task for
/// introducing a Command enum can be satisfied by enhancing/maintaining this
/// existing `CommandType`. Future additions should extend variants here
/// (preserving exhaustive matching) rather than layering new indirection.
#[derive(Debug, Clone)]
pub enum CommandType {
    /// Generate color gradient between two colors
    GenerateGradient {
        args: GradientArgs,
        output_path: Option<String>,
    },
    /// Find closest matching colors in collections
    FindClosestColor {
        color_input: String,
        collection: Option<String>,
        algorithm: String,
        count: usize,
    },
    /// Analyze color properties and conversion
    AnalyzeColor {
        color_input: String,
        include_schemes: bool,
        output_format: String,
    },
    /// Convert color between different formats
    ConvertColor {
        color_input: String,
        target_format: String,
        precision: usize,
    },
}

/// Pre-execution hook step using functional composition
#[derive(Debug, Clone)]
pub enum PreHookStep {
    /// Validate command parameters
    ValidateParameters,
    /// Log command start
    LogStart,
    /// Check prerequisites (files, permissions, etc.)
    CheckPrerequisites,
    /// Custom validation function
    Custom(fn(&CommandType) -> crate::error::Result<()>),
}

/// Post-execution hook step for result processing
#[derive(Debug, Clone)]
pub enum PostHookStep {
    /// Format command output
    FormatOutput,
    /// Log execution completion
    LogCompletion,
    /// Save output to file
    SaveOutput,
    /// Custom processing function
    Custom(fn(&ExecutionResult) -> ExecutionResult),
}

/// Command execution context with functional composition
#[derive(Clone)]
pub struct ExecutionContext {
    /// Command type determines execution strategy
    pub command_type: CommandType,
    /// Pre-execution hooks (validation, logging, etc.)
    pub pre_hooks: Vec<PreHookStep>,
    /// Post-execution hooks (cleanup, formatting, etc.)
    pub post_hooks: Vec<PostHookStep>,
    /// Execution metadata
    pub metadata: HashMap<String, String>,
    /// Logger capability (effect boundary) - defaults to no-op
    pub logger: &'static dyn Logger,
}

/// Execution result with metadata and functional composition support
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Whether command executed successfully
    pub success: bool,
    /// Command output (stdout)
    pub output: String,
    /// Error message if execution failed
    pub error_message: Option<String>,
    /// Result metadata
    pub metadata: HashMap<String, String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u128,
}

impl std::fmt::Debug for ExecutionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExecutionContext")
            .field("command_type", &self.command_type)
            .field("pre_hooks", &self.pre_hooks)
            .field("post_hooks", &self.post_hooks)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl ExecutionContext {
    /// Create new execution context
    #[must_use]
    pub fn new(command_type: CommandType) -> Self {
        Self {
            command_type,
            pre_hooks: Vec::new(),
            post_hooks: Vec::new(),
            metadata: HashMap::new(),
            logger: &DEFAULT_LOGGER,
        }
    }

    /// Builder pattern for adding pre-hooks
    /// Builder pattern for adding pre-hooks
    #[must_use]
    pub fn with_pre_hook(mut self, hook: PreHookStep) -> Self {
        self.pre_hooks.push(hook);
        self
    }

    /// Builder pattern for adding post-hooks
    #[must_use]
    pub fn with_post_hook(mut self, hook: PostHookStep) -> Self {
        self.post_hooks.push(hook);
        self
    }

    /// Add metadata
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Attach a logger (explicit effect injection)
    #[must_use]
    pub fn with_logger(mut self, logger: &'static dyn Logger) -> Self {
        self.logger = logger;
        self
    }
}

impl ExecutionResult {
    /// Create successful result
    #[must_use]
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            error_message: None,
            metadata: HashMap::new(),
            execution_time_ms: 0,
        }
    }

    /// Create successful result with metadata
    #[must_use]
    pub const fn success_with_metadata(output: String, metadata: HashMap<String, String>) -> Self {
        Self {
            success: true,
            output,
            error_message: None,
            metadata,
            execution_time_ms: 0,
        }
    }

    /// Create failure result
    #[must_use]
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            output: String::new(),
            error_message: Some(error),
            metadata: HashMap::new(),
            execution_time_ms: 0,
        }
    }

    /// Add execution time
    #[must_use]
    pub const fn with_execution_time(mut self, time_ms: u128) -> Self {
        self.execution_time_ms = time_ms;
        self
    }
}

/// Available command types - compile-time constant
pub const AVAILABLE_COMMAND_TYPES: &[&str] = &[
    "generate_gradient",
    "find_closest_color",
    "analyze_color",
    "convert_color",
];
