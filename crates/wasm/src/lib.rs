use thinkeloquent_tools_chunking_directory_mapping_core::{
    ScanOptions, DirectoryScanner, OutputFormat, OutputFormatter,
    ContentAnalyzer, ScanResult, FileEntry
};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Enhanced options wrapper for WASM
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WasmScanOptions {
    pub mapper_profile: Option<String>,
    pub enhanced_analysis: Option<bool>,
    pub output_format: Option<String>, // "basic", "compact", "detailed", "hierarchical"
    pub max_depth: Option<usize>,
    pub include_hidden: Option<bool>,
    pub follow_symlinks: Option<bool>,
    pub ignore_patterns: Option<Vec<String>>,
}

impl Default for WasmScanOptions {
    fn default() -> Self {
        Self {
            mapper_profile: None,
            enhanced_analysis: None,
            output_format: None,
            max_depth: None,
            include_hidden: None,
            follow_symlinks: None,
            ignore_patterns: None,
        }
    }
}

impl From<WasmScanOptions> for ScanOptions {
    fn from(wasm_opts: WasmScanOptions) -> Self {
        let mut opts = ScanOptions::default();
        
        if let Some(profile) = wasm_opts.mapper_profile {
            opts.mapper_profile = profile;
        }
        
        if let Some(enhanced) = wasm_opts.enhanced_analysis {
            opts.enhanced_analysis = enhanced;
        }
        
        if let Some(format_str) = wasm_opts.output_format {
            opts.output_format = match format_str.as_str() {
                "basic" => OutputFormat::Basic,
                "compact" => OutputFormat::Compact,
                "detailed" => OutputFormat::Detailed,
                "hierarchical" => OutputFormat::Hierarchical,
                _ => OutputFormat::Basic,
            };
        }
        
        if let Some(depth) = wasm_opts.max_depth {
            opts.max_depth = Some(depth);
        }
        
        if let Some(hidden) = wasm_opts.include_hidden {
            opts.include_hidden = hidden;
        }
        
        if let Some(symlinks) = wasm_opts.follow_symlinks {
            opts.follow_symlinks = symlinks;
        }
        
        if let Some(patterns) = wasm_opts.ignore_patterns {
            opts.ignore_patterns = patterns;
        }
        
        opts
    }
}

// Enhanced scan result wrapper
#[derive(Serialize, Deserialize, Debug)]
pub struct WasmScanResult {
    pub result: ScanResult,
    pub formatted_output: Option<String>,
    pub json_output: Option<String>,
    pub yaml_output: Option<String>,
}

#[wasm_bindgen]
pub fn scan_directory(path: &str, options_json: Option<String>) -> Result<JsValue, JsValue> {
    console_log!("Scanning directory: {}", path);
    
    let wasm_options = if let Some(opts_str) = options_json {
        serde_json::from_str::<WasmScanOptions>(&opts_str)
            .unwrap_or_else(|e| {
                console_log!("Failed to parse options: {}, using defaults", e);
                WasmScanOptions::default()
            })
    } else {
        WasmScanOptions::default()
    };
    
    let options: ScanOptions = wasm_options.clone().into();
    let scanner = DirectoryScanner::new(options.clone());
    
    match scanner.scan(path) {
        Ok(result) => {
            console_log!("Scan completed: {} files found", result.stats.total_files);
            
            // Create enhanced result with multiple output formats
            let mut wasm_result = WasmScanResult {
                result: result.clone(),
                formatted_output: None,
                json_output: None,
                yaml_output: None,
            };
            
            // Generate formatted output
            wasm_result.formatted_output = Some(
                OutputFormatter::format_result(&result, &options.output_format)
            );
            
            // Generate JSON output
            wasm_result.json_output = serde_json::to_string_pretty(&result)
                .map_err(|e| console_log!("JSON serialization error: {}", e))
                .ok();
            
            // Generate YAML output
            wasm_result.yaml_output = serde_yaml::to_string(&result)
                .map_err(|e| console_log!("YAML serialization error: {}", e))
                .ok();
            
            to_value(&wasm_result).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        },
        Err(e) => {
            console_log!("Scan failed: {}", e);
            Err(JsValue::from_str(&format!("Scan error: {}", e)))
        }
    }
}

#[wasm_bindgen]
pub fn scan_directory_simple(path: &str, options_json: Option<String>) -> Result<JsValue, JsValue> {
    console_log!("Simple directory scan: {}", path);
    
    let options = if let Some(opts_str) = options_json {
        serde_json::from_str::<ScanOptions>(&opts_str)
            .unwrap_or_else(|_| ScanOptions::default())
    } else {
        ScanOptions::default()
    };
    
    let scanner = DirectoryScanner::new(options);
    
    match scanner.scan(path) {
        Ok(result) => {
            console_log!("Simple scan completed: {} files found", result.stats.total_files);
            to_value(&result).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        },
        Err(e) => {
            console_log!("Simple scan failed: {}", e);
            Err(JsValue::from_str(&format!("Scan error: {}", e)))
        }
    }
}

#[wasm_bindgen]
pub fn format_scan_result(result_json: &str, format_type: &str) -> Result<String, JsValue> {
    let result: ScanResult = serde_json::from_str(result_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse result: {}", e)))?;
    
    let output_format = match format_type {
        "basic" => OutputFormat::Basic,
        "compact" => OutputFormat::Compact,
        "detailed" => OutputFormat::Detailed,
        "hierarchical" => OutputFormat::Hierarchical,
        _ => return Err(JsValue::from_str("Invalid format type. Use: basic, compact, detailed, hierarchical")),
    };
    
    Ok(OutputFormatter::format_result(&result, &output_format))
}

#[wasm_bindgen]
pub fn scan_result_to_yaml(result_json: &str) -> Result<String, JsValue> {
    let result: ScanResult = serde_json::from_str(result_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse result: {}", e)))?;
    
    serde_yaml::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("YAML serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn analyze_file_content(file_path: &str, content: &str, _language: Option<String>) -> Result<JsValue, JsValue> {
    console_log!("Analyzing file content: {}", file_path);
    
    let analyzer = ContentAnalyzer::new();
    
    // Create a basic FileEntry for analysis
    let file_entry = FileEntry {
        path: file_path.into(),
        name: std::path::Path::new(file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(file_path)
            .to_string(),
        size: content.len() as u64,
        modified: std::time::SystemTime::now(),
        is_dir: false,
        tags: vec![],
        metadata: None,
        enhanced_info: None,
    };
    
    match analyzer.analyze_file(&file_entry) {
        Ok(enhanced_info) => {
            console_log!("File analysis completed for: {}", file_path);
            to_value(&enhanced_info).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        },
        Err(e) => {
            console_log!("File analysis failed: {}", e);
            Err(JsValue::from_str(&format!("Analysis error: {}", e)))
        }
    }
}

#[wasm_bindgen]
pub fn analyze_branching_details(content: &str, language: Option<String>) -> Result<JsValue, JsValue> {
    console_log!("Analyzing branching details for {} chars of content", content.len());
    
    let analyzer = ContentAnalyzer::new();
    let details = analyzer.analyze_branching_details(content, &language);
    
    // Convert BranchingDetails to a serializable format
    let details_map: HashMap<String, serde_json::Value> = [
        ("conditional_count".to_string(), serde_json::Value::Number(details.conditional_count.into())),
        ("loop_count".to_string(), serde_json::Value::Number(details.loop_count.into())),
        ("switch_count".to_string(), serde_json::Value::Number(details.switch_count.into())),
        ("max_nesting".to_string(), serde_json::Value::Number(details.max_nesting.into())),
        ("logical_operators".to_string(), serde_json::Value::Number(details.logical_operators.into())),
        ("cyclomatic_complexity".to_string(), serde_json::json!(details.cyclomatic_complexity)),
        ("cognitive_complexity".to_string(), serde_json::json!(details.cognitive_complexity)),
        ("hardcoded_dates_count".to_string(), serde_json::Value::Number(details.hardcoded_dates_count.into())),
        ("hardcoded_values_count".to_string(), serde_json::Value::Number(details.hardcoded_values_count.into())),
        ("pure_branches".to_string(), serde_json::Value::Number(details.pure_branches.into())),
        ("non_pure_branches".to_string(), serde_json::Value::Number(details.non_pure_branches.into())),
        ("future_logic_count".to_string(), serde_json::Value::Number(details.future_logic_count.into())),
        ("past_logic_count".to_string(), serde_json::Value::Number(details.past_logic_count.into())),
        ("total_branches".to_string(), serde_json::Value::Number(details.total_branches.into())),
    ].into();
    
    console_log!("Branching analysis completed");
    to_value(&details_map).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn get_scan_statistics(result_json: &str) -> Result<JsValue, JsValue> {
    let result: ScanResult = serde_json::from_str(result_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse result: {}", e)))?;
    
    to_value(&result.stats).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn get_scan_errors(result_json: &str) -> Result<JsValue, JsValue> {
    let result: ScanResult = serde_json::from_str(result_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse result: {}", e)))?;
    
    to_value(&result.errors).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn create_scan_options(
    mapper_profile: Option<String>,
    enhanced_analysis: Option<bool>,
    output_format: Option<String>,
    max_depth: Option<usize>,
    include_hidden: Option<bool>,
    follow_symlinks: Option<bool>,
) -> Result<String, JsValue> {
    let options = WasmScanOptions {
        mapper_profile,
        enhanced_analysis,
        output_format,
        max_depth,
        include_hidden,
        follow_symlinks,
        ignore_patterns: None,
    };
    
    serde_json::to_string(&options)
        .map_err(|e| JsValue::from_str(&format!("Options serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn validate_scan_options(options_json: &str) -> Result<bool, JsValue> {
    match serde_json::from_str::<WasmScanOptions>(options_json) {
        Ok(_) => Ok(true),
        Err(e) => Err(JsValue::from_str(&format!("Invalid options: {}", e))),
    }
}

#[wasm_bindgen]
pub fn get_supported_output_formats() -> JsValue {
    let formats = vec!["basic", "compact", "detailed", "hierarchical"];
    to_value(&formats).unwrap()
}

#[wasm_bindgen]
pub fn get_supported_mapper_profiles() -> JsValue {
    let profiles = vec!["generic", "enhanced"];
    to_value(&profiles).unwrap()
}

#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn get_build_info() -> JsValue {
    let info = serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": env!("CARGO_PKG_NAME"),
        "authors": env!("CARGO_PKG_AUTHORS"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "build_date": std::env::var("VERGEN_BUILD_DATE").unwrap_or_else(|_| "unknown".to_string()),
        "git_sha": std::env::var("VERGEN_GIT_SHA").unwrap_or_else(|_| "unknown".to_string()),
    });
    
    to_value(&info).unwrap_or_else(|_| JsValue::from_str("Build info unavailable"))
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Thinkeloquent Directory Scanner WASM module loaded v{}", get_version());
    console_log!("Available functions: scan_directory, scan_directory_simple, format_scan_result, analyze_file_content, analyze_branching_details");
}