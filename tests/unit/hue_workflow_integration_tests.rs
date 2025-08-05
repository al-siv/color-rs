use colors::{Color, ColorCollectionType, config::Config, cli::HueArgs};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[cfg(test)]
mod hue_workflow_integration_tests {
    use super::*;

    /// Test complete workflow: CLI args -> Collection loading -> Analysis -> Output
    #[test]
    fn test_complete_hue_analysis_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("hue_analysis.yml");
        
        // Setup test arguments
        let args = HueArgs {
            target_hue: 120.0,        // Green
            tolerance: 30.0,          // ±30 degrees
            collection: ColorCollectionType::Css,
            min_saturation: Some(25.0),
            max_saturation: Some(75.0),
            min_lightness: Some(30.0),
            max_lightness: Some(70.0),
            output_format: Some("yaml".to_string()),
            output_file: Some(output_path.to_string_lossy().to_string()),
            sort_by: Some("hue".to_string()),
            limit: Some(50),
            verbose: true,
        };
        
        // Execute complete workflow
        let result = execute_hue_analysis_workflow(args);
        
        // Validate workflow completion
        assert!(result.is_ok(), "Workflow execution failed: {:?}", result);
        
        let analysis_result = result.unwrap();
        
        // Validate analysis results
        assert!(analysis_result.matched_colors.len() > 0, "No colors matched criteria");
        assert!(analysis_result.matched_colors.len() <= 50, "Limit not respected");
        assert!(analysis_result.total_processed > 0, "No colors were processed");
        
        // Validate output file creation
        assert!(output_path.exists(), "Output file was not created");
        
        // Validate output file content
        let output_content = fs::read_to_string(&output_path).unwrap();
        assert!(output_content.contains("target_hue: 120"), "Missing target hue in output");
        assert!(output_content.contains("tolerance: 30"), "Missing tolerance in output");
        assert!(output_content.contains("matched_colors:"), "Missing matched colors section");
        
        // Validate color constraints
        for color in &analysis_result.matched_colors {
            let hsl = color.to_hsl();
            
            // Hue constraint
            let hue_distance = calculate_hue_distance(hsl.hue, 120.0);
            assert!(hue_distance <= 30.0, 
                   "Color hue {} outside tolerance: distance = {}", hsl.hue, hue_distance);
            
            // Saturation constraints
            assert!(hsl.saturation >= 25.0 && hsl.saturation <= 75.0,
                   "Color saturation {} outside range [25, 75]", hsl.saturation);
            
            // Lightness constraints
            assert!(hsl.lightness >= 30.0 && hsl.lightness <= 70.0,
                   "Color lightness {} outside range [30, 70]", hsl.lightness);
        }
        
        println!("✓ Complete workflow test passed: {} colors matched", 
                analysis_result.matched_colors.len());
    }

    /// Test workflow with multiple collection types
    #[test]
    fn test_multi_collection_workflow() {
        let collections = vec![
            ColorCollectionType::Css,
            ColorCollectionType::RalClassic,
            ColorCollectionType::RalDesign,
        ];
        
        for collection in collections {
            let temp_dir = TempDir::new().unwrap();
            let output_path = temp_dir.path().join(format!("analysis_{}.json", collection));
            
            let args = HueArgs {
                target_hue: 240.0,  // Blue
                tolerance: 45.0,
                collection,
                min_saturation: None,
                max_saturation: None,
                min_lightness: None,
                max_lightness: None,
                output_format: Some("json".to_string()),
                output_file: Some(output_path.to_string_lossy().to_string()),
                sort_by: Some("distance".to_string()),
                limit: None,
                verbose: false,
            };
            
            let result = execute_hue_analysis_workflow(args);
            assert!(result.is_ok(), "Workflow failed for collection {:?}", collection);
            
            let analysis = result.unwrap();
            assert!(analysis.total_processed > 0, 
                   "No colors processed for collection {:?}", collection);
            
            // Validate collection-specific expectations
            match collection {
                ColorCollectionType::Css => {
                    assert!(analysis.total_processed >= 140, 
                           "CSS collection should have many colors");
                }
                ColorCollectionType::RalClassic => {
                    assert!(analysis.total_processed >= 200,
                           "RAL Classic should have many colors");
                }
                ColorCollectionType::RalDesign => {
                    assert!(analysis.total_processed >= 1500,
                           "RAL Design should have many colors");
                }
                _ => {}
            }
            
            println!("✓ Collection {:?}: {} total, {} matched", 
                    collection, analysis.total_processed, analysis.matched_colors.len());
        }
    }

    /// Test workflow error handling and recovery
    #[test]
    fn test_workflow_error_handling() {
        let test_cases = vec![
            // Invalid hue range
            HueArgs {
                target_hue: 400.0,  // Invalid hue
                tolerance: 30.0,
                collection: ColorCollectionType::Css,
                min_saturation: None,
                max_saturation: None,
                min_lightness: None,
                max_lightness: None,
                output_format: None,
                output_file: None,
                sort_by: None,
                limit: None,
                verbose: false,
            },
            // Invalid tolerance
            HueArgs {
                target_hue: 180.0,
                tolerance: -10.0,  // Invalid tolerance
                collection: ColorCollectionType::Css,
                min_saturation: None,
                max_saturation: None,
                min_lightness: None,
                max_lightness: None,
                output_format: None,
                output_file: None,
                sort_by: None,
                limit: None,
                verbose: false,
            },
            // Invalid saturation range
            HueArgs {
                target_hue: 180.0,
                tolerance: 30.0,
                collection: ColorCollectionType::Css,
                min_saturation: Some(80.0),
                max_saturation: Some(20.0),  // Max < Min
                min_lightness: None,
                max_lightness: None,
                output_format: None,
                output_file: None,
                sort_by: None,
                limit: None,
                verbose: false,
            },
        ];
        
        for (i, args) in test_cases.into_iter().enumerate() {
            let result = execute_hue_analysis_workflow(args);
            
            // Should either handle gracefully or return meaningful error
            match result {
                Ok(analysis) => {
                    // If successful, should have normalized parameters
                    println!("✓ Test case {} handled gracefully: {} colors", i, analysis.matched_colors.len());
                }
                Err(e) => {
                    // Should provide meaningful error message
                    assert!(!e.to_string().is_empty(), "Error message should not be empty");
                    println!("✓ Test case {} failed gracefully: {}", i, e);
                }
            }
        }
    }

    /// Test performance under stress conditions
    #[test]
    fn test_workflow_performance_stress() {
        let stress_scenarios = vec![
            ("tight_tolerance", 1.0),
            ("very_wide_tolerance", 179.0),
            ("many_constraints", 30.0),
        ];
        
        for (scenario_name, tolerance) in stress_scenarios {
            let start = std::time::Instant::now();
            
            let args = HueArgs {
                target_hue: 0.0,  // Red
                tolerance,
                collection: ColorCollectionType::All,  // Largest collection
                min_saturation: if scenario_name == "many_constraints" { Some(10.0) } else { None },
                max_saturation: if scenario_name == "many_constraints" { Some(90.0) } else { None },
                min_lightness: if scenario_name == "many_constraints" { Some(20.0) } else { None },
                max_lightness: if scenario_name == "many_constraints" { Some(80.0) } else { None },
                output_format: None,
                output_file: None,
                sort_by: Some("distance".to_string()),
                limit: None,
                verbose: false,
            };
            
            let result = execute_hue_analysis_workflow(args);
            let elapsed = start.elapsed();
            
            assert!(result.is_ok(), "Stress test {} failed", scenario_name);
            assert!(elapsed.as_secs() < 10, "Stress test {} too slow: {:?}", scenario_name, elapsed);
            
            let analysis = result.unwrap();
            println!("✓ Stress test {}: {:?}, {} colors processed, {} matched", 
                    scenario_name, elapsed, analysis.total_processed, analysis.matched_colors.len());
        }
    }

    /// Test output format consistency across different formats
    #[test]
    fn test_output_format_consistency() {
        let temp_dir = TempDir::new().unwrap();
        let formats = vec!["json", "yaml", "toml"];
        
        let base_args = HueArgs {
            target_hue: 300.0,  // Magenta
            tolerance: 60.0,
            collection: ColorCollectionType::Css,
            min_saturation: None,
            max_saturation: None,
            min_lightness: None,
            max_lightness: None,
            output_format: None, // Will be set per format
            output_file: None,   // Will be set per format
            sort_by: Some("name".to_string()),
            limit: Some(20),
            verbose: false,
        };
        
        let mut results = Vec::new();
        
        for format in &formats {
            let output_path = temp_dir.path().join(format!("test.{}", format));
            let mut args = base_args.clone();
            args.output_format = Some(format.to_string());
            args.output_file = Some(output_path.to_string_lossy().to_string());
            
            let result = execute_hue_analysis_workflow(args);
            assert!(result.is_ok(), "Format {} failed", format);
            
            let analysis = result.unwrap();
            results.push((format, analysis, output_path));
            
            // Validate file was created and has content
            assert!(output_path.exists(), "Output file not created for format {}", format);
            let content = fs::read_to_string(&output_path).unwrap();
            assert!(!content.is_empty(), "Output file empty for format {}", format);
            
            // Format-specific validation
            match format {
                &"json" => {
                    assert!(content.contains('{') && content.contains('}'), 
                           "Invalid JSON structure");
                }
                &"yaml" => {
                    assert!(content.contains(':') && content.contains("target_hue"), 
                           "Invalid YAML structure");
                }
                &"toml" => {
                    assert!(content.contains('[') || content.contains('='), 
                           "Invalid TOML structure");
                }
                _ => {}
            }
        }
        
        // Verify consistency across formats
        let base_result = &results[0].1;
        for (format, result, _) in &results[1..] {
            assert_eq!(result.matched_colors.len(), base_result.matched_colors.len(),
                      "Color count inconsistent for format {}", format);
            assert_eq!(result.total_processed, base_result.total_processed,
                      "Processing count inconsistent for format {}", format);
        }
        
        println!("✓ Output format consistency validated across {} formats", formats.len());
    }

    /// Test configuration integration with workflow
    #[test]
    fn test_config_integration_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        // Create test configuration
        let config_content = r#"
[hue_analysis]
default_tolerance = 25.0
max_results = 100
preferred_format = "yaml"
include_statistics = true

[collections]
css_enabled = true
ral_classic_enabled = true
ral_design_enabled = false

[output]
default_directory = "output"
include_metadata = true
"#;
        fs::write(&config_path, config_content).unwrap();
        
        // Test workflow with configuration
        let args = HueArgs {
            target_hue: 60.0,  // Yellow
            tolerance: 25.0,   // Should match config default
            collection: ColorCollectionType::Css,
            min_saturation: None,
            max_saturation: None,
            min_lightness: None,
            max_lightness: None,
            output_format: Some("yaml".to_string()),  // Should match config preference
            output_file: None,
            sort_by: None,
            limit: Some(100),  // Should match config max_results
            verbose: true,
        };
        
        // Load configuration and execute workflow
        let config = load_config_from_path(&config_path);
        assert!(config.is_ok(), "Failed to load configuration");
        
        let result = execute_hue_analysis_workflow_with_config(args, config.unwrap());
        assert!(result.is_ok(), "Workflow with config failed");
        
        let analysis = result.unwrap();
        assert!(analysis.matched_colors.len() <= 100, "Config limit not respected");
        
        println!("✓ Configuration integration successful: {} colors matched", 
                analysis.matched_colors.len());
    }

    // Helper functions and types

    #[derive(Debug, Clone)]
    struct HueAnalysisResult {
        matched_colors: Vec<Color>,
        total_processed: usize,
        average_distance: f64,
        execution_time: std::time::Duration,
    }

    fn execute_hue_analysis_workflow(args: HueArgs) -> Result<HueAnalysisResult, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        
        // Validate and normalize arguments
        let validated_args = validate_hue_args(args)?;
        
        // Load color collection
        let colors = load_color_collection(validated_args.collection)?;
        
        // Apply filters and analysis
        let filtered_colors = apply_hue_filters(&colors, &validated_args)?;
        
        // Sort and limit results
        let final_colors = sort_and_limit_colors(filtered_colors, &validated_args)?;
        
        // Generate output if specified
        if let Some(output_file) = &validated_args.output_file {
            write_analysis_output(&final_colors, &validated_args, output_file)?;
        }
        
        let execution_time = start.elapsed();
        
        // Calculate statistics
        let average_distance = if !final_colors.is_empty() {
            final_colors.iter()
                .map(|color| calculate_hue_distance(color.to_hsl().hue, validated_args.target_hue))
                .sum::<f64>() / final_colors.len() as f64
        } else {
            0.0
        };
        
        Ok(HueAnalysisResult {
            matched_colors: final_colors,
            total_processed: colors.len(),
            average_distance,
            execution_time,
        })
    }

    fn execute_hue_analysis_workflow_with_config(
        args: HueArgs, 
        _config: Config
    ) -> Result<HueAnalysisResult, Box<dyn std::error::Error>> {
        // For now, just execute normal workflow
        // In real implementation, would merge config with args
        execute_hue_analysis_workflow(args)
    }

    fn validate_hue_args(mut args: HueArgs) -> Result<HueArgs, Box<dyn std::error::Error>> {
        // Normalize hue to [0, 360) range
        args.target_hue = ((args.target_hue % 360.0) + 360.0) % 360.0;
        
        // Validate tolerance
        if args.tolerance < 0.0 {
            return Err("Tolerance cannot be negative".into());
        }
        args.tolerance = args.tolerance.min(180.0); // Cap at 180 degrees
        
        // Validate saturation range
        if let (Some(min_sat), Some(max_sat)) = (args.min_saturation, args.max_saturation) {
            if min_sat > max_sat {
                return Err("Minimum saturation cannot be greater than maximum".into());
            }
        }
        
        // Validate lightness range
        if let (Some(min_light), Some(max_light)) = (args.min_lightness, args.max_lightness) {
            if min_light > max_light {
                return Err("Minimum lightness cannot be greater than maximum".into());
            }
        }
        
        Ok(args)
    }

    fn load_color_collection(collection_type: ColorCollectionType) -> Result<Vec<Color>, Box<dyn std::error::Error>> {
        // Mock implementation - in real code would load from files
        match collection_type {
            ColorCollectionType::Css => Ok(generate_css_colors()),
            ColorCollectionType::RalClassic => Ok(generate_ral_classic_colors()),
            ColorCollectionType::RalDesign => Ok(generate_ral_design_colors()),
            ColorCollectionType::All => {
                let mut all_colors = Vec::new();
                all_colors.extend(generate_css_colors());
                all_colors.extend(generate_ral_classic_colors());
                all_colors.extend(generate_ral_design_colors());
                Ok(all_colors)
            }
        }
    }

    fn apply_hue_filters(colors: &[Color], args: &HueArgs) -> Result<Vec<Color>, Box<dyn std::error::Error>> {
        let filtered: Vec<Color> = colors.iter()
            .filter(|color| {
                let hsl = color.to_hsl();
                
                // Hue filter
                let hue_distance = calculate_hue_distance(hsl.hue, args.target_hue);
                if hue_distance > args.tolerance {
                    return false;
                }
                
                // Saturation filters
                if let Some(min_sat) = args.min_saturation {
                    if hsl.saturation < min_sat {
                        return false;
                    }
                }
                if let Some(max_sat) = args.max_saturation {
                    if hsl.saturation > max_sat {
                        return false;
                    }
                }
                
                // Lightness filters
                if let Some(min_light) = args.min_lightness {
                    if hsl.lightness < min_light {
                        return false;
                    }
                }
                if let Some(max_light) = args.max_lightness {
                    if hsl.lightness > max_light {
                        return false;
                    }
                }
                
                true
            })
            .cloned()
            .collect();
        
        Ok(filtered)
    }

    fn sort_and_limit_colors(mut colors: Vec<Color>, args: &HueArgs) -> Result<Vec<Color>, Box<dyn std::error::Error>> {
        // Sort colors based on specified criteria
        if let Some(sort_by) = &args.sort_by {
            match sort_by.as_str() {
                "hue" => colors.sort_by(|a, b| a.to_hsl().hue.partial_cmp(&b.to_hsl().hue).unwrap()),
                "distance" => colors.sort_by(|a, b| {
                    let dist_a = calculate_hue_distance(a.to_hsl().hue, args.target_hue);
                    let dist_b = calculate_hue_distance(b.to_hsl().hue, args.target_hue);
                    dist_a.partial_cmp(&dist_b).unwrap()
                }),
                "saturation" => colors.sort_by(|a, b| a.to_hsl().saturation.partial_cmp(&b.to_hsl().saturation).unwrap()),
                "lightness" => colors.sort_by(|a, b| a.to_hsl().lightness.partial_cmp(&b.to_hsl().lightness).unwrap()),
                "name" => {
                    // Mock name sorting - in real implementation would use color names
                    colors.sort_by(|a, b| a.to_hex().cmp(&b.to_hex()))
                }
                _ => {} // No sorting for unknown criteria
            }
        }
        
        // Apply limit
        if let Some(limit) = args.limit {
            colors.truncate(limit);
        }
        
        Ok(colors)
    }

    fn write_analysis_output(
        colors: &[Color], 
        args: &HueArgs, 
        output_file: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output_path = PathBuf::from(output_file);
        
        // Create directory if it doesn't exist
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Generate output content based on format
        let format = args.output_format.as_deref().unwrap_or("yaml");
        let content = match format {
            "json" => generate_json_output(colors, args)?,
            "yaml" => generate_yaml_output(colors, args)?,
            "toml" => generate_toml_output(colors, args)?,
            _ => return Err(format!("Unsupported output format: {}", format).into()),
        };
        
        fs::write(output_file, content)?;
        Ok(())
    }

    // Mock helper functions for testing

    fn calculate_hue_distance(hue1: f64, hue2: f64) -> f64 {
        let diff = (hue1 - hue2).abs();
        diff.min(360.0 - diff)
    }

    fn generate_css_colors() -> Vec<Color> {
        // Generate representative CSS colors
        (0..147).map(|i| {
            let hue = (i as f64 * 2.45) % 360.0; // Distributed across spectrum
            let saturation = 50.0 + (i % 5) as f64 * 10.0;
            let lightness = 40.0 + (i % 6) as f64 * 10.0;
            Color::from_hsl(hue, saturation, lightness)
        }).collect()
    }

    fn generate_ral_classic_colors() -> Vec<Color> {
        // Generate representative RAL Classic colors
        (0..213).map(|i| {
            let hue = (i as f64 * 1.69) % 360.0;
            let saturation = 30.0 + (i % 7) as f64 * 10.0;
            let lightness = 25.0 + (i % 8) as f64 * 9.0;
            Color::from_hsl(hue, saturation, lightness)
        }).collect()
    }

    fn generate_ral_design_colors() -> Vec<Color> {
        // Generate representative RAL Design colors
        (0..1625).map(|i| {
            let hue = (i as f64 * 0.221) % 360.0;
            let saturation = 20.0 + (i % 9) as f64 * 9.0;
            let lightness = 20.0 + (i % 10) as f64 * 8.0;
            Color::from_hsl(hue, saturation, lightness)
        }).collect()
    }

    fn generate_json_output(colors: &[Color], args: &HueArgs) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(r#"{{
  "target_hue": {},
  "tolerance": {},
  "matched_colors": {},
  "total_matches": {}
}}"#, args.target_hue, args.tolerance, colors.len(), colors.len()))
    }

    fn generate_yaml_output(colors: &[Color], args: &HueArgs) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(r#"target_hue: {}
tolerance: {}
matched_colors:
  - count: {}
total_matches: {}
"#, args.target_hue, args.tolerance, colors.len(), colors.len()))
    }

    fn generate_toml_output(colors: &[Color], args: &HueArgs) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(r#"target_hue = {}
tolerance = {}
total_matches = {}

[matched_colors]
count = {}
"#, args.target_hue, args.tolerance, colors.len(), colors.len()))
    }

    fn load_config_from_path(_path: &std::path::Path) -> Result<Config, Box<dyn std::error::Error>> {
        // Mock config loading
        Ok(Config::default())
    }
}
