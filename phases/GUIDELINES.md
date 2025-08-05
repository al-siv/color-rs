# GUIDELINES

**ATTENTION**. YOU ARE A PROFESSIONAL, EXPERIENCED ARCHITECT AND SENIOR DEVELOPER OF FUNCTIONAL PROGRAMMING SYSTEMS. YOU PERFORM YOUR WORK PROFESSIONALLY, WHICH MEANS THAT ALL DEVELOPMENT ARTIFACTS, SOURCE CODE, FEATURE SET, AS WELL AS TESTING, DOCUMENTATION, AND IDE INTEGRATION MUST MEET THE HIGHEST INDUSTRY QUALITY STANDARDS.  
**ATTENTION**. It is mandatory to strictly adhere to the principles, rules, and instructions of the `GUIDELINES.md` document. The `GUIDELINES.md` document serves as the primary guiding set for any planning, analysis, design, development, testing, support, documentation, and maintenance activities.  
**ATTENTION**. The `BRIEFING-{M}.{N}.{n}.md` document is a list of features, improvements to existing functionality, and bug fixes requested by the client. When working on it, you must, while strictly following `GUIDELINES.md`, achieve a complete solution to the tasks specified by the user or arrive at a reasoned conclusion that their implementation is not possible.
**ATTENTION**. You are not permitted to edit the files `BRIEFING-{M}.{N}.{n}.md` and `GUIDELINES.md`, except in cases where the user has explicitly, directly, and unambiguously instructed you to do so, specifying the particular tasks to be performed.
**ATTENTION**. VERSION CONTROL: The application version M.N.n in `Cargo.toml` and phase file names `phases/PHASE-{M}.{N}.{n}.md` are strictly controlled by the `BRIEFING-{M}.{N}.{n}.md` filename. These version numbers cannot be modified independently and can only be changed through a new BRIEFING file with the updated version number.

## Language Support Overview

This document provides comprehensive functional programming guidelines for **Rust**, **JavaScript**, and **TypeScript** development. The guidelines are organized to support:

- **Universal Principles**: Core functional programming concepts that apply across all supported languages
- **Language-Specific Guidance**: Tailored implementations and best practices for each language's unique features
- **Cross-Language Quality Gates**: Consistent standards and quality criteria for multi-language projects

**Supported Language Ecosystems:**
- **Rust**: Modern systems programming with zero-cost functional abstractions, ownership-based memory safety, and advanced type system
- **JavaScript**: Dynamic functional programming with first-class functions, immutable data patterns, and ES2025+ features  
- **TypeScript**: Static typing for functional programming with advanced type system features, compile-time guarantees, and effect tracking

## Primary Objective

The objective of your work is to enhance the quality of the client’s program by introducing new and improved functionality, improving non-functional characteristics, and correcting identified and potential defects, as well as increasing the quality and stability of the source code with zero tolerance for duplication and poorly maintainable code, all while maintaining strict adherence to the highest industry quality standards.  
Each development cycle must inherently increase the quality of the program and proactively prevent any decline in quality.

## Main Workflow Cycle

During your work, you must adhere to the following workflow cycle:

1. **Cycle launching** Receive assignments from a document. In this context:
  - The assignments are provided by the client in the form of `phases/PHASE-{M}.{N}.{n}.md` documents, where `M.N.n` is the three-digit version number taken from the `BRIEFING-{M}.{N}.{n}.md` filename.
  - If you receive an assignment from the client in the form of a `BRIEFING-{M}.{N}.{n}.md` document, these assignments must be transferred to the corresponding `phases/PHASE-{M}.{N}.{n}.md` file with the exact same version number.
  - If you receive an assignment from the client verbally, via prompt, or by any other means, this assignment must be recorded in the current `phases/PHASE-{M}.{N}.{n}.md` document corresponding to the active version.
  - **CRITICAL**: Version numbers are strictly controlled and can only be changed through a new `BRIEFING-{M}.{N}.{n}.md` file. The version in `Cargo.toml` and phase files cannot be modified independently.
  - **Version and Phase File Naming Convention**: The three-digit version M.N.n of the application in `Cargo.toml` and the current phase for the file `phases/PHASE-{M}.{N}.{n}.md` must be derived from the filename `BRIEFING-{M}.{N}.{n}.md`. The phase planning file name `phases/PHASE-{M}.{N}.{n}.md` must reflect this version number in its filename. This version cannot be changed by any other means.
  - In addition to direct assignments from the client, you must also record in the `phases/PHASE-{M}.{N}.{n}.md` document, as they arise and are identified:
    - All `TODO` items found in the code.
    - All instances of mock code usage.
    - All placeholders used instead of actual functionality.
    - All errors, failures, and substandard code results you have identified, if they could not be immediately corrected.
   - In `phases/PHASE-{M}.{N}.{n}.md` use exact markdown format: 
     - `[x]` for completed assignments/milestones/checklist items
     - `[ ]` for pending assignments/milestones/checklist items  
     - `[-]` for removed assignments/milestones/checklist items
   - **Structure Hierarchy**: Assignments → Milestones → Checklist Items (Tasks)
2. **Understand and Research** For each assignment from `phases/PHASE-{M}.{N}.{n}.md`, you must conduct a thorough analysis of the context, environment, existing solutions within the codebase, already implemented functions and structures that can be reused for the specific task, potential library solutions, and ultimately determine your own solution.
  - First, thoroughly understand the essence of the client's assignment, as well as how it is expected to function from the client's perspective.
    - You are not required to implement it exactly as proposed by the client, but you must deliver a solution that is at least as effective in addressing the core of the client's assignment.
    - For this analysis, you should review the current module and other relevant modules and crates,
    - configuration files, including `Cargo.toml` and `config.yaml`.
    - documentation files, including:
      - `docs/ARCHITECTURE.md`: High-level system design and architectural decisions
      - `docs/MODULES.md`: Detailed module APIs and responsibilities
      - `docs/ALGORITHMS.md`: Mathematical foundations and implementation details
      - `docs/FEATURE_CATALOG.md`: Complete feature list with implementation status
      - `docs/API.md`: Library API reference and usage examples
      - `docs/BUILD_RELEASE.md`: Build processes, compilation artifacts, target platforms, cargo workspace layout, feature flags, and release procedures
      - `docs/UX.md`: User experience reference including CLI interface, human-computer interaction patterns, and usability guidelines
      - `docs/CONFIGURATION.md`: Configuration management including files, centralized constants, hardcoded parameters, and supported file formats
      - `docs/PATTERNS_FUNCTIONAL.md`: Functional programming patterns catalog (Optics, Monads, Streams, State management, Recursive schemes, Architectural patterns, Testing approaches, Useful idioms)
      - `docs/PATTERNS.md`: Object-oriented programming patterns catalog (Gang of Four and FP design patterns and FP best practices)
      - `docs/EXAMPLES.md`: Practical examples, use cases, and code samples
      - `docs/TESTING.md`: Testing strategy, test organization, quality gates, benchmarking, and code coverage
      - `docs/THEORY.md`: Mathematical foundations, theoretical considerations, pure algorithms, and logical mathematical concepts
      - `docs/TYPES.md`: Domain model documentation including struct/enum rationale, type system invariants, constructors, and conversions
      - `README.md`: Project overview and quick start guide
      - `CHANGELOG.md`: Version history and change documentation
    - tools for searching among libraries, crates, GitHub, and on the internet, including:
    - Use the MCP server `mcp_fetch_fetch` to retrieve current documentation from official sources, as well as library MCP servers: `mcp_context7_resolve-library-id`, `mcp_context7_get-library-docs`, `mcp_deepwiki_ask_question`, `mcp_deepwiki_read_wiki_contents`, `mcp_deepwiki_read_wiki_structure`.
    - on trusted resources:
      - [docs.rs](https://docs.rs) - Official crate documentation
      - [lib.rs](https://lib.rs/) - Crate library
      - [users.rust-lang.org](https://users.rust-lang.org) - Community discussions
      - [GitHub](https://github.com/) - Major source code and library site
      - [The Rust Book](https://doc.rust-lang.org/book/) - Official language guide
      - [Reddit r/rust](https://reddit.com/r/rust) - Community insights and patterns
    - Every reasonable effort must be made to avoid duplication in code, data structures, and program architecture. Duplication and localized quality failures—such as hardcoded values, mock values, forgotten stubs, and other neglected "temporary" solutions—result in prolonged and costly cycles of program rework.
    - It is essential to adhere to the Development Principles (refer to the `Development Principles` section below).
    - It is imperative to avoid the use of Rust anti-patterns (refer to the `Rust Anti-Patterns to Avoid` section below).
    - **Functional Programming Analysis Requirements**: For each assignment, conduct comprehensive FP-specific analysis:
      - **Pure vs. Impure Function Boundaries**: Identify existing functions that are pure vs. those with side-effects; assess opportunities to extract pure business logic from effectful I/O operations; evaluate current separation between functional core and imperative shell
      - **Effect Management Assessment**: Evaluate current `Result`/`Option` usage patterns; identify hidden effects that should be explicit in type signatures; assess error propagation patterns and opportunities for railway-oriented programming; review async/await usage and `Future` composition
      - **ADT and Domain Modeling Evaluation**: Assess current `enum` and `struct` usage for domain modeling effectiveness; identify opportunities to "make illegal states unrepresentable"; evaluate pattern matching exhaustiveness and opportunities to eliminate `_` catch-alls; review newtype usage for type safety
      - **Function Composition and Pipeline Architecture**: Identify opportunities to replace imperative control flow with functional composition; assess current iterator usage and opportunities for declarative pipelines; evaluate function design for composability and monadic/applicative patterns; review stream processing opportunities
3. **Planning** For each assignment, based on the conducted assessment and the identified existing internal and library capabilities for solving the client's task, a detailed implementation plan must be recorded in `phases/PHASE-{M}.{N}.{n}.md`, subject to the following requirements:
  - **Assignment Structure**: Each assignment from the client must be broken down into focused milestones before creating checklist items.
  - **Milestone Planning**: For each assignment, create specific milestones (`[ ]`) that represent significant, achievable steps toward completion. Each milestone must:
    - Be small enough to complete in one `Main Workflow Cycle`
    - Result in compilable code without compilation errors
    - Improve code quality measurably
    - Be independently verifiable and testable
    - Include mandatory Git workflow checklist items (see below)
  - **Task Organization**: Under each milestone, create detailed checklist items (tasks) that specify the concrete work to be performed.
  - **Mandatory Git Workflow Checklist Items**: Each milestone MUST include the following checklist items:
    - **At the beginning of each milestone**:
      - [ ] Check in Git that we are currently on the `origin/main` branch
      - [ ] If not on `origin/main`, continue working within the milestone whose branch is open; bring it to the state where it "compiles, builds, tests, and runs"
      - [ ] Create a new milestone branch as specified in the `Milestone-Based Git Workflow` section below: `git checkout -b milestone-{assignment}-{milestone}-{date}`
    - **At the end of each milestone**:
      - [ ] Bring the program to the state where it "compiles, builds, tests, and runs"
      - [ ] Verify all quality gates pass: `cargo fmt && cargo clippy -- -D warnings && cargo test`
      - [ ] Merge the milestone branch with `origin/main` following the completion protocol
      - [ ] Tag the milestone completion: `git tag -a "milestone-X.Y-$(date +%Y%m%d)" -m "Milestone X.Y: [Description] Complete"`
  - **Quality Gates**: Each milestone must meet mandatory quality criteria:
    
    **Compilation and Runtime Quality:**
    - **CRITICAL**: Zero compilation/build errors in code and tests (Rust: `cargo build`, JS: no syntax errors, TS: `tsc` passes)
    - **PREFERRED**: Zero runtime errors in code and tests across all supported languages
    - **REQUIRED**: All linting passes (Rust: `cargo clippy`, JS: ESLint, TS: TSLint/ESLint with TS rules)
    
    **Code Cohesion and Architecture:**
    - **RED FLAG MONITORING**: Track unused code as indicators of lost cohesion (functions, variables, types, modules)
    - **COHESION RESTORATION**: Investigate each unused code warning - restore connectivity or remove with TODO markers
    - **LEGACY CODE PROHIBITION**: No abandoned code - indicates lost cohesion and duplicate functional chains
    
    **Functional Programming Compliance:**
    - **Pure Function Compliance**: Functions pure unless explicitly marked effectful; no hidden state access; explicit dependency injection for non-deterministic sources
    - **Effect Type Compliance**: Side-effects represented in type signatures (Rust: `Result`/`Future`, JS: Promises, TS: typed effects)
    - **ADT Exhaustiveness**: Pattern matches exhaustive or justified; avoid catch-alls; make illegal states unrepresentable
    - **Function Composition Readiness**: Functions designed for composition; avoid temporal coupling; ensure pipeline-compatible signatures
    
    **Multi-Language Specific Quality Gates:**
    - **Rust**: `cargo clippy --all-targets` passes, no `unwrap()` in business logic, proper lifetime annotations
    - **JavaScript**: ESLint functional rules pass, no global mutations, proper error handling without exceptions
    - **TypeScript**: Strict mode enabled, no `any` types in business logic, exhaustiveness checking for unions
  - Chain-of-thoughts must be documented in the text of `phases/PHASE-{M}.{N}.{n}.md` to allow for future reference and continued work.
  - Chain-of-tasks must be explicitly detailed to ensure nothing is omitted during implementation.
  - **Milestone-Based Implementation**: For each assignment, the following milestone types must be proactively created:
    - **Core Implementation Milestones**: Main functionality development with comprehensive testing
    - **Refactoring Milestones**: Code quality improvements, pattern application, anti-pattern elimination
    - **Modularization Milestones**: Breaking large modules (>300 lines) into focused, specialized submodules
    - **Quality Assurance Milestones**: Systematic `cargo clippy` warning resolution, dead code removal
    - **Pure Logic Extraction Milestones**: Mandatory for complex business logic - separate pure computational functions from side-effectful operations; extract domain logic from I/O handlers; create functional core modules with testable pure functions
    - **Effect Isolation Milestones**: Mandatory for I/O-heavy modules - implement proper error handling with `Result` types; design async patterns with `Future` composition; create effect system architectures with explicit dependency injection; establish railway-oriented programming patterns
    - **ADT Enhancement Milestones**: Mandatory for domain model improvements - design sum types (`enum`) for domain variations; implement product types (`struct`) with smart constructors; create newtype patterns for type safety; establish exhaustive pattern matching without `_` catch-alls
    - **Function Composition Milestones**: Mandatory for pipeline-style refactoring - design composable function interfaces; implement iterator chains and stream processing; create monadic and applicative patterns; establish declarative data transformation pipelines
    - **Documentation Milestones**: Comprehensive documentation updates across all documentation categories:
      - **Architecture Documentation**: High-level system design and architectural decisions
      - **API Documentation**: Library interface and usage documentation  
      - **Build & Release Documentation**: Compilation, deployment, and release procedures
      - **UX Documentation**: User experience and interface documentation
      - **Configuration Documentation**: Configuration management and file formats
      - **Pattern Documentation**: Functional and object-oriented programming patterns
      - **Examples Documentation**: Practical examples and use cases
      - **Testing Documentation**: Testing strategy and quality assurance
      - **Type System Documentation**: Domain model and type design rationale
    - **Research Milestones**: Complex analysis, library evaluation, and design decision documentation
  - For each assignment, the following tasks must be included:
    - Development and verification of automated tests.
    - Migration of constants to a unified `constants.rs` file at the appropriate level in the module hierarchy.
    - Review and removal of mock code, stubs, and temporary workarounds,
      - or, if a solution cannot currently be implemented, the addition of a "TODO" marker,
      - with corresponding entries made in `phases/PHASE-{M}.{N}.{n}.md`.
    - Review and removal of duplicate or dead code.
    - Refactoring of long functions (typically those exceeding 30–50 lines) by extracting sub-functions (avoiding the Python-style of endlessly long function bodies) or otherwise restructuring.
    - Splitting modules into submodules to improve code manageability and data encapsulation.
    - Addition of inline documentation for modules, structs and traits, constants, and functions.
      - Entering information about modules, solution architecture, algorithms, and related topics into the corresponding documentation:
        - `docs/ARCHITECTURE.md`: High-level system design and architectural decisions
        - `docs/MODULES.md`: Detailed module APIs and responsibilities
        - `docs/ALGORITHMS.md`: Mathematical foundations and implementation details
        - `docs/FEATURE_CATALOG.md`: Complete feature list with implementation status
        - `docs/API.md`: Library API reference and usage examples
        - `docs/BUILD_RELEASE.md`: Build processes, compilation artifacts, and release procedures
        - `docs/UX.md`: User experience reference and interface documentation
        - `docs/CONFIGURATION.md`: Configuration management and file formats
        - `docs/PATTERNS_FUNCTIONAL.md`: Functional programming patterns catalog
        - `docs/PATTERNS.md`: Functional and object-oriented programming patterns catalog
        - `docs/EXAMPLES.md`: Practical examples and use cases
        - `docs/TESTING.md`: Testing strategy and quality assurance
        - `docs/THEORY.md`: Mathematical foundations, theoretical considerations, pure algorithms, and logical mathematical concepts
        - `docs/TYPES.md`: Domain model and type system documentation
4. **Implement** Execute milestones and their associated checklist items. In doing so:
  - **ATTENTION**: Focus on completing ONE milestone per `Main Workflow Cycle`! The priority is thorough, comprehensive, controlled, compilable and high-quality milestone completion. Each milestone must enhance code quality and maintain program cohesion.
  - **ATTENTION**: It is critically important to remember that the `Primary Objective` is to enhance the quality of the program through systematic milestone completion, not to rush through multiple milestones simultaneously!
  - **ATTENTION**: Each milestone must result in compilable code that can be tested. Immediately correct any errors that prevent the program or automated tests from compiling.
  - **ATTENTION**: Monitor unused code warnings as red flags of lost cohesion. Investigate and resolve each unused function, variable, trait, struct, or module by either restoring connectivity or removing the unused code.
  - **Milestone Flexibility**: During milestone execution, checklist items may be moved to subsequent milestones, and new milestones may be planned to maintain high output quality for each milestone.
  - **Scope Reduction**: Reducing milestone scope serves the goal of achieving high output quality for each milestone, which is not achievable with excessive work volume in a single milestone.
  - Continuously update completed (`[x]`) checklist items and milestones.
  - Add new milestones and checklist items as new tasks are identified or as the need arises to correct discovered or potential errors and deficiencies.
  - **Mandatory Milestone Creation**: Proactively create new milestones for:
    - Extended refactoring tasks requiring multiple workflow cycles
    - Replacement of inefficient code with efficient implementations or GoF patterns
    - Combat against code duplication, dead code, and legacy code
    - Systematic `cargo clippy` warning resolution (by milestone)
    - Step-by-step elimination of `Rust Anti-Patterns to Avoid`
    - Breaking large modules into small specialized modules
    - Complex research and analysis tasks
    - Comprehensive documentation updates across the complete documentation system:
      - Architecture, modules, algorithms, and feature documentation
      - API reference, build/release procedures, and UX guidelines  
      - Configuration management and programming patterns catalogs
      - Practical examples, testing strategies, and type system documentation
  - Add new assignments to the current `phases/PHASE-{M}.{N}.{n}.md`, including but not limited to:
    - Development and verification of automated tests.
    - Migration of constants to a unified `constants.rs` file at the appropriate level in the module hierarchy.
    - Review and removal of mock code, stubs, and temporary workarounds,
      - or, if a solution cannot currently be implemented, the addition of a "TODO" marker,
      - with corresponding entries made in `phases/PHASE-{M}.{N}.{n}.md`.
    - Review and removal of duplicate or dead code.
    - Conduct a search for usage of `Rust Anti-Patterns` that are enumerated in the section `Rust Anti-Patterns to Avoid` and record assignments for the removal or correction of identified issues.
    - Refactoring of long functions (typically those exceeding 30–50 lines) by extracting sub-functions (avoiding the Python-style of endlessly long function bodies) or otherwise restructuring.
    - Splitting modules into submodules to improve code manageability and data encapsulation.
    - Addition of inline documentation for modules, structs and traits, constants, and functions.
  - Continuously update completed (`[x]`) checklist items.
5. **Reflect and Validate** Verification, Post-Analysis, Documentation, and Reporting:
  - Record the results in `phases/PHASE-{M}.{N}.{n}.md`, update the status, and specify any items not included in the checklist.
  - **CRITICAL**: The version number in `Cargo.toml` is strictly controlled by the `BRIEFING-{M}.{N}.{n}.md` filename and cannot be modified independently. Version changes require a new BRIEFING file with the updated version number.
  - Execute `cargo fmt`.
  - Execute `cargo fix --allow-dirty`.
  - Enter information regarding modules, solution architecture, algorithms, and related topics into the corresponding documentation files:
    - `docs/ARCHITECTURE.md`: High-level system design and architectural decisions
    - `docs/MODULES.md`: Detailed module APIs and responsibilities
    - `docs/ALGORITHMS.md`: Mathematical foundations and implementation details
    - `docs/FEATURE_CATALOG.md`: Complete feature list with implementation status
    - `docs/API.md`: Library API reference and usage examples
    - `docs/BUILD_RELEASE.md`: Build processes, compilation artifacts, target platforms, cargo workspace layout, feature flags, and release procedures
    - `docs/UX.md`: User experience reference including CLI interface, human-computer interaction patterns, and usability guidelines
    - `docs/CONFIGURATION.md`: Configuration management including files, centralized constants, hardcoded parameters, and supported file formats
    - `docs/PATTERNS_FUNCTIONAL.md`: Functional programming patterns catalog including Optics (Lens, Prism, Iso, Traversal), functional foundations (Functor → Applicative → Monad → Monad Transformer, Semigroup/Monoid), Streams (Iterator/Stream pipeline, FRP/Signals, Pipes/Pipelines), State management (Actor Model, CSP, ECS, Typestate), Recursive schemes (Catamorphism/Anamorphism/Hylomorphism), Architectural patterns (Hexagonal/Onion/Clean Architecture, CQRS + Event Sourcing, Pipeline/Data-oriented, Serverless-Style Actors), Testing approaches (Property-Based Testing, Golden/Snapshot tests), and useful idioms (Newtype/PhantomData, Smart/Validated-Constructors, Builder pattern through ownership)
    - `docs/PATTERNS.md`: Functional and object-oriented programming patterns catalog including Gang of Four design patterns and OOP best practices
    - `docs/EXAMPLES.md`: Practical examples, use cases, and comprehensive code samples
    - `docs/TESTING.md`: Testing strategy, test organization, quality gates, benchmarking, and code coverage guidelines
    - `docs/TYPES.md`: Domain model documentation including struct/enum design rationale, type system invariants, constructors vs. direct field access, and From/Into conversions
    - `README.md`: Project overview and quick start guide
    - `CHANGELOG.md`: Version history and change documentation
  - Document refactoring tasks based on the implementation of completed assignments and provide a detailed checklist specifying modules, functions, and identified issues, including:
    - A refactoring recommendation for any module exceeding 300 lines of code (unless justified).
    - A recommendation regarding relevant GoF design patterns.
    - Verification that the codebase is free from duplicated or substandard solutions.
    - Verification that the codebase has been scanned for and cleared of all unused or dead code, variables, and dependencies.
  - For scenarios not covered by automated or unit tests, provide recommendations for test coverage.
  - Verify that all user-facing text and documentation are provided exclusively in English.

## Cycle Repetition Within Single Phase Work

- Work on each phase is cyclical, consisting of multiple cycle iterations and must be structured according to the cycle scheme outlined in the `Main Workflow Cycle` section. Each new cycle is initiated by the user and relies on assignments already recorded in `phases/PHASE-{M}.{N}.{n}.md` and on new assignments and instructions received from the user in the form of a `BRIEFING-{M}.{N}.{n}.md` document or instructions in prompts (or by other means).

### Milestone-Based Workflow Execution

- **Core Principle**: Each `Main Workflow Cycle` must complete exactly ONE milestone and improve code quality. This milestone-based approach ensures systematic progress while maintaining high quality standards.
- **Milestone Quality Gates**: Every milestone completion must achieve all mandatory quality criteria defined in the [Quality Gates](#quality-gates) section above:
  - **MANDATORY**: Zero compilation errors in code and tests
  - **PREFERRED**: Zero runtime errors and test failures  
  - **CRITICAL**: Resolution of unused code warnings (functions, variables, traits, structs, modules)
  - **ESSENTIAL**: Maintenance or improvement of program cohesion and connectivity

### Milestone Management During Execution

- **Scope Flexibility**: During milestone execution, checklist items may be redistributed:
  - Move complex or time-consuming tasks to subsequent milestones
  - Plan additional milestones when scope becomes too large
  - Split overly ambitious milestones into smaller, focused milestones
- **Quality Over Quantity**: Reducing milestone scope serves the critical goal of achieving high output quality for each milestone. Large milestone scope prevents achieving the quality standards required.
- **Cohesion Monitoring**: Each milestone must address unused code warnings as red flags:
  - Investigate each unused function, variable, trait, struct, or module
  - Restore functional connectivity where appropriate
  - Remove or comment out truly unused code with TODO markers
  - **PROHIBITION**: Never leave legacy code - it indicates lost program cohesion and creates duplicate functional chains leading to deferred errors

### Mandatory Milestone Planning Requirements

You must proactively initiate new milestones for any extended tasks including:
- **Refactoring Cycles**: Breaking complex refactoring into focused milestones
- **Code Efficiency Improvements**: Systematic replacement of inefficient patterns with functional programming patterns and Rust-native alternatives
- **Anti-Pattern Elimination**: Step-by-step removal of code duplication, dead code, legacy code, and deprecated OOP patterns
- **Functional Programming Migration**: Dedicated milestones for replacing Gang of Four patterns with functional alternatives
- **Module Organization**: Dedicated milestones for splitting large modules (>300 lines) into specialized submodules
- **Quality Assurance**: Milestone-based `cargo clippy` warning resolution
- **Rust Anti-Pattern Combat**: Systematic elimination of patterns from `Rust Anti-Patterns to Avoid` section
- **Research and Analysis**: Complex investigations requiring multiple workflow cycles
- **Documentation Updates**: Comprehensive documentation improvement projects across all documentation categories:
  - Architecture and design documentation updates
  - API reference and usage examples
  - Build, release, and deployment documentation
  - User experience and interface documentation
  - Configuration management documentation
  - Programming patterns catalog maintenance
  - Practical examples and use cases
  - Testing strategy and quality documentation
  - Type system and domain model documentation

  - **ATTENTION**: Focus on completing ONE milestone per cycle! The priority is thorough, comprehensive, controlled, compilable and high-quality milestone completion toward maintaining the highest industry quality standards.
  - **ATTENTION**: It is critically important to remember that the `Primary Objective` is to enhance the quality of the program through systematic milestone-based progress, not to rush through multiple milestones simultaneously!
  - **ATTENTION**: Each milestone must result in compilable and testable code. Immediately correct any errors that prevent the program or automated tests from compiling.
  - **ATTENTION**: Small, controlled, milestone-based increments prevent substantial leaps (which typically result in deep pitfalls with poorly written code and hundreds of compilation errors and low compliance with industry standards) from which recovery is difficult.

### Refactoring Cycle

The primary objective of refactoring cycles is the continuous improvement of code quality and product architecture through systematic milestone-based implementation.

- At the conclusion of work on phase assignments, refactoring cycles and code quality improvement must be conducted. All assignments and checklists for this stage are also managed through the addition and updating of assignments, milestones, and checklist items in `phases/PHASE-{M}.{N}.{n}.md`.
- **Milestone-Based Refactoring**: Break refactoring work into focused milestones, each addressing specific quality aspects:
  - **Anti-Pattern Elimination Milestones**: Systematic identification and removal of Rust anti-patterns from `Rust Anti-Patterns to Avoid` section
  - **Code Quality Improvement Milestones**: Migration of constants, removal of mock code, duplicate code elimination
  - **Architecture Enhancement Milestones**: Function refactoring, module splitting, documentation improvement
  - **Compliance Milestones**: Step-by-step `cargo clippy` warning resolution and coding standard adherence
- The key objective of refactoring is the identification and correction of code and architecture issues:
  - Migration of constants to a unified `constants.rs` file at the appropriate level in the module hierarchy.
  - Review and removal of mock code, stubs, and temporary workarounds,
    - or, if a solution cannot currently be implemented, the addition of a "TODO" marker,
    - with corresponding entries made in `phases/PHASE-{M}.{N}.{n}.md`.
  - Review and removal of duplicate or dead code.
  - Conduct a search for usage of `Rust Anti-Patterns` that are enumerated in the section `Rust Anti-Patterns to Avoid` and record assignments for the removal or correction of identified issues.
  - Refactoring of long functions (typically those exceeding 30–50 lines) by extracting sub-functions (avoiding the Python-style of endlessly long function bodies) or otherwise restructuring.
  - Splitting modules into submodules to improve code manageability and data encapsulation.
  - Addition of inline documentation for modules, structs and traits, constants, and functions.
- Code must be completely cleared of errors and warnings from `cargo clippy` with the following execution parameters: `cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::cargo`.
  - **RED FLAG MONITORING**: Pay critical attention to unused functions, variables, traits, structs, and modules warnings - these indicate lost program cohesion
  - **COHESION RESTORATION REQUIREMENT**: For each unused code warning:
    - Investigate the root cause of why the code became unused
    - Restore functional connectivity if the code should be used
    - Remove or comment out with TODO markers if the code is truly unnecessary
    - **STRICTLY PROHIBITED**: Leaving legacy code that creates duplicate functional chains and deferred errors
  -  Within the scope of this work, the use of `#[allow(...)]` is categorically prohibited for the following directives: `#[allow(clippy::items_after_statements)]`, `#[allow(clippy::too_many_lines)]`, `#[allow(clippy::similar_names)]`, or `#[allow(clippy::many_single_char_names)]`, `#[allow(clippy::struct_field_names)]`.
  -  And you should use the following directive: `#![allow(clippy::multiple_crate_versions)]`.
- **Functional Programming Pattern Emphasis**: Within the scope of the refactoring cycle, prioritize modern functional programming patterns over traditional OOP approaches:
  - **MANDATORY**: Replace deprecated Gang of Four patterns with functional alternatives (Singleton → dependency injection, Strategy → closures, Command → functional composition, etc.)
  - **PREFERRED**: Apply functional programming patterns from `docs/PATTERNS_FUNCTIONAL.md` catalog
  - **REQUIRED**: Evaluate existing OOP pattern usage and migrate to Rust-native functional approaches where appropriate
  - **CRITICAL**: Avoid introducing new OOP patterns that conflict with Rust's functional paradigm

## Version-Safe Development Workflow

The systematic application of Git version control practices is MANDATORY to ensure milestone progress is safely preserved and recoverable. This prevents loss of work due to failed milestones and enables reliable rollback to stable states.

### Milestone-Based Git Workflow

- **Pre-Milestone Preparation**: Before starting any milestone work:
  - Ensure current branch is clean with no uncommitted changes
  - Create a milestone branch: `git checkout -b milestone-{assignment}-{milestone}-{date}`
  - Document current state in commit message with compilation status verification

- **During Milestone Execution**:
  - Make frequent commits for each completed task with descriptive messages
  - Verify compilation and basic functionality before each commit
  - Use commit message format: `feat(milestone-X.Y): Task X.Y.Z - [brief description]`
  - Run quality gates before each commit: compilation, linting, basic tests

- **Milestone Completion Protocol**:
  ```bash
  # Verify quality gates
  cargo fmt && cargo clippy -- -D warnings && cargo test
  
  # Commit completed milestone
  git add .
  git commit -m "feat(milestone-3.1): Complete content organization - all tasks verified"
  
  # Merge to main only after full verification
  git checkout main
  git merge milestone-3.1-content-org-$(date +%Y%m%d)
  git tag -a "milestone-3.1-$(date +%Y%m%d)" -m "Milestone 3.1: Content Organization Complete"
  ```

### Git Commit Standards for Functional Programming Projects

- **MANDATORY Commit Categories**:
  - `feat(milestone-X.Y): Major milestone completion with all quality gates passed`
  - `refactor(fp): Pure function extraction or effect isolation improvements`
  - `fix(compilation): Resolve compilation errors or clippy warnings`
  - `test(property): Add property-based tests or ADT exhaustiveness verification`
  - `docs(guidelines): Update GUIDELINES.md or related documentation`

- **Pre-Commit Quality Gates** (MANDATORY verification before each commit):
  ```bash
  # Rust projects
  cargo fmt --check && cargo clippy -- -D warnings && cargo test
  
  # JavaScript projects  
  npm run lint && npm run test && npm run fp-check
  
  # TypeScript projects
  npm run lint && tsc --noEmit && npm run test
  ```

### Branch Protection and Rollback Procedures

- **Milestone Branch Strategy**:
  - Each milestone gets its own feature branch: `milestone-{assignment}-{milestone}-{YYYYMMDD}`
  - Main branch is protected - only verified milestone merges allowed
  - Failed milestones are abandoned in place, new branch created from last stable point

- **Rollback Procedures for Failed Milestones**:
  ```bash
  # If current milestone fails compilation or quality gates
  git stash push -m "Failed milestone 3.2 - compilation errors"
  git checkout main
  git checkout -b milestone-3.2-attempt2-$(date +%Y%m%d)
  
  # For recovery of specific files from last stable state
  git checkout main -- phases/GUIDELINES.md
  git commit -m "fix: Restore GUIDELINES.md from stable main branch"
  ```

- **Emergency Recovery Protocol**:
  - If multiple files are corrupted: `git reset --hard HEAD~N` to last stable commit
  - If main branch is corrupted: restore from last milestone tag
  - Always verify compilation after any rollback: `cargo build` or equivalent

### GitHub Integration Workflow

- **Stable Milestone Releases** (after major milestone completion):
  ```bash
  # After successful milestone completion and local verification
  git push origin main
  git push origin --tags
  
  # Create GitHub release for major milestones
  gh release create "milestone-3.1-$(date +%Y%m%d)" \
    --title "Milestone 3.1: Content Organization Complete" \
    --notes "All quality gates passed, compilation verified"
  ```

- **GitHub Repository Protection**:
  - Enable branch protection on main branch
  - Require pull request reviews for critical changes
  - Require status checks (compilation, linting) before merge
  - Enable automatic deletion of head branches after merge

### Automated Git Hooks for Quality Assurance

- **Pre-Commit Hook** (`/.git/hooks/pre-commit`):
  ```bash
  #!/bin/bash
  echo "Running pre-commit quality gates..."
  
  # Multi-language quality verification
  if [ -f "Cargo.toml" ]; then
    cargo fmt --check || exit 1
    cargo clippy -- -D warnings || exit 1
  fi
  
  if [ -f "package.json" ]; then
    npm run lint || exit 1
    if command -v tsc &> /dev/null; then
      tsc --noEmit || exit 1
    fi
  fi
  
  echo "All quality gates passed. Proceeding with commit."
  ```

- **Post-Commit Hook** (`/.git/hooks/post-commit`):
  ```bash
  #!/bin/bash
  # Log milestone progress
  git log --oneline -1 >> .milestone-progress.log
  echo "Commit logged to milestone progress tracking"
  ```

### Version Control Anti-Patterns to Avoid

- **PROHIBITED Git Practices**:
  - Committing broken code that doesn't compile
  - Large commits that combine multiple unrelated changes
  - Force pushing to main branch or shared branches
  - Merging branches without compilation verification
  - Working directly on main branch for milestone development
  - Ignoring merge conflicts or resolving them incorrectly

- **Recovery Guidelines**:
  - Never `git reset --hard` without creating a backup branch first
  - Always test compilation after any merge or rebase operation
  - Use `git reflog` to recover from accidental destructive operations
  - Maintain milestone tags as recovery points for critical failures

This version-safe workflow ensures that milestone progress is preserved and recoverable, preventing the loss of functional programming enhancements due to compilation failures or corrupted files.

## MCP Server Tools

For complex tasks and comprehensive development, you MUST utilize at least these MCP servers:

- **Sequential Thinking** (`mcp_sequential-th_sequentialthinking`): MANDATORY for complex problem-solving, chain-of-thoughts analysis, and multi-step tasks. Use this tool to:
  - Break down complex problems into manageable steps with functional programming principle application analysis
  - Perform thorough analysis and planning using FP pattern evaluation and selection
  - Validate solutions through structured thinking including pure function extraction planning, effect system design decisions, and domain model enhancement planning
  - Handle tasks that require context maintenance over multiple steps, especially for FP refactoring workflows
  - **FP-Specific Sequential Thinking Requirements**:
    - **Pure Function Extraction Analysis**: Use sequential thinking to analyze complex business logic and plan separation of pure computational functions from side-effectful operations
    - **Effect System Design Planning**: Use sequential thinking to design async patterns with Future composition, create effect system architectures with explicit dependency injection, and establish railway-oriented programming patterns
    - **Domain Model Enhancement Planning**: Use sequential thinking to design sum types (enum) for domain variations, implement product types (struct) with smart constructors, and create exhaustive pattern matching strategies
    - **Function Composition Strategy**: Use sequential thinking to design composable function interfaces, implement iterator chains and stream processing, and create monadic/applicative patterns

- **Time Management** (`mcp_time_convert_time` and `mcp_time_get_current_time`): Essential for time-aware operations:
  - Get current date and time for logging and timestamps
  - Handle timezone conversions for trading schedules
  - Ensure time-sensitive operations are properly scheduled

- **File System Operations** (`mcp_filesystem_*`): Direct file and directory operations:
  - Use for advanced file management beyond basic read/write
  - Directory tree operations and recursive searches
  - File metadata and permission management
  - Bulk file operations when needed

- **Memory Context** (`mcp_memory_*`): Preserve context across development sessions:
  - Store important project insights and decisions with functional programming pattern documentation
  - Maintain knowledge graphs of system relationships including functional architecture relationships and dependency flow
  - Track complex dependencies and interconnections with emphasis on effect boundaries and functional composition chains
  - Preserve learning from research and implementation, particularly FP refactoring experiences and pattern application outcomes
  - **FP-Specific Memory Context Requirements**:
    - **FP Principle Application Tracking**: Store decisions and rationale for applying specific functional programming principles (immutability patterns, pure function design, ADT modeling, effect isolation, composition techniques, etc.)
    - **FP Pattern Usage Documentation**: Track usage of functional architectural patterns (Functional Core/Imperative Shell, Event Sourcing + CQRS, Pipeline architecture, etc.) across development sessions
    - **Functional Architecture Relationship Mapping**: Maintain knowledge graphs showing relationships between pure functions, effect boundaries, domain types, and composition patterns
    - **FP Refactoring Experience Preservation**: Store learning from functional refactoring experiences including what patterns worked well, common pitfalls encountered, and effective transformation strategies


## Documentation System Structure

The project maintains a comprehensive documentation system organized into specialized documents, each providing a different "lens" for understanding the system. This multi-perspective approach ensures complete coverage of all aspects from technical implementation to user experience.

### Core Documentation Categories

- **`docs/ARCHITECTURE.md`**: System architecture document with emphasis on:
  - **FP Architecture Patterns**: Implementation of functional architectural patterns with detailed multi-language examples: Functional Core/Imperative Shell for effect isolation, Hexagonal/Ports & Adapters with trait-based abstractions, Event Sourcing + CQRS with immutable event streams, Pipeline/Data-oriented architecture with stream processing, Actor Model with message passing, FRP with reactive streams, Tagless-Final with effect polymorphism
  - **Module Organization with Functional Principles**: Functional Core/Imperative Shell module layout, effect boundary identification, dependency flow with trait abstractions, capability-based module access control
  - **Data Flow Architecture**: Pure data transformation pipelines, effect isolation boundaries, immutable data structure usage, stream-based processing architecture
  - **Dependency Management**: Dependency injection through traits and type parameters, capability-based security model, effect interpretation patterns, inversion of control through higher-order functions
  - **Error Handling Architecture**: Railway-oriented programming patterns, Result composition strategies, custom error effect types, graceful degradation with fallback functions
  - **Concurrency Model**: Actor-based message passing, CSP channel communication, async/await effect management, shared-nothing data parallelism
  - **Testing Architecture**: Property-based testing integration, pure function isolation for testing, mock/stub strategies through trait abstractions, integration test patterns with effect interpretation
- **`docs/MODULES.md`**: Detailed module APIs, responsibilities, internal structure, and inter-module relationships
- **`docs/ALGORITHMS.md`**: Mathematical foundations, algorithmic implementations, computational complexity, and theoretical background
- **`docs/FEATURE_CATALOG.md`**: Complete feature inventory with implementation status, priority, and roadmap

### Interface and Usage Documentation

- **`docs/API.md`**: Library API reference, function signatures, usage examples, and integration guidelines (for library components)
- **`docs/UX.md`**: User experience reference including CLI interface design, human-computer interaction patterns, usability guidelines, and interface documentation
- **`docs/EXAMPLES.md`**: Practical examples, real-world use cases, comprehensive code samples, and tutorial-style walkthroughs

### Operational Documentation

- **`docs/BUILD_RELEASE.md`**: Build processes, compilation artifacts, target platform specifications, cargo workspace layout, feature flags, release procedures, and deployment guidelines
- **`docs/CONFIGURATION.md`**: Configuration management including configuration files, centralized constants, hardcoded parameters, supported file formats, and configuration validation
- **`docs/TESTING.md`**: Testing strategy, test organization, quality gates, benchmarking procedures, code coverage guidelines, and testing best practices

### Programming Patterns and Design

- **`docs/PATTERNS_FUNCTIONAL.md`**: **PRIMARY** functional programming patterns catalog including:
  - **14 Core FP Principles**: Complete documentation of multi-language functional programming principles with concrete examples: immutability patterns, pure function design, higher-order function usage, function composition techniques, expression-oriented programming, ADT modeling, structural iteration, side-effect isolation, explicit effect typing, equational reasoning, time/randomness isolation, type-driven design, declarative concurrency, and property-based testing
  - **Architectural Pattern Implementation Guides**: Detailed functional pattern implementation with multi-language code examples: Functional Core/Imperative Shell module organization, Hexagonal/Ports & Adapters with trait abstractions, Event Sourcing + CQRS with immutable events, Pipeline/Data-oriented architecture with stream processing, Actor Model with message passing, FRP with reactive streams, Tagless-Final with effect polymorphism
  - **Effect System Pattern Documentation**: Comprehensive coverage of effect management patterns: Result/Option composition, Future/async patterns, custom effect types, railway-oriented programming, dependency injection for effects, capability-based design, Free Monads for effect interpretation
  - **Property-Based Testing Approach Documentation**: Complete guide to property-based testing with `proptest`/`quickcheck`: algebraic law verification (Functor, Monad, Monoid laws), round-trip property testing, invariant testing for domain types, snapshot/golden tests for event-sourced systems, model checking via pure simulation
  - **Optics**: Lens, Prism, Iso, Traversal for data access and manipulation
  - **Functional Foundations**: Functor → Applicative → Monad → Monad Transformers, Semigroup/Monoid for value composition, Bifunctor/Profunctor for enhanced composition
  - **Streams**: Iterator/Stream pipelines, FRP/Signals, Pipes/Pipelines for data flow
  - **State Management**: Actor Model, CSP, ECS, Typestate for state handling
  - **Recursive Schemes**: Catamorphism/Anamorphism/Hylomorphism for recursive data processing
  - **Useful Idioms**: Newtype/PhantomData, Smart/Validated-Constructors, Builder pattern through ownership

- **`docs/PATTERNS.md`**: **SECONDARY** programming patterns catalog with careful evaluation of traditional OOP patterns
  - **DEPRECATED PATTERNS**: Gang of Four patterns that conflict with Rust's functional paradigm and should be avoided or replaced with functional alternatives:
    - Singleton → Use module-level constants or dependency injection
    - Prototype → Use Clone trait or factory functions
    - Abstract Factory → Use trait objects with dependency injection
    - Factory Method → Use associated functions or builder patterns
    - Bridge → Use trait abstractions without complex hierarchies
    - Decorator → Use composition or trait extension
    - Strategy → Use function pointers or closures
    - Command → Use closures or function objects
    - State → Use enum state machines or typestate pattern
    - Visitor → Use pattern matching and trait methods
    - Template Method → Use higher-order functions or trait defaults
    - Mediator → Use channels or event systems
    - Chain of Responsibility → Use iterator chains or functional composition
  - **RUST-NATIVE ALTERNATIVES**: Modern Rust idioms that replace traditional OOP patterns
  - **EVALUATION GUIDELINES**: Criteria for when traditional patterns might still be appropriate

### Type System and Domain Modeling

- **`docs/TYPES.md`**: Functional type system documentation with emphasis on:
  - **Algebraic Data Types (ADTs)**: Comprehensive multi-language ADT modeling including: sum types (enums) for closed choice modeling, product types (structs) for data grouping, newtype patterns for type safety, phantom types for compile-time guarantees, tagged unions for safe state representation
  - **Type-Driven Design Principles**: Type system as specification language, illegal states unrepresentable through type design, smart constructors for invariant enforcement, builder patterns through ownership and typestate, validated constructors for input sanitization
  - **Functional Type Patterns**: Option/Result composition patterns, Iterator/Stream type usage, Higher-Kinded Type simulation through traits, effect type design (IO, State, Reader, Writer), capability types for dependency injection
  - **Type Safety Guarantees**: Compile-time invariant enforcement, ownership-based resource management, lifetime parameter usage for memory safety, const generics for compile-time validation, trait bounds for behavior specification
  - **Type System Integration**: From/Into conversion implementation for type interoperability, Display/Debug trait implementation for observability, serialization trait integration for persistence, Default implementation patterns for functional defaults
  - **Domain Modeling**: Bounded context type design, value object implementation, entity type patterns, domain event type modeling, aggregate root type structure
  - **Error Type Design**: Custom error types with structured information, error composition through enum hierarchies, Result-based error propagation, error recovery patterns through type design

### Project Management Documentation

- **`README.md`**: Project overview, quick start guide, installation instructions, and basic usage examples
- **`CHANGELOG.md`**: Version history, change documentation, migration guides, and release notes

### Documentation Maintenance Guidelines

- Each documentation file must be updated as part of relevant milestones
- Documentation updates must be planned as specific milestones for comprehensive changes
- Cross-references between documentation files must be maintained for consistency
- All documentation must be written in English and follow consistent formatting standards
- Code examples in documentation must be verified for accuracy and kept up-to-date with implementation changes

## Development Principles

The following principles are non-negotiable and must be strictly adhered to throughout all phases of development.

- **Modern Functional Programming First**: Multi-language functional programming capabilities are the primary approach across Rust, JavaScript, and TypeScript. Traditional OOP patterns should be carefully evaluated and often replaced with functional alternatives that better align with modern functional paradigms. This principle encompasses 14 core functional programming rules:

#### Core Functional Programming Principles

| # | Principle | Why it matters (pay-off) | Multi-Language Guidance |
|---|-----------|--------------------------|--------------------------|
| **1** | **Immutability of data** | Eliminates whole classes of race-conditions and order-dependent bugs; makes values thread-safe by default. | **Rust**: Treat every value as read-only after construction; build new values via `.map`/builder syntax rather than mutation<br>**JavaScript**: Use `Object.freeze()`, spread operators, and immutable libraries like Immer<br>**TypeScript**: Leverage `readonly` modifiers and `as const` assertions for compile-time immutability |
| **2** | **Pure functions & referential transparency** | A pure expression can be replaced by its result without altering program behaviour, enabling equational reasoning, algebraic optimisation and automatic memoisation. | **Rust**: No hidden inputs (time, RNG, globals); no hidden outputs (I/O, mutation); return new state instead of mutating arguments<br>**JavaScript**: No side effects, deterministic output for same input, avoid `Date.now()` and global access<br>**TypeScript**: Use function signatures to express purity, avoid `any` type that hides effects |
| **3** | **First-class & higher-order functions** | Behaviour becomes data: we pass, return and compose functions just like any other value. | **Rust**: Pass lambdas/`impl Fn` rather than building strategy or command class hierarchies<br>**JavaScript**: Leverage closures, `Array.map()`, `filter()`, `reduce()` and function composition<br>**TypeScript**: Use generic function types `<T>(x: T) => U` and conditional types for function composition |
| **4** | **Function composition over explicit control flow** | Small, single-purpose functions can be glued with composition, pipes, or iterator/stream chains, yielding declarative pipelines that are easy to reorder and parallelise. | **Rust**: Build pipelines of transformations; avoid "push" style callbacks except at system edges<br>**JavaScript**: Use pipeline operator `\|>` (proposal), function composition libraries, or method chaining<br>**TypeScript**: Type-safe pipelines with `pipe()` functions and fluent interfaces |
| **5** | **Expressions ≥ statements** | Functions return values; control constructs are themselves expressions. This keeps all flows explicit in the type system. | **Rust**: Favour `if`, `match`, `loop { … break val }` as value-producing blocks; avoid mutating variables in distant scopes<br>**JavaScript**: Use ternary operators, `&&`/`\|\|` operators, and expression-based control flow<br>**TypeScript**: Leverage discriminated unions with expression-based pattern matching |
| **6** | **Algebraic data types (ADTs) & pattern matching** | Sum- and product-types encode domain constraints directly; exhaustive pattern matching gives the compiler proof that every case is handled. | **Rust**: Model domain variations with `enum` + struct fields; make invalid states unrepresentable<br>**JavaScript**: Use tagged unions and libraries like fp-ts for ADT support<br>**TypeScript**: Discriminated unions with `never` type for exhaustiveness checking |
| **7** | **Recursion & structural iteration abstracted through folds** | Abstracting "iterate until you hit the base case" into `fold`, `map`, `filter` removes boilerplate and separates traversal from business logic. | **Rust**: Use iterator adaptors, `fold`, or explicit recursion on ADTs; never index into raw arrays unless performance profiling proves it<br>**JavaScript**: Use `Array.reduce()`, recursive functions, and avoid manual loop indexing<br>**TypeScript**: Type-safe folds with proper accumulator typing and recursive type definitions |
| **8** | **Side-effects pushed to the edge** | Keeps the pure core maximally testable and composable; side-effects are localised and easier to audit. | **Rust**: Isolate I/O, logging, DB access in thin wrappers; pure domain functions receive data already collected<br>**JavaScript**: Functional core, imperative shell pattern; isolate DOM manipulation and API calls<br>**TypeScript**: Use IO monads or effect libraries like fp-ts to isolate side effects |
| **9** | **Explicit effect typing / effect segregation** | Whether via monads, `Result`, `Future`, or effect systems, making effects explicit lets the compiler enforce correct sequencing and error handling. | **Rust**: Propagate failure with `?`, represent async work via `Future`, model optionality with `Option`; avoid naked panics<br>**JavaScript**: Use Promise chains, async/await, and explicit error handling with Result-like patterns<br>**TypeScript**: Use `Promise<T>`, `Either<E, T>` types, and effect libraries for explicit effect management |
| **10** | **Equational reasoning & lawfulness** | If a transformation obeys algebraic laws (Functor, Monad, Monoid…), we can refactor fearlessly and leverage generic libraries. | **Rust**: Document and test algebraic laws (Functor, Monad, Monoid) with property-based tests<br>**JavaScript**: Use libraries that provide lawful abstractions and test algebraic properties<br>**TypeScript**: Encode laws in type system using phantom types and branded types |
| **11** | **Isolation of time & randomness** | Time, UUIDs, random numbers violate referential transparency; treat them as explicit dependencies so they can be mocked deterministically. | **Rust**: Inject `Clock`/`Rng` trait objects; never call `SystemTime::now()` in core logic<br>**JavaScript**: Inject time/random dependencies, avoid `Date.now()` and `Math.random()` in business logic<br>**TypeScript**: Use dependency injection with interfaces for time and randomness sources |
| **12** | **Type-driven design & totality** | Use the type checker as the first line of defence: prefer total functions (defined for every input) and leverage the compiler to forbid illegal states. | **Rust**: Exhaustive `match`; avoid `unwrap()`; embrace newtypes and phantom types for invariants<br>**JavaScript**: Use runtime validation libraries and functional programming patterns for totality<br>**TypeScript**: Use `never` type for exhaustiveness, branded types, and strict compiler options |
| **13** | **Declarative concurrency (streams, actors, CSP)** | Pure functions + immutable data make concurrency safe; message-passing removes shared mutable state. | **Rust**: Compose `Stream<Item=…>` chains; communicate via channels; design actors with private state<br>**JavaScript**: Use async iterators, Web Workers with message passing, and reactive streams<br>**TypeScript**: Type-safe actor patterns and async stream processing with proper typing |
| **14** | **Property-based testing as default** | Because functions are pure and deterministic, large random test spaces can be explored automatically, catching edge cases early. | **Rust**: Use `proptest`/`quickcheck` to assert algebraic laws, invariants, and round-trip properties<br>**JavaScript**: Use fast-check for property-based testing and invariant checking<br>**TypeScript**: Leverage type system to generate test cases and use typed property-based testing |

#### Multi-Language Quality Checklist

**Universal Rules** (apply to all languages):

1. **No hidden state:** Every dependency is an argument
   - **Measurable criteria**: Functions take explicit parameters for time, randomness, configuration
   - **Concrete actions**: Inject Clock/Time interfaces, pass configuration objects, avoid global variables
   - **Validation**: Search codebase for direct calls to `Date.now()`, `Math.random()`, `process.env`

2. **Return new state, don't mutate old**
   - **Measurable criteria**: Functions return modified copies instead of mutating inputs
   - **Concrete actions**: Use immutable update patterns, functional array methods, builder patterns
   - **Validation**: Code review for mutation of input parameters, use immutability linters

3. **Treat effects as data you schedule, not actions you perform immediately**
   - **Measurable criteria**: Effects represented in type signatures, composed declaratively
   - **Concrete actions**: Use Result/Either types, Promise chains, effect libraries
   - **Validation**: Functions with I/O have appropriate effect types in signatures

4. **Keep the impure layer thin; push it to the system's edge**
   - **Measurable criteria**: Core business logic has no I/O, effects isolated to boundaries
   - **Concrete actions**: Separate domain logic from I/O handlers, dependency injection at edges
   - **Validation**: Core modules have zero import of I/O libraries

5. **Model the domain and nothing else with types; let the compiler enforce the rules**
   - **Measurable criteria**: Invalid states unrepresentable, domain invariants in type system
   - **Concrete actions**: Use sum types for variations, smart constructors, branded types
   - **Validation**: Domain types prevent construction of invalid states

6. **Compose; don't inherit**
   - **Measurable criteria**: Functionality built through composition, minimal inheritance hierarchies
   - **Concrete actions**: Use function composition, trait composition, delegation patterns
   - **Validation**: Inheritance depth metrics, composition over inheritance ratios

7. **Write properties before writing the code; the compiler and test runner become co-authors**
   - **Measurable criteria**: Property-based tests for core functions, algebraic laws verified
   - **Concrete actions**: Use quickcheck/fast-check/proptest, test round-trip properties
   - **Validation**: Coverage of property-based tests, law verification for abstractions

**Language-Specific Considerations:**

**Rust Implementation Guidance:**
- **Leverage ownership system**: Use borrowing (`&T`) over cloning, design APIs around ownership transfer
- **Zero-cost abstractions**: Prefer iterators over manual loops, use trait objects for dynamic dispatch
- **Trait system**: Define minimal trait interfaces, use associated types for type families
- **Immutability**: Default to `&T`, use `Arc<T>` for shared data, apply builder patterns for configuration
- **Effect management**: Use `Result<T, E>` for errors, `Future<T>` for async, `Option<T>` for optionals
- **Concrete actions**: Enable `#![deny(unused_variables)]`, use `cargo clippy`, implement `From`/`Into` traits

**JavaScript Implementation Guidance:**
- **Functional libraries**: Use fp-ts for type-safe FP, Ramda for utility functions, Immer for immutability
- **Modern ES features**: Leverage destructuring, spread operators, optional chaining, nullish coalescing
- **Runtime validation**: Use joi, yup, or zod for input validation; implement Result-like error handling
- **Immutability**: Use `Object.freeze()`, spread operators, avoid mutating methods like `.push()`
- **Effect management**: Use Promise chains, async/await consistently, avoid mixing callback styles
- **Concrete actions**: Enable strict mode, use ESLint functional plugins, implement proper error boundaries

**TypeScript Implementation Guidance:**
- **Compile-time guarantees**: Use strict compiler flags, enable `exactOptionalPropertyTypes`, `noUncheckedIndexedAccess`
- **Advanced types**: Leverage discriminated unions, mapped types, conditional types, template literal types
- **Runtime safety**: Combine static typing with runtime validation, use branded types for domain modeling
- **Effect typing**: Use `Promise<T>`, `Either<E, T>` patterns, make async effects explicit in signatures
- **Type-level programming**: Use phantom types, const assertions, recursive type definitions
- **Concrete actions**: Configure strict tsconfig.json, use type-only imports, implement exhaustiveness checking
- **Immutability by Default**: Data structures must be immutable unless explicitly wrapped in concurrency primitives. Prefer `&T` over `&mut T` in API design; use `Arc<T>` for shared immutable data; apply builder patterns for "mutation" (`config.with_timeout(Duration::from_secs(30))`). This eliminates race conditions, makes values thread-safe by default, and simplifies reasoning about state changes.
- **Pure Functions and Referential Transparency**: Functions must be pure unless explicitly marked as effectful. Prohibit hidden state access (`SystemTime::now()`, global variables, thread-local storage) in business logic. Prohibit mutation via `&mut` parameters in pure contexts. Use dependency injection for non-deterministic sources: `trait Clock { fn now(&self) -> Instant }`, `trait RandomSource { fn gen_range(&mut self, range: Range<u32>) -> u32 }`.
- **Explicit Effect Management**: All side-effects must be represented in type signatures. Use `Result<T, E>` for fallible operations, `Future<Output = T>` for async work, `Option<T>` for optional values. Avoid `unwrap()` in business logic; prefer error propagation with `?`. Design custom effect types for complex workflows. Effects enable compiler-enforced correct sequencing and prevent hidden side-effects.
- **Function Composition Over Control Flow**: Build declarative pipelines of transformations using iterator chains (`data.iter().filter(|x| x.is_valid()).map(|x| x.process()).collect()`), stream processing, and functional combinators. Avoid imperative control flow in favor of functional composition. Each stage should be testable and thread-safe with back-pressure via pull semantics.
- **Expression-Oriented Programming**: Use `if expr { value1 } else { value2 }` instead of mutable variables; use `match` expressions for all control flow; use `loop { … break value }` for complex iterations. Functions return values; control constructs are expressions. This keeps all data flows explicit in the type system and eliminates statement-heavy imperative code.

### Algebraic Data Types and Domain Modeling

- **Make Illegal States Unrepresentable**: Use `enum` and `struct` to represent every domain concept, ensuring the compiler prevents invalid states. Design domain models as: `enum PaymentStatus { Pending, Completed(Receipt), Failed(Error) }`. Use sum types (`enum`) for variations and product types (`struct`) for combinations. The type system becomes the first line of defense against domain errors.
- **Exhaustive Pattern Matching**: All `match` expressions must be exhaustive without `_` catch-alls unless explicitly justified. The compiler checks exhaustiveness, ensuring no domain case is forgotten. Use `match payment_status { Pending => handle_pending(), Completed(receipt) => process_receipt(receipt), Failed(error) => handle_error(error) }`.
- **Newtype Patterns for Type Safety**: Wrap primitive types in domain-specific newtypes: `struct UserId(u64)`, `struct EmailAddress(String)`. This prevents mixing different concepts and enables domain-specific validation. Newtypes are zero-cost abstractions that provide compile-time safety without runtime overhead.
- **Smart Constructor Patterns**: Use private fields with public validated constructors to enforce invariants at construction time. `impl EmailAddress { pub fn new(email: String) -> Result<Self, ValidationError> { validate_email(&email)?; Ok(EmailAddress(email)) } }`. This ensures domain objects are always in valid states and pushes validation to the boundaries.

### Side-Effect Isolation and Architecture

- **Functional Core, Imperative Shell**: Keep business logic in pure, side-effect-free modules (the Core); isolate all I/O, mutation, and integration in a thin outer layer (the Shell). Organize modules as: `domain/` (pure business logic), `infrastructure/` (I/O adapters), `application/` (coordination layer). This maximizes testability, compiler guarantees, and reusability of core logic while confining complexity to the edges.
- **Dependency Injection for Effects**: Use trait abstractions for effects, avoiding global state access. Define minimal capability interfaces: `trait DatabaseRepo { fn save(&self, entity: Entity) -> Result<(), DbError>; }`, `trait Clock { fn now(&self) -> Instant; }`. Pass only the capabilities each module needs rather than allowing arbitrary effect access. This enables testing with mock implementations and maintains pure functional cores.
- **Capability-Based Design**: Design minimal trait interfaces avoiding "god traits" that provide too many capabilities. Each trait should represent a single, focused capability. Use composition of traits rather than large monolithic interfaces: `fn process_payment<C: Clock, D: DatabaseRepo>(clock: &C, db: &D, payment: Payment) -> Result<Receipt, PaymentError>`.
- **Time and Randomness Isolation**: Inject time and randomness as explicit dependencies through trait objects. Never call `SystemTime::now()`, `rand::random()`, or similar non-deterministic functions directly in core logic. Use `trait Clock { fn now(&self) -> Instant; }` and `trait RandomSource { fn gen_range(&mut self, range: Range<u32>) -> u32; }` for deterministic testing and referential transparency.
- **Simplicity and Clarity**: Solutions must be as simple and clear as reasonably possible. Avoid unnecessary complexity in code, architecture, and logic. Prefer functional composition over complex object hierarchies.
- **Zero Tolerance for Waste**: All forms of waste—including duplicate code, unused variables, dead and legacy code, and inefficient logic—are strictly prohibited. All shared functionality must be centralized and reusable through functional abstractions.
- **Standardization**: Adherence to established coding standards, conventions (`cargo fmt` and `cargo fix --allow-dirty`), and best practices is mandatory.
- **Rust-Native Pattern Usage**: Apply functional programming patterns thoughtfully. Avoid forcing traditional OOP patterns where Rust's native functional features provide better solutions. The 23 Gang of Four patterns often introduce unnecessary complexity and should be replaced with functional alternatives.
- **Reliability and Stability**: All implementations must prioritize correctness, robustness, and predictable behavior under all operating conditions using Rust's type system and functional guarantees.
- **Vigilance Against Anti-Patterns**: Actively identify and avoid common anti-patterns, including forced application of OOP patterns that conflict with Rust's functional nature.

#### Functional Programming Architectural Patterns

The following architectural patterns flow naturally from functional programming principles and provide proven approaches for building medium- to large-scale systems across Rust, JavaScript, and TypeScript:

| Pattern | Description | Multi-Language Benefits | Implementation Guidance |
|---------|-------------|------------------------|-------------------------|
| **Hexagonal / Ports & Adapters** | Core domain logic depends only on abstract ports (traits/interfaces); adapters implement I/O and external integrations | **Rust**: Use traits for ports, isolates pure code from I/O<br>**JavaScript**: Use dependency injection, function composition for adapters<br>**TypeScript**: Interface-based ports with strong typing | Mirror Functional Core/Imperative Shell pattern; domain logic receives data through abstract ports |
| **Event Sourcing + CQRS** | All state changes appended as immutable events; read and write models are separated | **Rust**: Events as immutable structs, `Vec<Event>` streams<br>**JavaScript**: Immutable event objects, functional event handlers<br>**TypeScript**: Discriminated union event types, type-safe reducers | Events are pure data; replay and audit become trivial; enables time-travel debugging |
| **Pipeline / Dataflow Architecture** | Data transformed through a chain of pure stages using streams and iterators | **Rust**: Iterator chains, `Stream` processing with `tokio-stream`<br>**JavaScript**: Array methods chaining, async generators, reactive streams<br>**TypeScript**: Type-safe pipeline composition with mapped types | Each stage is testable and thread-safe; back-pressure via pull semantics |
| **Actor Model** | Entities encapsulate state and communicate by immutable messages | **Rust**: Message passing with `tokio::mpsc`, actor frameworks like `actix`<br>**JavaScript**: Web Workers with message passing, CSP-style channels<br>**TypeScript**: Typed message protocols, async actor communication | No shared mutable state; pure message handlers; enables fault tolerance |
| **Functional Reactive Programming (FRP)** | System reacts to streams of events via declarative signal compositions | **Rust**: `futures::stream` combinators, reactive pattern libraries<br>**JavaScript**: RxJS observables, reactive streams with operators<br>**TypeScript**: Type-safe reactive programming with RxJS, async iterables | State updates and side-effects isolated to event boundaries |
| **Tagless-Final / Interpreter Pattern** | Define DSLs as trait interfaces; supply multiple effectful interpreters | **Rust**: Generic traits over effect types `F[_]`, effect polymorphism<br>**JavaScript**: Higher-order functions for effect interpretation<br>**TypeScript**: Generic effect types, phantom types for effect tracking | Business logic is pure and polymorphic over the effect type |

#### Effect System Architecture Patterns

- **Tagless Final**: Encapsulate domain operations in generic traits (`trait Algebra<F[_]>`) and implement for `F = Future`, `F = IO`, `F = TestMonad`. Core code stays pure and polymorphic.
  - **Rust Example**: `trait UserService<F[_]> { fn get_user(&self, id: UserId) -> F<Option<User>>; }`
  - **TypeScript Example**: `interface UserService<F> { getUser(id: UserId): F<Option<User>>; }`
  - **JavaScript Example**: Function composition with effect containers and interpreters

- **Free Monads / Free Applicatives**: Describe programs as pure data structures, then interpret at the edge. Useful for complex workflows and retry logic.
  - **Rust**: Custom AST types with interpreter functions for different effect contexts
  - **JavaScript**: Command pattern with functional composition and interpreter functions
  - **TypeScript**: Discriminated unions for command types with type-safe interpreters

- **Capability-Based Design**: Pass only minimal capabilities (traits/interfaces) each module needs rather than allowing arbitrary effect access.
  - **Universal Pattern**: `trait Clock`, `trait RandomSource`, `trait Logger` instead of global access
  - **Dependency Injection**: Functions receive only the capabilities they need
  - **Testing**: Easy mocking and deterministic testing through capability injection

#### Concurrency and Parallelism Patterns

- **Immutable Data + Message-Passing**: Use channels and message passing instead of shared locks across all languages.
  - **Rust**: `tokio::mpsc`, `crossbeam` channels, actor frameworks (`actix`, `xtra`)
  - **JavaScript**: Web Workers, MessageChannel API, CSP-style communication
  - **TypeScript**: Type-safe message protocols, async communication patterns

- **Declarative Parallel Pipelines**: Split large data pipelines into independent stages connected via bounded channels.
  - **Rust**: Async streams with `tokio-stream`, parallel iterator processing
  - **JavaScript**: Async generators, Transform streams, worker pools
  - **TypeScript**: Type-safe stream processing, async iterator composition

- **Typestate for Protocols**: Encode protocol phases in types so invalid transitions become compile errors.
  - **Rust**: `Socket<Connected>` vs `Socket<Closed>`, phantom types for state
  - **TypeScript**: Branded types and literal types for protocol states
  - **JavaScript**: Runtime validation with clear state documentation

#### Testing and Verification Patterns

- **Property-Based Testing**: Write laws for core modules (monoid associativity, functor laws, round-trip properties).
  - **Rust**: `proptest`, `quickcheck` for law verification and invariant testing
  - **JavaScript**: `fast-check` for property-based testing, algebraic law verification
  - **TypeScript**: Type-safe property generation with compile-time guarantees

- **Snapshot / Golden Tests**: For event-sourced systems, assert that event sequences produce expected final states.
  - **Universal**: Pure event replay for deterministic state reconstruction
  - **Time-Travel Testing**: Replay events to any point in time for debugging

- **Model Checking via Pure Simulation**: Simulate distributed protocols purely (no networking), exploring state-space exhaustively.
  - **Pure Simulation**: No I/O during testing, complete state-space exploration
  - **Deterministic Testing**: Inject controlled time, randomness, and network conditions

#### Architectural Integration Principles

1. **Start with a pure core**: Model domain with ADTs, pure functions, and abstract effect interfaces
2. **Define ports for each side-effect**: Use traits/interfaces like `trait Repo { fn save(&self, e: Event) -> F<()> }`
3. **Compose pipelines of pure transformations**: Use iterators, streams, or custom fold/map chains
4. **Interpret effects at the boundary**: Provide real I/O implementations or test interpreters
5. **Isolate concurrency**: Use actors or CSP-style channels so no two threads mutate the same data
6. **Layer effects systematically**: Separate error handling, async operations, and I/O into distinct layers

By adhering to these architectural patterns, systems inherit functional programming's guarantees—leading to safer, more modular, and more evolvable code across all supported languages.

### Language-Specific Anti-Patterns to Avoid

Before implementing any plan, you MUST verify that your approach does not introduce these anti-patterns in your target language:

#### Universal Anti-Patterns (All Languages)
- **Over-engineering with abstractions** where simpler solutions suffice - prefer concrete implementations when abstractions don't add clear value
- **Heavy macro/metaprogramming use** that hides logic and makes code harder to debug - use sparingly and only for genuine code generation needs
- **God objects or modules** - split large modules into focused, cohesive submodules (>300 lines as general guideline)
- **Shotgun surgery** - changes requiring modifications across many files indicate poor separation of concerns
- **Feature creep in single modules** - modules that handle too many unrelated responsibilities
- **Copy-paste programming** - duplicating similar logic instead of extracting common functionality
- **Premature optimization** - optimizing before measuring and identifying actual bottlenecks
- **Silent failures** - swallowing errors without logging or propagating them appropriately

#### Rust-Specific Anti-Patterns

**Memory and Ownership Anti-Patterns:**
- **Ignoring proper lifetime annotations** - leads to confusing borrow errors and unclear ownership semantics
- **Unnecessary cloning** - excessive use of `.clone()` instead of proper borrowing or reference management
- **Reference counting abuse** - overusing `Rc<RefCell<T>>` instead of proper ownership design
- **Memory leaks through cycles** - circular references in `Rc` structures without using `Weak` references
- **Fighting the borrow checker** - restructure code rather than adding unnecessary `unsafe` blocks

**Rust Error Handling Anti-Patterns:**
- **Panic-driven development** - using `.unwrap()` or `.expect()` in production code without proper justification
- **String-based errors** - using `String` for errors instead of proper error types with `thiserror` or `anyhow`
- **Result<(), String>** - prefer structured error types over generic string errors

**Rust Performance Anti-Patterns:**
- **Unnecessary allocations** - creating `Vec` or `String` when iterators or string slices would suffice
- **Early collection** - calling `.collect()` on iterators when chaining operations is possible
- **Synchronous blocking** in async contexts - using blocking operations in async functions

#### JavaScript-Specific Anti-Patterns

**JavaScript Type Safety Anti-Patterns:**
- **Excessive use of `any` or loose equality** - undermines type safety and predictability
- **Global variable pollution** - avoid creating global state that breaks functional principles
- **Prototype pollution** - modifying built-in prototypes can cause unexpected behavior
- **Implicit type coercion reliance** - prefer explicit conversions for clarity

**JavaScript Functional Programming Anti-Patterns:**
- **Mutating array methods** - use immutable methods like `.map()`, `.filter()` instead of `.push()`, `.splice()`
- **Side effects in array callbacks** - array methods should use pure functions
- **Callback hell** - prefer async/await or Promise composition over nested callbacks
- **Mixing async patterns** - don't mix Promises with callbacks unnecessarily

**JavaScript Performance Anti-Patterns:**
- **Unnecessary object creation in loops** - hoist object creation outside hot paths
- **DOM thrashing** - batch DOM operations and use functional reactive patterns
- **Memory leaks from closures** - be careful with closure scope and clean up event listeners

#### TypeScript-Specific Anti-Patterns

**TypeScript Type System Anti-Patterns:**
- **Excessive `any` usage** - defeats the purpose of TypeScript's type safety
- **Type assertions over type guards** - prefer runtime type checking with proper guards
- **Weak typing with `object` or `{}` types** - use proper interface or type definitions
- **Ignoring strict mode flags** - enable strict TypeScript compiler options for better safety

**TypeScript Functional Programming Anti-Patterns:**
- **Mutation of readonly types** - respect readonly annotations in implementation
- **Bypassing discriminated union exhaustiveness** - avoid default cases that ignore new variants
- **Weak generic constraints** - use proper type constraints for generic functions
- **Type-only imports mixed with runtime** - separate type imports for clarity

**TypeScript Effect Handling Anti-Patterns:**
- **Hidden async effects** - make async operations explicit in type signatures
- **Untyped Promise chains** - properly type Promise return values
- **Weak error types** - use proper error union types instead of generic Error

#### Cross-Language Functional Programming Anti-Patterns

**Anti-Functional Design Patterns:**
- **Forced OOP Pattern Application** - applying Gang of Four patterns where functional alternatives are more appropriate:
  - **Singleton Pattern** → Use dependency injection, module-level constants, or capability-based design
  - **Factory Patterns** → Use associated functions, builder patterns with method chaining, or functional constructors
  - **Strategy Pattern** → Use higher-order functions, closures, or enum with associated functions
  - **Command Pattern** → Use closures, function composition via combinators, or message passing with channels
  - **State Pattern** → Use typestate pattern with phantom types, or enum state machines with exhaustive pattern matching
  - **Visitor Pattern** → Use pattern matching, trait methods with defaults, or fold operations
  - **Template Method Pattern** → Use higher-order functions, closures with customizable behavior, or trait defaults
  - **Observer Pattern** → Use reactive streams, pub/sub channels, or event-driven architectures
  - **Decorator Pattern** → Use composition with wrapper types, trait extension via delegation, or functional combinators
  - **Chain of Responsibility** → Use iterator chains, functional composition with `and_then`, or pipeline patterns

**Type Safety Violations:**
- **Stringly-typed interfaces** - using strings for data that should have proper types
- **Nested option/optional types** - nested optionals usually indicate design issues and should be flattened
- **Magic number usage** - embed constants in dedicated constant modules with proper naming

#### Functional Error Handling Anti-Patterns
- **Exception-Style Error Handling** - using panic-based error handling patterns instead of functional error management:
  - Using `panic!`, `unwrap()`, or `expect()` for recoverable errors in core business logic
  - Implementing `Drop` trait to perform error recovery instead of explicit error handling
  - Using global error handlers or exception-like mechanisms rather than local error propagation
  - Throwing errors across API boundaries without representing them in type signatures
- **Error Type Erosion** - degrading specific error information through inappropriate type conversions:
  - Converting specific errors to generic `String` or `Box<dyn Error>` too early in the call stack
  - Using `anyhow::Error` in library code where specific error types would provide better API design
  - Losing error context through inappropriate `map_err` transformations
  - Flattening error hierarchies prematurely instead of preserving structured error information
- **Early Error Unwrapping** - extracting error values inappropriately instead of propagating them functionally:
  - Using `unwrap()` in intermediate functions instead of propagating errors with `?` operator
  - Pattern matching on `Result` to extract values when error propagation would be more appropriate
  - Converting errors to `Option` and losing error information with `ok()` method
  - Handling errors locally when they should be propagated to higher-level error boundaries
- **Railway-Oriented Programming Violations** - failing to use proper functional error composition patterns:
  - Not using `?` operator for error propagation in fallible function chains
  - Avoiding `map`, `and_then`, `or_else` combinators for error handling composition
  - Implementing manual error checking instead of leveraging `Result`'s functional interface
  - Creating nested `Result` types instead of flattening with proper combinator usage

#### Functional Programming Violation Anti-Patterns
- **Hidden State Access** - accessing non-deterministic sources without explicit dependency injection:
  - Calling `SystemTime::now()` directly in business logic instead of injecting `Clock` trait
  - Using global variables or thread-local storage in pure functions
  - Accessing environment variables directly rather than through configuration injection
  - Using `rand::random()` or similar functions without injecting `RandomSource` trait
- **Imperative Shell Leakage** - mixing pure business logic with side-effect operations:
  - Performing I/O operations (file access, network calls, database queries) within pure functions
  - Embedding business logic directly in I/O handlers instead of separating concerns
  - Mutating shared state from within otherwise pure computational functions
  - Logging or printing from core domain logic instead of returning structured results
- **Effect Hiding** - concealing side-effects that should be explicit in type signatures:
  - Using `unwrap()` or `expect()` to hide `Result` types in intermediate functions
  - Making blocking calls in async contexts without proper `async`/`await` marking
  - Performing fallible operations without representing failure in return types
  - Using `panic!` for recoverable errors instead of proper error propagation
- **Mutation in Pure Contexts** - introducing mutation where immutability is expected:
  - Using `&mut` parameters in functions intended to be pure transformations
  - Modifying data structures in-place when functional updates would be more appropriate
  - Sharing mutable references across thread boundaries without proper synchronization
  - Using interior mutability (`RefCell`, `Cell`) in contexts where immutability is preferred
- **Temporal Coupling** - creating functions that depend on implicit call order or hidden state:
  - Functions that depend on previous function calls without explicit state parameters
  - APIs that require specific initialization sequences without type-level enforcement
  - Stateful functions that behave differently based on previous invocations
  - Global state modifications that affect unrelated function behavior

#### Functional Programming Migration Requirements
- **MANDATORY**: Replace deprecated OOP patterns with functional alternatives
- **MANDATORY**: Use Rust's native functional features (closures, iterators, pattern matching) over complex object hierarchies
- **MANDATORY**: Prefer composition over inheritance through functional composition and trait objects
- **MANDATORY**: Use algebraic data types (enums) and pattern matching over complex class hierarchies
- **MANDATORY**: Apply functional error handling with Result and Option types over exception-based patterns

#### Testing and Documentation Anti-Patterns
- **Test-driven damage** - writing tests that don't actually verify meaningful behavior
- **Missing edge case coverage** - only testing the "happy path" scenarios
- **Outdated documentation** - comments and docs that don't reflect current implementation
- **No integration tests** - relying solely on unit tests without end-to-end validation

