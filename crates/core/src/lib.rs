use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, Instant};
use std::fs;
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub modified: SystemTime,
    pub is_dir: bool,
    pub tags: Vec<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub enhanced_info: Option<EnhancedFileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedFileInfo {
    pub language: Option<String>,
    pub framework: Option<String>,
    pub line_count: Option<usize>,
    pub complexity_score: Option<f64>,
    pub importance_score: Option<f64>,
    pub content_summary: Option<String>,
    pub exports: Vec<String>,
    pub imports: Vec<String>,
    pub dependencies: Vec<String>,
    pub related_files: Vec<PathBuf>,
    pub purpose: Option<String>,
    pub last_author: Option<String>,
    pub change_frequency: Option<String>,
    pub api_surface: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BranchingDetails {
    pub conditional_count: usize,
    pub loop_count: usize,
    pub switch_count: usize,
    pub max_nesting: usize,
    pub logical_operators: usize,
    pub cyclomatic_complexity: f64,
    pub cognitive_complexity: f64,
    // New enhanced analysis fields
    pub hardcoded_dates_count: usize,
    pub hardcoded_values_count: usize,
    pub pure_branches: usize,
    pub non_pure_branches: usize,
    pub nesting_distribution: std::collections::HashMap<usize, usize>,
    pub future_logic_count: usize,
    pub past_logic_count: usize,
    pub total_branches: usize,
}

impl BranchingDetails {
    pub fn new() -> Self {
        Self {
            conditional_count: 0,
            loop_count: 0,
            switch_count: 0,
            max_nesting: 0,
            logical_operators: 0,
            cyclomatic_complexity: 0.0,
            cognitive_complexity: 0.0,
            hardcoded_dates_count: 0,
            hardcoded_values_count: 0,
            pure_branches: 0,
            non_pure_branches: 0,
            nesting_distribution: std::collections::HashMap::new(),
            future_logic_count: 0,
            past_logic_count: 0,
            total_branches: 0,
        }
    }
    
    pub fn detect_hardcoded_dates(&mut self, content: &str) {
        // Count ISO dates (YYYY-MM-DD format)
        let iso_dates = content.matches(char::is_numeric)
            .filter(|line| line.contains('-') && 
                (line.contains("202") || line.contains("201") || line.contains("200")))
            .count();
            
        // Count slash format dates (MM/DD/YYYY or DD/MM/YYYY)
        let slash_dates = content.lines()
            .map(|line| line.matches('/').count())
            .sum::<usize>() / 2; // Approximate: each date has 2 slashes
            
        // Count year patterns in conditionals
        let year_patterns = content.lines()
            .filter(|line| line.contains("if") && 
                (line.contains("202") || line.contains("201") || line.contains("200")))
            .count();
            
        self.hardcoded_dates_count = iso_dates + slash_dates + year_patterns;
    }
    
    pub fn detect_hardcoded_values(&mut self, content: &str) {
        let mut count = 0;
        
        // Look for magic numbers in conditionals (excluding 0, 1, powers of 2)
        for line in content.lines() {
            if line.contains("if") {
                // Simple approach: count numeric literals that aren't common values
                for word in line.split_whitespace() {
                    if let Ok(num) = word.trim_matches(|c: char| !c.is_ascii_digit() && c != '.' && c != '-').parse::<f64>() {
                        if num != 0.0 && num != 1.0 && num != 2.0 && !Self::is_power_of_two(num as u64) {
                            count += 1;
                        }
                    }
                }
            }
        }
        
        // Look for hardcoded strings in conditionals
        for line in content.lines() {
            if line.contains("if") && line.contains('"') {
                count += line.matches('"').count() / 2; // Each string has 2 quotes
            }
        }
        
        self.hardcoded_values_count = count;
    }
    
    fn is_power_of_two(n: u64) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
    
    pub fn analyze_branch_purity(&mut self, content: &str, _language: &str) {
        let mut pure_count = 0;
        let mut non_pure_count = 0;
        
        for line in content.lines() {
            if line.contains("if") {
                if Self::is_non_pure_line(line) {
                    non_pure_count += 1;
                } else {
                    pure_count += 1;
                }
            }
        }
        
        self.pure_branches = pure_count;
        self.non_pure_branches = non_pure_count;
    }
    
    fn is_non_pure_line(line: &str) -> bool {
        line.contains("fs::") || line.contains("File::") || line.contains("Path::") ||
        line.contains("SystemTime::") || line.contains("Instant::") ||
        line.contains("environment_var") || line.contains("GLOBAL_") ||
        line.contains("rand::") || line.contains(".gen_bool") || line.contains(".read(") || line.contains(".write(") ||
        line.contains("http_client") || line.contains("socket")
    }
    
    pub fn detect_future_logic(&mut self, content: &str) {
        let mut count = 0;
        
        for line in content.lines() {
            if line.contains("if") {
                // Look for future dates
                if line.contains("2025") || line.contains("2026") || line.contains("2027") {
                    count += 1;
                }
                // Look for version checks that might be future
                if line.contains(">=") && (line.contains("\"2.") || line.contains("\"3.")) {
                    count += 1;
                }
                // Look for high API level checks
                if line.contains("api_level >=") || line.contains("api_version >=") {
                    count += 1;
                }
                // Look for feature flags
                if line.contains("feature_flags") || line.contains("beta_features") {
                    count += 1;
                }
            }
        }
        
        self.future_logic_count = count;
    }
    
    pub fn detect_past_logic(&mut self, content: &str) {
        let mut count = 0;
        
        for line in content.lines() {
            if line.contains("if") {
                // Look for past dates
                if line.contains("2020") || line.contains("2021") || line.contains("2022") {
                    count += 1;
                }
                // Look for deprecated version checks
                if line.contains("<") && (line.contains("\"1.") || line.contains("\"0.")) {
                    count += 1;
                }
                // Look for old API level checks
                if line.contains("api_level <") || line.contains("api_version <") {
                    count += 1;
                }
                // Look for deprecation patterns
                if line.contains("deprecated") || line.contains("end_of_life") || line.contains("support_end") {
                    count += 1;
                }
            }
        }
        
        self.past_logic_count = count;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryNode {
    pub path: PathBuf,
    pub name: String,
    pub children: Vec<FileEntry>,
    pub subdirs: Vec<DirectoryNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_size: u64,
    pub scan_duration_ms: u64,
    pub files_per_second: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub root_path: PathBuf,
    pub files: Vec<FileEntry>,
    pub stats: ScanStats,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOptions {
    pub max_depth: Option<usize>,
    pub ignore_patterns: Vec<String>,
    pub follow_symlinks: bool,
    pub include_hidden: bool,
    pub mapper_profile: String,
    pub collect_metadata: bool,
    pub enhanced_analysis: bool,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Basic,
    Compact,
    Detailed,
    Hierarchical,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            max_depth: None,
            ignore_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(), 
                "target".to_string(),
                ".DS_Store".to_string(),
            ],
            follow_symlinks: false,
            include_hidden: false,
            mapper_profile: "generic".to_string(),
            collect_metadata: false,
            enhanced_analysis: false,
            output_format: OutputFormat::Basic,
        }
    }
}

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },
    
    #[error("Path not found: {path}")]
    PathNotFound { path: PathBuf },
    
    #[error("Scan limit exceeded: {limit}")]
    LimitExceeded { limit: usize },
    
    #[error("Invalid configuration: {message}")]
    InvalidConfig { message: String },
}

pub trait Mapper {
    fn classify(&self, entry: &FileEntry) -> Vec<String>;
    fn name(&self) -> &str;
}

pub trait EnhancedMapper {
    fn analyze(&self, entry: &mut FileEntry) -> Result<(), ScanError>;
    fn name(&self) -> &str;
}

pub struct GenericMapper;

impl Mapper for GenericMapper {
    fn classify(&self, entry: &FileEntry) -> Vec<String> {
        let mut tags = Vec::new();
        
        if entry.is_dir {
            tags.push("directory".to_string());
            return tags;
        }
        
        let path_str = entry.path.to_string_lossy().to_lowercase();
        let name_lower = entry.name.to_lowercase();
        
        // Documentation files
        if name_lower.starts_with("readme") || 
           name_lower.ends_with(".md") || 
           name_lower.ends_with(".txt") ||
           name_lower.ends_with(".rst") {
            tags.push("documentation".to_string());
        }
        
        // Configuration files
        if name_lower.ends_with(".json") ||
           name_lower.ends_with(".yaml") ||
           name_lower.ends_with(".yml") ||
           name_lower.ends_with(".toml") ||
           name_lower.ends_with(".ini") ||
           name_lower.ends_with(".cfg") {
            tags.push("configuration".to_string());
        }
        
        // Script files
        if name_lower.ends_with(".sh") ||
           name_lower.ends_with(".bash") ||
           name_lower.ends_with(".zsh") ||
           name_lower.ends_with(".fish") ||
           name_lower.ends_with(".ps1") ||
           name_lower.ends_with(".bat") ||
           name_lower.ends_with(".cmd") {
            tags.push("script".to_string());
        }
        
        // Source code
        if name_lower.ends_with(".rs") ||
           name_lower.ends_with(".py") ||
           name_lower.ends_with(".js") ||
           name_lower.ends_with(".ts") ||
           name_lower.ends_with(".jsx") ||
           name_lower.ends_with(".tsx") ||
           name_lower.ends_with(".go") ||
           name_lower.ends_with(".java") ||
           name_lower.ends_with(".c") ||
           name_lower.ends_with(".cpp") ||
           name_lower.ends_with(".h") ||
           name_lower.ends_with(".hpp") {
            tags.push("source".to_string());
        }
        
        // Test files
        if path_str.contains("test") || 
           path_str.contains("spec") ||
           name_lower.contains("test") ||
           name_lower.contains("spec") {
            tags.push("test".to_string());
        }
        
        // Examples
        if path_str.contains("example") ||
           path_str.contains("demo") ||
           name_lower.contains("example") ||
           name_lower.contains("demo") {
            tags.push("example".to_string());
        }
        
        if tags.is_empty() {
            tags.push("unclassified".to_string());
        }
        
        tags
    }
    
    fn name(&self) -> &str {
        "generic"
    }
}

pub struct ContentAnalyzer;

impl ContentAnalyzer {
    pub fn new() -> Self {
        ContentAnalyzer
    }
    
    pub fn analyze_file(&self, entry: &FileEntry) -> Result<EnhancedFileInfo, ScanError> {
        if entry.is_dir {
            return Ok(EnhancedFileInfo::default());
        }

        let mut enhanced_info = EnhancedFileInfo::default();
        
        // Detect language based on extension
        enhanced_info.language = self.detect_language(&entry.name);
        
        // Try to read file content for analysis
        if let Ok(content) = fs::read_to_string(&entry.path) {
            enhanced_info.line_count = Some(content.lines().count());
            enhanced_info.complexity_score = Some(self.calculate_complexity(&content, &enhanced_info.language));
            enhanced_info.content_summary = Some(self.generate_summary(&content, &enhanced_info.language));
            enhanced_info.exports = self.extract_exports(&content, &enhanced_info.language);
            enhanced_info.imports = self.extract_imports(&content, &enhanced_info.language);
            enhanced_info.api_surface = self.extract_api_surface(&content, &enhanced_info.language);
            enhanced_info.purpose = Some(self.infer_purpose(&entry.path, &content, &enhanced_info.language));
        }
        
        // Calculate importance based on various factors
        enhanced_info.importance_score = Some(self.calculate_importance(entry, &enhanced_info));
        
        Ok(enhanced_info)
    }
    
    fn detect_language(&self, filename: &str) -> Option<String> {
        let extension = std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
            
        match extension {
            "rs" => Some("rust".to_string()),
            "py" => Some("python".to_string()),
            "js" => Some("javascript".to_string()),
            "ts" => Some("typescript".to_string()),
            "jsx" => Some("javascript".to_string()),
            "tsx" => Some("typescript".to_string()),
            "go" => Some("go".to_string()),
            "java" => Some("java".to_string()),
            "c" => Some("c".to_string()),
            "cpp" | "cxx" | "cc" => Some("cpp".to_string()),
            "h" | "hpp" => Some("c".to_string()),
            "md" => Some("markdown".to_string()),
            "json" => Some("json".to_string()),
            "yaml" | "yml" => Some("yaml".to_string()),
            "toml" => Some("toml".to_string()),
            "sh" | "bash" => Some("shell".to_string()),
            _ => None,
        }
    }
    
    fn calculate_complexity(&self, content: &str, language: &Option<String>) -> f64 {
        let lines = content.lines().count() as f64;
        let chars = content.len() as f64;
        
        // Base complexity on file size and content patterns
        let mut complexity = (lines / 100.0) + (chars / 10000.0);
        
        // Calculate branching complexity (cyclomatic + nesting)
        let branching_complexity = self.calculate_branching_complexity(content, language);
        complexity += branching_complexity;
        
        if let Some(lang) = language {
            match lang.as_str() {
                "rust" | "cpp" => {
                    // Count complex patterns
                    complexity += (content.matches("impl ").count() as f64) * 0.5;
                    complexity += (content.matches("trait ").count() as f64) * 0.3;
                    complexity += (content.matches("struct ").count() as f64) * 0.2;
                }
                "javascript" | "typescript" => {
                    complexity += (content.matches("class ").count() as f64) * 0.4;
                    complexity += (content.matches("function ").count() as f64) * 0.3;
                    complexity += (content.matches("async ").count() as f64) * 0.2;
                }
                _ => {}
            }
        }
        
        complexity.min(10.0) // Cap at 10
    }
    
    fn calculate_branching_complexity(&self, content: &str, language: &Option<String>) -> f64 {
        let mut cyclomatic_complexity = 1.0; // Base complexity
        let mut cognitive_complexity = 0.0;
        
        let lines: Vec<&str> = content.lines().collect();
        let mut nesting_level = 0;
        let mut max_nesting = 0;
        
        for line in &lines {
            let trimmed = line.trim();
            
            // Update nesting level based on indentation and braces
            if trimmed.contains('{') {
                nesting_level += 1;
                max_nesting = max_nesting.max(nesting_level);
            }
            if trimmed.contains('}') && nesting_level > 0 {
                nesting_level -= 1;
            }
            
            // Language-specific conditional detection
            if let Some(lang) = language {
                match lang.as_str() {
                    "rust" => {
                        cyclomatic_complexity += self.count_rust_conditionals(trimmed);
                        cognitive_complexity += self.calculate_rust_cognitive_complexity(trimmed, nesting_level);
                    }
                    "javascript" | "typescript" => {
                        cyclomatic_complexity += self.count_js_conditionals(trimmed);
                        cognitive_complexity += self.calculate_js_cognitive_complexity(trimmed, nesting_level);
                    }
                    "python" => {
                        cyclomatic_complexity += self.count_python_conditionals(trimmed);
                        cognitive_complexity += self.calculate_python_cognitive_complexity(trimmed, nesting_level);
                    }
                    "java" => {
                        cyclomatic_complexity += self.count_java_conditionals(trimmed);
                        cognitive_complexity += self.calculate_java_cognitive_complexity(trimmed, nesting_level);
                    }
                    "go" => {
                        cyclomatic_complexity += self.count_go_conditionals(trimmed);
                        cognitive_complexity += self.calculate_go_cognitive_complexity(trimmed, nesting_level);
                    }
                    "c" | "cpp" => {
                        cyclomatic_complexity += self.count_c_conditionals(trimmed);
                        cognitive_complexity += self.calculate_c_cognitive_complexity(trimmed, nesting_level);
                    }
                    _ => {
                        cyclomatic_complexity += self.count_generic_conditionals(trimmed);
                        cognitive_complexity += self.calculate_generic_cognitive_complexity(trimmed, nesting_level);
                    }
                }
            } else {
                cyclomatic_complexity += self.count_generic_conditionals(trimmed);
                cognitive_complexity += self.calculate_generic_cognitive_complexity(trimmed, nesting_level);
            }
        }
        
        // Apply nesting penalty - deeply nested code is harder to understand
        let nesting_penalty = (max_nesting as f64).powf(1.5) * 0.2;
        
        // Combine all complexity measures with different weights
        let total_branching_complexity = 
            (cyclomatic_complexity * 0.4) + 
            (cognitive_complexity * 0.4) + 
            (nesting_penalty * 0.2);
        
        total_branching_complexity.min(8.0) // Cap branching complexity at 8
    }
    
    fn count_rust_conditionals(&self, line: &str) -> f64 {
        let mut complexity = 0.0;
        
        // Basic conditionals
        if line.contains(" if ") || line.starts_with("if ") {
            complexity += 1.0;
        }
        if line.contains(" match ") || line.starts_with("match ") {
            complexity += 1.0; // match itself adds 1
        }
        if line.contains(" while ") || line.starts_with("while ") {
            complexity += 1.0;
        }
        if line.contains(" for ") || line.starts_with("for ") {
            complexity += 1.0;
        }
        
        // Match arms - each arm is a decision point
        if line.contains("=>") && !line.contains("//") {
            complexity += 1.0;
        }
        
        // Logical operators (short-circuit evaluation)
        complexity += (line.matches(" && ").count() as f64) * 0.5;
        complexity += (line.matches(" || ").count() as f64) * 0.5;
        
        // Rust-specific patterns
        if line.contains("?") && (line.contains("Ok(") || line.contains("Some(") || line.contains("None") || line.contains("Err(")) {
            complexity += 0.3; // ? operator adds some complexity
        }
        
        complexity
    }
    
    fn count_js_conditionals(&self, line: &str) -> f64 {
        let mut complexity = 0.0;
        
        // Basic conditionals
        if line.contains(" if ") || line.starts_with("if ") {
            complexity += 1.0;
        }
        if line.contains(" switch ") || line.starts_with("switch ") {
            complexity += 1.0;
        }
        if line.contains(" while ") || line.starts_with("while ") {
            complexity += 1.0;
        }
        if line.contains(" for ") || line.starts_with("for ") {
            complexity += 1.0;
        }
        
        // Switch cases
        if line.trim().starts_with("case ") {
            complexity += 1.0;
        }
        
        // Ternary operator
        if line.contains(" ? ") && line.contains(" : ") {
            complexity += 1.0;
        }
        
        // Logical operators
        complexity += (line.matches(" && ").count() as f64) * 0.5;
        complexity += (line.matches(" || ").count() as f64) * 0.5;
        
        // Try-catch
        if line.contains(" catch ") || line.starts_with("catch ") {
            complexity += 1.0;
        }
        
        complexity
    }
    
    fn count_python_conditionals(&self, line: &str) -> f64 {
        let mut complexity = 0.0;
        
        // Basic conditionals
        if line.contains(" if ") || line.starts_with("if ") || line.ends_with(" if") {
            complexity += 1.0;
        }
        if line.contains(" while ") || line.starts_with("while ") {
            complexity += 1.0;
        }
        if line.contains(" for ") || line.starts_with("for ") {
            complexity += 1.0;
        }
        
        // Exception handling
        if line.contains("except ") || line.starts_with("except ") {
            complexity += 1.0;
        }
        
        // Logical operators
        complexity += (line.matches(" and ").count() as f64) * 0.5;
        complexity += (line.matches(" or ").count() as f64) * 0.5;
        
        // List comprehensions with conditions
        if line.contains("[") && line.contains(" if ") && line.contains("]") {
            complexity += 0.5;
        }
        
        complexity
    }
    
    fn count_java_conditionals(&self, line: &str) -> f64 {
        let mut complexity = 0.0;
        
        // Basic conditionals
        if line.contains(" if ") || line.starts_with("if ") {
            complexity += 1.0;
        }
        if line.contains(" switch ") || line.starts_with("switch ") {
            complexity += 1.0;
        }
        if line.contains(" while ") || line.starts_with("while ") {
            complexity += 1.0;
        }
        if line.contains(" for ") || line.starts_with("for ") {
            complexity += 1.0;
        }
        
        // Switch cases
        if line.trim().starts_with("case ") {
            complexity += 1.0;
        }
        
        // Exception handling
        if line.contains(" catch ") || line.starts_with("catch ") {
            complexity += 1.0;
        }
        
        // Logical operators
        complexity += (line.matches(" && ").count() as f64) * 0.5;
        complexity += (line.matches(" || ").count() as f64) * 0.5;
        
        complexity
    }
    
    fn count_go_conditionals(&self, line: &str) -> f64 {
        let mut complexity = 0.0;
        
        // Basic conditionals
        if line.contains(" if ") || line.starts_with("if ") {
            complexity += 1.0;
        }
        if line.contains(" switch ") || line.starts_with("switch ") {
            complexity += 1.0;
        }
        if line.contains(" for ") || line.starts_with("for ") {
            complexity += 1.0;
        }
        
        // Switch cases
        if line.trim().starts_with("case ") {
            complexity += 1.0;
        }
        
        // Select cases (Go-specific)
        if line.contains(" select ") || line.starts_with("select ") {
            complexity += 1.0;
        }
        
        // Logical operators
        complexity += (line.matches(" && ").count() as f64) * 0.5;
        complexity += (line.matches(" || ").count() as f64) * 0.5;
        
        complexity
    }
    
    fn count_c_conditionals(&self, line: &str) -> f64 {
        let mut complexity = 0.0;
        
        // Basic conditionals
        if line.contains(" if ") || line.starts_with("if ") {
            complexity += 1.0;
        }
        if line.contains(" switch ") || line.starts_with("switch ") {
            complexity += 1.0;
        }
        if line.contains(" while ") || line.starts_with("while ") {
            complexity += 1.0;
        }
        if line.contains(" for ") || line.starts_with("for ") {
            complexity += 1.0;
        }
        
        // Switch cases
        if line.trim().starts_with("case ") {
            complexity += 1.0;
        }
        
        // Ternary operator
        if line.contains(" ? ") && line.contains(" : ") {
            complexity += 1.0;
        }
        
        // Logical operators
        complexity += (line.matches(" && ").count() as f64) * 0.5;
        complexity += (line.matches(" || ").count() as f64) * 0.5;
        
        complexity
    }
    
    fn count_generic_conditionals(&self, line: &str) -> f64 {
        let mut complexity = 0.0;
        
        // Generic conditional patterns
        if line.contains(" if ") || line.starts_with("if ") {
            complexity += 1.0;
        }
        if line.contains(" while ") || line.starts_with("while ") {
            complexity += 1.0;
        }
        if line.contains(" for ") || line.starts_with("for ") {
            complexity += 1.0;
        }
        if line.contains(" switch ") || line.starts_with("switch ") {
            complexity += 1.0;
        }
        
        // Generic logical operators
        complexity += (line.matches(" && ").count() as f64) * 0.5;
        complexity += (line.matches(" || ").count() as f64) * 0.5;
        complexity += (line.matches(" and ").count() as f64) * 0.5;
        complexity += (line.matches(" or ").count() as f64) * 0.5;
        
        complexity
    }
    
    fn calculate_rust_cognitive_complexity(&self, line: &str, nesting_level: usize) -> f64 {
        let mut cognitive_score = 0.0;
        let nesting_multiplier = 1.0 + (nesting_level as f64 * 0.5);
        
        if line.contains(" if ") || line.starts_with("if ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" match ") || line.starts_with("match ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" while ") || line.contains(" loop ") || line.contains(" for ") {
            cognitive_score += 1.5 * nesting_multiplier; // Loops are slightly more complex
        }
        
        cognitive_score
    }
    
    fn calculate_js_cognitive_complexity(&self, line: &str, nesting_level: usize) -> f64 {
        let mut cognitive_score = 0.0;
        let nesting_multiplier = 1.0 + (nesting_level as f64 * 0.5);
        
        if line.contains(" if ") || line.starts_with("if ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" switch ") || line.starts_with("switch ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" while ") || line.contains(" for ") {
            cognitive_score += 1.5 * nesting_multiplier;
        }
        
        cognitive_score
    }
    
    fn calculate_python_cognitive_complexity(&self, line: &str, nesting_level: usize) -> f64 {
        let mut cognitive_score = 0.0;
        let nesting_multiplier = 1.0 + (nesting_level as f64 * 0.5);
        
        if line.contains(" if ") || line.starts_with("if ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" while ") || line.contains(" for ") {
            cognitive_score += 1.5 * nesting_multiplier;
        }
        
        cognitive_score
    }
    
    fn calculate_java_cognitive_complexity(&self, line: &str, nesting_level: usize) -> f64 {
        let mut cognitive_score = 0.0;
        let nesting_multiplier = 1.0 + (nesting_level as f64 * 0.5);
        
        if line.contains(" if ") || line.starts_with("if ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" switch ") || line.starts_with("switch ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" while ") || line.contains(" for ") {
            cognitive_score += 1.5 * nesting_multiplier;
        }
        
        cognitive_score
    }
    
    fn calculate_go_cognitive_complexity(&self, line: &str, nesting_level: usize) -> f64 {
        let mut cognitive_score = 0.0;
        let nesting_multiplier = 1.0 + (nesting_level as f64 * 0.5);
        
        if line.contains(" if ") || line.starts_with("if ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" switch ") || line.contains(" select ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" for ") {
            cognitive_score += 1.5 * nesting_multiplier;
        }
        
        cognitive_score
    }
    
    fn calculate_c_cognitive_complexity(&self, line: &str, nesting_level: usize) -> f64 {
        let mut cognitive_score = 0.0;
        let nesting_multiplier = 1.0 + (nesting_level as f64 * 0.5);
        
        if line.contains(" if ") || line.starts_with("if ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" switch ") || line.starts_with("switch ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" while ") || line.contains(" for ") {
            cognitive_score += 1.5 * nesting_multiplier;
        }
        
        cognitive_score
    }
    
    fn calculate_generic_cognitive_complexity(&self, line: &str, nesting_level: usize) -> f64 {
        let mut cognitive_score = 0.0;
        let nesting_multiplier = 1.0 + (nesting_level as f64 * 0.5);
        
        if line.contains(" if ") || line.starts_with("if ") {
            cognitive_score += 1.0 * nesting_multiplier;
        }
        if line.contains(" while ") || line.contains(" for ") {
            cognitive_score += 1.5 * nesting_multiplier;
        }
        
        cognitive_score
    }
    
    fn detect_hardcoded_dates(&self, line: &str) -> bool {
        // Simple date patterns using string matching
        if line.matches("-").count() >= 2 && (
            line.contains("2019") || line.contains("2020") || line.contains("2021") ||
            line.contains("2022") || line.contains("2023") || line.contains("2024") ||
            line.contains("2025") || line.contains("2026") || line.contains("2027")
        ) {
            return true;
        }
        
        // Common date separators
        if line.matches("/").count() >= 2 && (
            line.contains("2019") || line.contains("2020") || line.contains("2021") ||
            line.contains("2022") || line.contains("2023") || line.contains("2024") ||
            line.contains("2025") || line.contains("2026") || line.contains("2027")
        ) {
            return true;
        }
        
        // Year patterns in conditionals
        if line.contains(" if ") || line.contains("==") || line.contains("!=") || 
            line.contains(">") || line.contains("<") {
            for year in 1990..=2030 {
                if line.contains(&year.to_string()) {
                    return true;
                }
            }
        }
        
        // Common timestamp patterns (starts with 1 and has many digits)
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            if word.starts_with('1') && word.len() >= 10 && word.chars().all(|c| c.is_ascii_digit()) {
                return true;
            }
        }
        
        false
    }
    
    fn count_hardcoded_values(&self, line: &str) -> usize {
        let mut count = 0;
        
        // Magic numbers in conditionals (excluding common values like 0, 1, -1)
        if line.contains(" if ") || line.contains("==") || line.contains("!=") || 
            line.contains(">") || line.contains("<") {
            
            // Common powers of 2 and small numbers to exclude
            let common_numbers = ["0", "1", "2", "4", "8", "16", "32", "64", "128", "256", "512", "1024", "-1"];
            
            // Look for numeric literals (including floats)
            let words: Vec<&str> = line.split_whitespace().collect();
            for word in words {
                let clean_word = word.trim_matches(|c| !char::is_ascii_digit(&c) && c != '-' && c != '.');
                if !clean_word.is_empty() {
                    // Handle both integers and floats
                    if clean_word.chars().all(|c| c.is_ascii_digit() || c == '-' || c == '.') {
                        if clean_word.len() >= 2 && !common_numbers.contains(&clean_word) {
                            // Try parsing as float first, then integer
                            if let Ok(_) = clean_word.parse::<f64>() {
                                // Check if it's not a year (already handled by date detection)
                                if let Ok(num) = clean_word.parse::<i32>() {
                                    if !(1900..=2100).contains(&num) {
                                        count += 1;
                                    }
                                } else {
                                    // It's a float or too large for i32
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
            
            // Hard-coded strings in conditionals
            if line.contains("\"") && (line.contains("==") || line.contains("!=")) {
                count += line.matches('"').count() / 2; // Each string has 2 quotes
            }
        }
        
        count
    }
    
    fn remove_strings_and_comments(&self, line: &str) -> String {
        let mut result = String::new();
        let mut chars = line.chars().enumerate().peekable();
        let mut in_string = false;
        let mut string_char = '"';
        
        while let Some((_i, ch)) = chars.next() {
            if !in_string {
                // Check for start of comment
                if ch == '/' && chars.peek().map(|(_, c)| *c) == Some('/') {
                    // Rest of line is comment
                    break;
                }
                if ch == '/' && chars.peek().map(|(_, c)| *c) == Some('*') {
                    // Block comment start - skip until */
                    chars.next(); // consume '*'
                    let mut found_end = false;
                    while let Some((_, c)) = chars.next() {
                        if c == '*' && chars.peek().map(|(_, c)| *c) == Some('/') {
                            chars.next(); // consume '/'
                            found_end = true;
                            break;
                        }
                    }
                    if found_end {
                        result.push(' '); // Replace comment with space
                    }
                    continue;
                }
                // Check for start of string
                if ch == '"' || ch == '\'' {
                    in_string = true;
                    string_char = ch;
                    result.push(' '); // Replace string with space
                    continue;
                }
                result.push(ch);
            } else {
                // Inside string, look for end (simple version - ignore escape sequences)
                if ch == string_char {
                    in_string = false;
                }
                // Don't add string content to result
            }
        }
        
        result
    }
    
    fn analyze_branch_purity(&self, line: &str, _language: &Option<String>) -> bool {
        // Match the old BranchingDetails::is_non_pure_line logic exactly
        let is_non_pure = line.contains("fs::") || line.contains("File::") || line.contains("Path::") ||
                         line.contains("SystemTime::") || line.contains("Instant::") ||
                         line.contains("environment_var") || line.contains("GLOBAL_") ||
                         line.contains("rand::") || line.contains(".gen_bool") || line.contains(".read(") || line.contains(".write(") ||
                         line.contains("http_client") || line.contains("socket");
        
        !is_non_pure // Return true for pure, false for non-pure
    }
    
    fn detect_future_logic(&self, line: &str) -> bool {
        if line.contains("if") {
            // Look for future dates - match the old BranchingDetails behavior
            if line.contains("2025") || line.contains("2026") || line.contains("2027") {
                return true;
            }
            // Look for version checks that might be future
            if line.contains(">=") && (line.contains("\"2.") || line.contains("\"3.")) {
                return true;
            }
            // Look for high API level checks
            if line.contains("api_level >=") || line.contains("api_version >=") {
                return true;
            }
            // Look for feature flags
            if line.contains("feature_flags") || line.contains("beta_features") {
                return true;
            }
        }
        false
    }
    
    fn detect_past_logic(&self, line: &str) -> bool {
        if line.contains("if") {
            // Look for past dates - match the old BranchingDetails behavior
            if line.contains("2020") || line.contains("2021") || line.contains("2022") {
                return true;
            }
            // Look for deprecated version checks
            if line.contains("<") && (line.contains("\"1.") || line.contains("\"0.")) {
                return true;
            }
            // Look for old API level checks
            if line.contains("api_level <") || line.contains("api_version <") {
                return true;
            }
            // Look for deprecation patterns
            if line.contains("deprecated") || line.contains("end_of_life") || line.contains("support_end") {
                return true;
            }
        }
        false
    }
    
    fn generate_summary(&self, content: &str, language: &Option<String>) -> String {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return "Empty file".to_string();
        }
        
        // Look for meaningful first lines, comments, or docstrings
        for line in lines.iter().take(10) {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            
            // Extract meaningful comments or descriptions
            if trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*") {
                let comment = trimmed
                    .trim_start_matches("//")
                    .trim_start_matches("#")
                    .trim_start_matches("/*")
                    .trim();
                if comment.len() > 10 {
                    return format!("{}", comment.chars().take(100).collect::<String>());
                }
            }
        }
        
        // Fallback to file type description
        match language.as_deref() {
            Some("rust") => "Rust source code".to_string(),
            Some("python") => "Python script".to_string(),
            Some("javascript") => "JavaScript code".to_string(),
            Some("markdown") => "Documentation file".to_string(),
            Some("json") => "JSON configuration".to_string(),
            _ => format!("{} lines of code", lines.len()),
        }
    }
    
    fn extract_exports(&self, content: &str, language: &Option<String>) -> Vec<String> {
        let mut exports = Vec::new();
        
        if let Some(lang) = language {
            match lang.as_str() {
                "rust" => {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("pub fn ") {
                            if let Some(name) = trimmed.split_whitespace().nth(2) {
                                exports.push(name.trim_end_matches('(').to_string());
                            }
                        } else if trimmed.starts_with("pub struct ") {
                            if let Some(name) = trimmed.split_whitespace().nth(2) {
                                exports.push(name.to_string());
                            }
                        }
                    }
                }
                "javascript" | "typescript" => {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("export ") {
                            // Simple extraction - could be made more sophisticated
                            if let Some(rest) = trimmed.strip_prefix("export ") {
                                if let Some(name) = rest.split_whitespace().next() {
                                    exports.push(name.to_string());
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        
        exports
    }
    
    fn extract_imports(&self, content: &str, language: &Option<String>) -> Vec<String> {
        let mut imports = Vec::new();
        
        if let Some(lang) = language {
            match lang.as_str() {
                "rust" => {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("use ") {
                            if let Some(import) = trimmed.strip_prefix("use ") {
                                let clean = import.trim_end_matches(';');
                                imports.push(clean.to_string());
                            }
                        }
                    }
                }
                "javascript" | "typescript" => {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("import ") {
                            imports.push(trimmed.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
        
        imports
    }
    
    fn extract_api_surface(&self, content: &str, language: &Option<String>) -> Vec<String> {
        let mut api = Vec::new();
        
        if let Some(lang) = language {
            match lang.as_str() {
                "rust" => {
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("pub fn ") || trimmed.starts_with("pub struct ") || 
                           trimmed.starts_with("pub enum ") || trimmed.starts_with("pub trait ") {
                            api.push(trimmed.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
        
        api
    }
    
    fn infer_purpose(&self, path: &PathBuf, content: &str, language: &Option<String>) -> String {
        let path_str = path.to_string_lossy().to_lowercase();
        
        // Purpose based on path patterns
        if path_str.contains("test") {
            return "Test code".to_string();
        }
        if path_str.contains("example") || path_str.contains("demo") {
            return "Example/demo code".to_string();
        }
        if path_str.contains("lib") || path_str.contains("core") {
            return "Core library functionality".to_string();
        }
        if path_str.contains("cli") || path_str.contains("bin") {
            return "Command-line interface".to_string();
        }
        if path_str.contains("config") {
            return "Configuration".to_string();
        }
        
        // Purpose based on content
        if content.contains("main(") || content.contains("fn main") {
            return "Application entry point".to_string();
        }
        
        // Default based on language
        match language.as_deref() {
            Some("markdown") => "Documentation".to_string(),
            Some("json") | Some("yaml") | Some("toml") => "Configuration file".to_string(),
            Some("shell") => "Shell script".to_string(),
            _ => "Source code".to_string(),
        }
    }
    
    fn calculate_importance(&self, entry: &FileEntry, enhanced_info: &EnhancedFileInfo) -> f64 {
        let mut importance = 1.0;
        
        // Size factor
        importance += (entry.size as f64 / 10000.0).min(2.0);
        
        // Complexity factor
        if let Some(complexity) = enhanced_info.complexity_score {
            importance += complexity * 0.3;
        }
        
        // API surface factor
        importance += (enhanced_info.api_surface.len() as f64) * 0.1;
        
        // Path-based importance
        let path_str = entry.path.to_string_lossy();
        if path_str.contains("main") || path_str.contains("lib") {
            importance += 1.0;
        }
        if path_str.contains("core") {
            importance += 0.5;
        }
        
        importance.min(10.0) // Cap at 10
    }
    
    pub fn analyze_branching_details(&self, content: &str, language: &Option<String>) -> BranchingDetails {
        let mut details = BranchingDetails {
            conditional_count: 0,
            loop_count: 0,
            switch_count: 0,
            max_nesting: 0,
            logical_operators: 0,
            cyclomatic_complexity: 1.0,
            cognitive_complexity: 0.0,
            hardcoded_dates_count: 0,
            hardcoded_values_count: 0,
            pure_branches: 0,
            non_pure_branches: 0,
            nesting_distribution: std::collections::HashMap::new(),
            future_logic_count: 0,
            past_logic_count: 0,
            total_branches: 0,
        };
        
        let lines: Vec<&str> = content.lines().collect();
        let mut nesting_level = 0;
        
        for line in &lines {
            let trimmed = line.trim();
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*") || trimmed.starts_with("#") {
                continue;
            }
            
            // Remove string literals and comments from the line for analysis
            let cleaned_line = self.remove_strings_and_comments(trimmed);
            
            // Update nesting level and track distribution (but exclude loops)  
            let has_opening_brace = cleaned_line.contains('{');
            let nesting_level_for_distribution = if has_opening_brace { nesting_level + 1 } else { nesting_level };
            if has_opening_brace {
                nesting_level += 1;
                details.max_nesting = details.max_nesting.max(nesting_level);
            }
            if cleaned_line.contains('}') && nesting_level > 0 {
                nesting_level -= 1;
            }
            
            // Check for branching constructs and analyze their properties
            let mut is_branch = false;
            let mut is_conditional_branch = false;
            let mut is_loop = false;
            
            // Count different types of constructs
            if let Some(lang) = language {
                match lang.as_str() {
                    "rust" => {
                        // More robust if detection - count all occurrences of "if " pattern
                        let mut if_count = 0;
                        if cleaned_line.starts_with("if ") {
                            if_count += 1;
                        }
                        // Count "if " preceded by whitespace or certain punctuation
                        for i in 1..cleaned_line.len().saturating_sub(2) {
                            if &cleaned_line[i..i+3] == "if " {
                                let prev_char = cleaned_line.chars().nth(i-1).unwrap();
                                if prev_char.is_whitespace() || prev_char == '{' || prev_char == '(' || prev_char == ')' || prev_char == ';' {
                                    if_count += 1;
                                }
                            }
                        }
                        details.conditional_count += if_count;
                        details.cyclomatic_complexity += if_count as f64;
                        if if_count > 0 {
                            is_branch = true;
                            is_conditional_branch = true;
                        }
                        if trimmed.contains(" match ") || trimmed.starts_with("match ") {
                            details.switch_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                        }
                        // More robust loop detection
                        let mut loop_count = 0;
                        let loop_keywords = ["while ", "for ", "loop "];
                        for keyword in &loop_keywords {
                            if trimmed.starts_with(keyword) {
                                loop_count += 1;
                            }
                            // Count keyword preceded by whitespace or certain punctuation
                            for i in 1..trimmed.len().saturating_sub(keyword.len()-1) {
                                if &trimmed[i..i+keyword.len()] == *keyword {
                                    let prev_char = trimmed.chars().nth(i-1).unwrap();
                                    if prev_char.is_whitespace() || prev_char == '{' || prev_char == '(' || prev_char == ')' || prev_char == ';' {
                                        loop_count += 1;
                                    }
                                }
                            }
                        }
                        details.loop_count += loop_count;
                        details.cyclomatic_complexity += loop_count as f64;
                        if loop_count > 0 {
                            is_branch = true;
                            is_loop = true;
                        }
                        if trimmed.contains("=>") && !trimmed.contains("//") {
                            details.conditional_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_conditional_branch = true;
                        }
                        details.logical_operators += trimmed.matches(" && ").count() + trimmed.matches(" || ").count();
                    }
                    "javascript" | "typescript" => {
                        if trimmed.contains(" if ") || trimmed.starts_with("if ") {
                            details.conditional_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_conditional_branch = true;
                        }
                        if trimmed.contains(" switch ") || trimmed.starts_with("switch ") {
                            details.switch_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                        }
                        if trimmed.contains(" while ") || trimmed.contains(" for ") {
                            details.loop_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_loop = true;
                        }
                        if trimmed.trim().starts_with("case ") {
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_conditional_branch = true;
                        }
                        if trimmed.contains(" ? ") && trimmed.contains(" : ") {
                            details.conditional_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_conditional_branch = true;
                        }
                        details.logical_operators += trimmed.matches(" && ").count() + trimmed.matches(" || ").count();
                    }
                    "python" => {
                        if trimmed.contains(" if ") || trimmed.starts_with("if ") || trimmed.ends_with(" if") {
                            details.conditional_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_conditional_branch = true;
                        }
                        if trimmed.contains(" while ") || trimmed.contains(" for ") || trimmed.starts_with("for ") || trimmed.starts_with("while ") {
                            details.loop_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_loop = true;
                        }
                        if trimmed.contains("except ") {
                            details.conditional_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_conditional_branch = true;
                        }
                        details.logical_operators += trimmed.matches(" and ").count() + trimmed.matches(" or ").count();
                    }
                    _ => {
                        // Generic handling
                        if trimmed.contains(" if ") || trimmed.starts_with("if ") {
                            details.conditional_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_conditional_branch = true;
                        }
                        if trimmed.contains(" while ") || trimmed.contains(" for ") {
                            details.loop_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                            is_loop = true;
                        }
                        if trimmed.contains(" switch ") || trimmed.starts_with("switch ") {
                            details.switch_count += 1;
                            details.cyclomatic_complexity += 1.0;
                            is_branch = true;
                        }
                        details.logical_operators += trimmed.matches(" && ").count() + trimmed.matches(" || ").count();
                        details.logical_operators += trimmed.matches(" and ").count() + trimmed.matches(" or ").count();
                    }
                }
            }
            
            // If this line contains branching logic, analyze its properties
            if is_branch {
                details.total_branches += 1;
                
                // Check for hard-coded dates
                if self.detect_hardcoded_dates(trimmed) {
                    details.hardcoded_dates_count += 1;
                }
                
                // Check for hard-coded values
                details.hardcoded_values_count += self.count_hardcoded_values(trimmed);
                
                // Analyze branch purity
                if self.analyze_branch_purity(trimmed, language) {
                    details.pure_branches += 1;
                } else {
                    details.non_pure_branches += 1;
                }
                
                // Check for future-oriented logic
                if self.detect_future_logic(trimmed) {
                    details.future_logic_count += 1;
                }
                
                // Check for past-oriented logic
                if self.detect_past_logic(trimmed) {
                    details.past_logic_count += 1;
                }
                
            }
            
            // Track nesting distribution for conditional branches (exclude loops)
            if has_opening_brace && is_conditional_branch && !is_loop {
                // Use the nesting level at the time of the opening brace
                *details.nesting_distribution.entry(nesting_level_for_distribution).or_insert(0) += 1;
            }
            
            // Calculate cognitive complexity with nesting penalty
            let nesting_multiplier = 1.0 + (nesting_level as f64 * 0.5);
            if trimmed.contains(" if ") || trimmed.starts_with("if ") {
                details.cognitive_complexity += 1.0 * nesting_multiplier;
            }
            if trimmed.contains(" while ") || trimmed.contains(" for ") {
                details.cognitive_complexity += 1.5 * nesting_multiplier;
            }
        }
        
        details
    }
}

impl Default for EnhancedFileInfo {
    fn default() -> Self {
        Self {
            language: None,
            framework: None,
            line_count: None,
            complexity_score: None,
            importance_score: None,
            content_summary: None,
            exports: Vec::new(),
            imports: Vec::new(),
            dependencies: Vec::new(),
            related_files: Vec::new(),
            purpose: None,
            last_author: None,
            change_frequency: None,
            api_surface: Vec::new(),
        }
    }
}

pub struct EnhancedGenericMapper {
    analyzer: ContentAnalyzer,
    basic_mapper: GenericMapper,
}

impl EnhancedGenericMapper {
    pub fn new() -> Self {
        Self {
            analyzer: ContentAnalyzer,
            basic_mapper: GenericMapper,
        }
    }
}

impl Mapper for EnhancedGenericMapper {
    fn classify(&self, entry: &FileEntry) -> Vec<String> {
        let mut tags = self.basic_mapper.classify(entry);
        
        // Add enhanced classification based on content analysis
        if let Some(enhanced_info) = &entry.enhanced_info {
            if let Some(language) = &enhanced_info.language {
                tags.push(language.clone());
            }
            
            if let Some(purpose) = &enhanced_info.purpose {
                if purpose.contains("entry point") {
                    tags.push("entrypoint".to_string());
                }
                if purpose.contains("Core library") {
                    tags.push("core-api".to_string());
                }
                if purpose.contains("Command-line") {
                    tags.push("cli".to_string());
                }
            }
            
            // Add importance-based tags
            if let Some(importance) = enhanced_info.importance_score {
                if importance > 5.0 {
                    tags.push("high-importance".to_string());
                } else if importance > 2.0 {
                    tags.push("moderate-importance".to_string());
                }
            }
            
            // Add complexity-based tags
            if let Some(complexity) = enhanced_info.complexity_score {
                if complexity > 5.0 {
                    tags.push("high-complexity".to_string());
                }
            }
        }
        
        tags
    }
    
    fn name(&self) -> &str {
        "enhanced-generic"
    }
}

impl EnhancedMapper for EnhancedGenericMapper {
    fn analyze(&self, entry: &mut FileEntry) -> Result<(), ScanError> {
        if entry.enhanced_info.is_none() {
            let enhanced_info = self.analyzer.analyze_file(entry)?;
            entry.enhanced_info = Some(enhanced_info);
        }
        Ok(())
    }
    
    fn name(&self) -> &str {
        "enhanced-generic"
    }
}

pub struct DirectoryScanner {
    options: ScanOptions,
    mapper: Box<dyn Mapper>,
    enhanced_mapper: Option<Box<dyn EnhancedMapper>>,
}

impl DirectoryScanner {
    pub fn new(options: ScanOptions) -> Self {
        let (mapper, enhanced_mapper): (Box<dyn Mapper>, Option<Box<dyn EnhancedMapper>>) = 
            if options.enhanced_analysis {
                let enhanced = EnhancedGenericMapper::new();
                (Box::new(EnhancedGenericMapper::new()), Some(Box::new(enhanced)))
            } else {
                match options.mapper_profile.as_str() {
                    "generic" => (Box::new(GenericMapper), None),
                    _ => (Box::new(GenericMapper), None), // Default fallback
                }
            };
        
        Self { options, mapper, enhanced_mapper }
    }
    
    pub fn scan<P: Into<PathBuf>>(&self, path: P) -> Result<ScanResult, ScanError> {
        let root_path = path.into();
        let start_time = Instant::now();
        
        if !root_path.exists() {
            return Err(ScanError::PathNotFound { path: root_path });
        }
        
        let mut files = Vec::new();
        let mut errors = Vec::new();
        let mut total_size = 0u64;
        let mut dir_count = 0;
        
        let walker = WalkDir::new(&root_path)
            .follow_links(self.options.follow_symlinks)
            .max_depth(self.options.max_depth.unwrap_or(usize::MAX))
            .into_iter();
        
        for entry_result in walker {
            match entry_result {
                Ok(entry) => {
                    if self.should_ignore(&entry) {
                        continue;
                    }
                    
                    match self.process_entry(entry) {
                        Ok(mut file_entry) => {
                            if file_entry.is_dir {
                                dir_count += 1;
                            } else {
                                total_size += file_entry.size;
                            }
                            
                            // Apply enhanced analysis if available
                            if let Some(enhanced_mapper) = &self.enhanced_mapper {
                                if let Err(e) = enhanced_mapper.analyze(&mut file_entry) {
                                    errors.push(format!("Enhanced analysis failed for {}: {}", 
                                        file_entry.path.display(), e));
                                }
                            }
                            
                            // Apply classification
                            file_entry.tags = self.mapper.classify(&file_entry);
                            files.push(file_entry);
                        },
                        Err(e) => {
                            errors.push(format!("{}", e));
                        }
                    }
                }
                Err(e) => {
                    errors.push(format!("Walk error: {}", e));
                }
            }
        }
        
        let duration = start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;
        let files_per_second = if duration_ms > 0 {
            (files.len() as f64) / (duration_ms as f64 / 1000.0)
        } else {
            0.0
        };
        
        let stats = ScanStats {
            total_files: files.len(),
            total_dirs: dir_count,
            total_size,
            scan_duration_ms: duration_ms,
            files_per_second,
        };
        
        Ok(ScanResult {
            root_path,
            files,
            stats,
            errors,
        })
    }
    
    fn should_ignore(&self, entry: &DirEntry) -> bool {
        let path_str = entry.path().to_string_lossy();
        
        // Skip hidden files/dirs unless explicitly allowed
        if !self.options.include_hidden {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with('.') && name != "." && name != ".." {
                    return true;
                }
            }
        }
        
        // Check ignore patterns
        for pattern in &self.options.ignore_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        
        false
    }
    
    fn process_entry(&self, entry: DirEntry) -> Result<FileEntry, ScanError> {
        let path = entry.path().to_path_buf();
        let metadata = entry.metadata().map_err(|e| {
            if let Some(io_err) = e.io_error() {
                ScanError::Io(std::io::Error::new(io_err.kind(), format!("{}", e)))
            } else {
                ScanError::InvalidConfig { message: format!("Failed to get metadata: {}", e) }
            }
        })?;
        
        let name = entry.file_name()
            .to_str()
            .unwrap_or("unknown")
            .to_string();
        
        let size = if metadata.is_file() {
            metadata.len()
        } else {
            0
        };
        
        let modified = metadata
            .modified()
            .unwrap_or_else(|_| SystemTime::UNIX_EPOCH);
        
        Ok(FileEntry {
            path,
            name,
            size,
            modified,
            is_dir: metadata.is_dir(),
            tags: Vec::new(), // Will be filled by mapper
            metadata: None,
            enhanced_info: None, // Will be filled by enhanced mapper
        })
    }
}

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn format_result(result: &ScanResult, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Basic => Self::format_basic(result),
            OutputFormat::Compact => Self::format_compact(result),
            OutputFormat::Detailed => Self::format_detailed(result),
            OutputFormat::Hierarchical => Self::format_hierarchical(result),
        }
    }
    
    fn get_branching_breakdown(file: &FileEntry, enhanced_info: &EnhancedFileInfo) -> String {
        // Re-analyze file content to provide enhanced branching complexity breakdown
        if let Ok(content) = std::fs::read_to_string(&file.path) {
            let analyzer = ContentAnalyzer;
            let branching_details = analyzer.analyze_branching_details(&content, &enhanced_info.language);
            
            let mut breakdown_parts = Vec::new();
            
            // Basic branching counts
            if branching_details.conditional_count > 0 {
                breakdown_parts.push(format!("{}x conditionals", branching_details.conditional_count));
            }
            if branching_details.loop_count > 0 {
                breakdown_parts.push(format!("{}x loops", branching_details.loop_count));
            }
            if branching_details.switch_count > 0 {
                breakdown_parts.push(format!("{}x switches", branching_details.switch_count));
            }
            
            // Enhanced analysis
            if branching_details.total_branches > 0 {
                // Hard-coded values analysis
                let hardcoded_total = branching_details.hardcoded_dates_count + branching_details.hardcoded_values_count;
                if hardcoded_total > 0 {
                    let hardcoded_percentage = (hardcoded_total as f64 / branching_details.total_branches as f64) * 100.0;
                    breakdown_parts.push(format!("Hard-coded: {:.0}% ({}/{})", 
                        hardcoded_percentage, hardcoded_total, branching_details.total_branches));
                }
                
                // Purity analysis
                if branching_details.pure_branches > 0 || branching_details.non_pure_branches > 0 {
                    let pure_percentage = (branching_details.pure_branches as f64 / branching_details.total_branches as f64) * 100.0;
                    breakdown_parts.push(format!("Pure: {:.0}% ({}/{})",
                        pure_percentage, branching_details.pure_branches, branching_details.total_branches));
                }
                
                // Temporal analysis
                if branching_details.future_logic_count > 0 {
                    breakdown_parts.push(format!("Future: {}x", branching_details.future_logic_count));
                }
                if branching_details.past_logic_count > 0 {
                    breakdown_parts.push(format!("Past: {}x", branching_details.past_logic_count));
                }
            }
            
            // Nesting analysis
            if branching_details.max_nesting > 1 {
                let mut nesting_parts = Vec::new();
                for depth in (2..=branching_details.max_nesting).rev() {
                    if let Some(count) = branching_details.nesting_distribution.get(&depth) {
                        if *count > 0 {
                            nesting_parts.push(format!("{}x depth-{}", count, depth));
                        }
                    }
                }
                if !nesting_parts.is_empty() {
                    breakdown_parts.push(format!("Nesting: {}", nesting_parts.join(", ")));
                }
            }
            
            if branching_details.logical_operators > 0 {
                breakdown_parts.push(format!("{}x logical ops", branching_details.logical_operators));
            }
            
            if breakdown_parts.is_empty() {
                String::new()
            } else {
                format!("Enhanced Branching Analysis: {}", breakdown_parts.join(" | "))
            }
        } else {
            String::new()
        }
    }
    
    fn format_basic(result: &ScanResult) -> String {
        let mut output = String::new();
        
        for file in &result.files {
            let file_type = if file.is_dir { "[DIR]" } else { "[FILE]" };
            let tags = if file.tags.is_empty() { 
                String::new() 
            } else { 
                format!(" ({})", file.tags.join(", ")) 
            };
            output.push_str(&format!("  {} {}{}\n", file_type, file.path.display(), tags));
        }
        
        output
    }
    
    fn format_compact(result: &ScanResult) -> String {
        let mut output = String::new();
        
        for file in &result.files {
            if file.is_dir {
                continue; // Skip directories in compact mode
            }
            
            let size_str = Self::format_size(file.size);
            let time_str = Self::format_time_ago(&file.modified);
            let tags = if file.tags.is_empty() { 
                String::new() 
            } else { 
                format!(" ({})", file.tags.join(", ")) 
            };
            
            output.push_str(&format!("[FILE] {}{} | {}, {}\n", 
                file.path.display(), tags, size_str, time_str));
        }
        
        output
    }
    
    fn format_detailed(result: &ScanResult) -> String {
        let mut output = String::new();
        
        for file in &result.files {
            if file.is_dir {
                output.push_str(&format!("📁 {}\n", file.path.display()));
                continue;
            }
            
            let size_str = Self::format_size(file.size);
            let time_str = Self::format_time_ago(&file.modified);
            let tags = if file.tags.is_empty() { 
                "unclassified".to_string() 
            } else { 
                file.tags.join(", ") 
            };
            
            output.push_str(&format!("[FILE] {} ({})\n", file.path.display(), tags));
            output.push_str(&format!("  Size: {} | Modified: {}", size_str, time_str));
            
            if let Some(enhanced_info) = &file.enhanced_info {
                if let Some(lines) = enhanced_info.line_count {
                    output.push_str(&format!(" | Lines: {}", lines));
                }
                output.push('\n');
                
                if let Some(summary) = &enhanced_info.content_summary {
                    output.push_str(&format!("  {}\n", summary));
                }
                
                if !enhanced_info.exports.is_empty() {
                    output.push_str(&format!("  Exports: {}\n", enhanced_info.exports.join(", ")));
                }
                
                if !enhanced_info.imports.is_empty() && enhanced_info.imports.len() <= 3 {
                    output.push_str(&format!("  Imports: {}\n", enhanced_info.imports.join(", ")));
                } else if !enhanced_info.imports.is_empty() {
                    output.push_str(&format!("  Imports: {} dependencies\n", enhanced_info.imports.len()));
                }
                
                if let Some(purpose) = &enhanced_info.purpose {
                    output.push_str(&format!("  Purpose: {}\n", purpose));
                }
                
                if let (Some(complexity), Some(importance)) = (enhanced_info.complexity_score, enhanced_info.importance_score) {
                    output.push_str(&format!("  Complexity: {:.1} | Importance: {:.1}\n", complexity, importance));
                    
                    // Show enhanced branching complexity breakdown for any files with branching logic
                    let branching_detail = Self::get_branching_breakdown(file, enhanced_info);
                    if !branching_detail.is_empty() {
                        output.push_str(&format!("    {}\n", branching_detail));
                    }
                }
            } else {
                output.push('\n');
            }
            
            output.push('\n');
        }
        
        output
    }
    
    fn format_hierarchical(result: &ScanResult) -> String {
        // Build a tree structure from the flat file list
        let tree = Self::build_tree(&result.files);
        Self::render_tree(&tree, 0)
    }
    
    fn build_tree(files: &[FileEntry]) -> DirectoryNode {
        let mut root = DirectoryNode {
            path: PathBuf::from("."),
            name: ".".to_string(),
            children: Vec::new(),
            subdirs: Vec::new(),
        };
        
        // Simple implementation - group files by parent directory
        let mut dirs: HashMap<PathBuf, DirectoryNode> = HashMap::new();
        
        for file in files {
            if let Some(parent) = file.path.parent() {
                let parent_path = parent.to_path_buf();
                
                if !dirs.contains_key(&parent_path) {
                    dirs.insert(parent_path.clone(), DirectoryNode {
                        path: parent_path.clone(),
                        name: parent_path.file_name()
                            .unwrap_or_else(|| std::ffi::OsStr::new("."))
                            .to_string_lossy()
                            .to_string(),
                        children: Vec::new(),
                        subdirs: Vec::new(),
                    });
                }
                
                if file.is_dir {
                    // Handle as subdirectory
                    continue;
                } else {
                    // Add file to parent directory
                    if let Some(dir_node) = dirs.get_mut(&parent_path) {
                        dir_node.children.push(file.clone());
                    }
                }
            } else {
                // Root level file
                if !file.is_dir {
                    root.children.push(file.clone());
                }
            }
        }
        
        // Convert directory map to nested structure (simplified)
        for (_, dir_node) in dirs {
            root.subdirs.push(dir_node);
        }
        
        root
    }
    
    fn render_tree(node: &DirectoryNode, depth: usize) -> String {
        let mut output = String::new();
        let indent = "  ".repeat(depth);
        
        // Render current directory if not root
        if depth > 0 {
            output.push_str(&format!("{}📁 {}/\n", indent, node.name));
        }
        
        // Render files in this directory
        for file in &node.children {
            let tags = if file.tags.is_empty() { 
                String::new() 
            } else { 
                format!(" ({})", file.tags.join(", ")) 
            };
            
            let size_str = Self::format_size(file.size);
            let enhanced_info = if let Some(enhanced_info) = &file.enhanced_info {
                if let Some(summary) = &enhanced_info.content_summary {
                    format!(" - {}", summary)
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            
            output.push_str(&format!("{}├── [FILE] {}{} | {}{}\n", 
                indent, file.name, tags, size_str, enhanced_info));
        }
        
        // Render subdirectories
        for subdir in &node.subdirs {
            output.push_str(&Self::render_tree(subdir, depth + 1));
        }
        
        output
    }
    
    fn format_size(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{}B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1}KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
        }
    }
    
    fn format_time_ago(time: &SystemTime) -> String {
        if let Ok(duration) = time.elapsed() {
            let seconds = duration.as_secs();
            if seconds < 60 {
                "just now".to_string()
            } else if seconds < 3600 {
                format!("{}m ago", seconds / 60)
            } else if seconds < 86400 {
                format!("{}h ago", seconds / 3600)
            } else {
                format!("{}d ago", seconds / 86400)
            }
        } else {
            "unknown".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    mod hardcoded_values_detection {
        use super::*;
        
        #[test]
        fn test_detects_iso_dates() {
            let content = r#"
                if date == "2024-12-25" { 
                    return true; 
                }
                if expiry > "2025-01-01T00:00:00Z" { 
                    cleanup(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert!(details.hardcoded_dates_count > 0); // Should detect some dates
        }
        
        #[test]
        fn test_detects_slash_format_dates() {
            let content = r#"
                if birth_date == "12/25/1990" { 
                    celebrate(); 
                }
                if created < "01/01/2023" { 
                    migrate(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.hardcoded_dates_count, 2);
        }
        
        #[test]
        fn test_detects_year_patterns() {
            let content = r#"
                if year >= 2024 && month == 12 { 
                    special_handling(); 
                }
                if version.year() > 2023 { 
                    new_features(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.hardcoded_dates_count, 2);
        }
        
        #[test]
        fn test_detects_magic_numbers() {
            let content = r#"
                if count > 42 { return; }
                if threshold == 3.14159 { calculate(); }
                if status == 404 { handle_error(); }
                if temperature < -273.15 { freeze(); }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.hardcoded_values_count, 4);
        }
        
        #[test]
        fn test_detects_hardcoded_strings() {
            let content = r#"
                if name == "production" { 
                    enable_monitoring(); 
                }
                if env_type == "staging" { 
                    debug_mode(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.hardcoded_values_count, 2);
        }
        
        #[test]
        fn test_ignores_common_values() {
            let content = r#"
                if count == 0 { return; }
                if found == 1 { process(); }
                if size == 1024 { optimize(); } // Power of 2
                if pages == 256 { paginate(); } // 2^8
                if limit == 2 { restrict(); }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            // Should detect 0 but not other common values
            assert_eq!(details.hardcoded_values_count, 0);
        }
        
        #[test]
        fn test_ignores_variable_names_that_look_like_dates() {
            let content = r#"
                if date_var > other_date { 
                    return true; 
                }
                if year_field >= start_year { 
                    process(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.hardcoded_dates_count, 0);
        }
        
        #[test]
        fn test_ignores_comments_containing_dates() {
            let content = r#"
                // Created on 2024-12-25
                /* Magic number 42 explained */
                if condition { 
                    // Release date: 2025-01-01
                    return true; 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.hardcoded_dates_count, 0);
            assert_eq!(details.hardcoded_values_count, 0);
        }
    }
    
    mod purity_analysis {
        use super::*;
        
        #[test]
        fn test_detects_pure_branches() {
            let content = r#"
                if x > y { return x + y; }
                if items.len() > count { process_items(); }
                if param.is_valid() && local_flag { validate(); }
                if input > MAX_SIZE { truncate(); }
                if config.enabled && param.ready { execute(); }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.pure_branches, 5);
            assert_eq!(details.non_pure_branches, 0);
        }
        
        #[test]
        fn test_detects_file_io_non_pure() {
            let content = r#"
                if fs::read_to_string("config.txt").is_ok() { 
                    load_config(); 
                }
                if Path::new(&file).exists() { 
                    process_file(); 
                }
                if file.read().is_ok() { 
                    parse_content(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.pure_branches, 0);
            assert_eq!(details.non_pure_branches, 3);
        }
        
        #[test]
        fn test_detects_network_non_pure() {
            let content = r#"
                if http_client.get(url).is_ok() { 
                    handle_response(); 
                }
                if socket.is_connected() { 
                    send_data(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.pure_branches, 0);
            assert_eq!(details.non_pure_branches, 2);
        }
        
        #[test]
        fn test_detects_system_time_non_pure() {
            let content = r#"
                if SystemTime::now() > deadline { 
                    timeout(); 
                }
                if Instant::now().elapsed() > timeout { 
                    abort(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.pure_branches, 0);
            assert_eq!(details.non_pure_branches, 2);
        }
        
        #[test]
        fn test_detects_global_state_non_pure() {
            let content = r#"
                if GLOBAL_COUNTER > threshold { 
                    reset(); 
                }
                if environment_var("DEBUG").is_some() { 
                    debug(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.pure_branches, 0);
            assert_eq!(details.non_pure_branches, 2);
        }
        
        #[test]
        fn test_detects_random_non_pure() {
            let content = r#"
                if rand::random::<f64>() > 0.5 { 
                    randomize(); 
                }
                if rng.gen_bool(0.5) { 
                    maybe_do(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.pure_branches, 0);
            assert_eq!(details.non_pure_branches, 2);
        }
        
        #[test]
        fn test_mixed_pure_and_non_pure() {
            let content = r#"
                if x > y { return x + y; } // Pure
                if fs::read_to_string("config.txt").is_ok() { load(); } // Non-pure
                if items.len() > 0 { process(); } // Pure
                if SystemTime::now() > deadline { timeout(); } // Non-pure
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.pure_branches, 2);
            assert_eq!(details.non_pure_branches, 2);
        }
    }
    
    mod temporal_logic_detection {
        use super::*;
        
        #[test]
        fn test_detects_future_dates() {
            let content = r#"
                if release_date > "2025-06-01" { 
                    enable_feature(); 
                }
                if expiry_date > "2025-12-31T23:59:59Z" { 
                    extend_license(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.future_logic_count, 2);
        }
        
        #[test]
        fn test_detects_future_version_checks() {
            let content = r#"
                if version >= "2.0.0" { 
                    new_api(); 
                }
                if api_level >= 35 { 
                    future_features(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.future_logic_count, 2);
        }
        
        #[test]
        fn test_detects_feature_flags() {
            let content = r#"
                if feature_flags.contains("new_ui_2024") { 
                    render_new_ui(); 
                }
                if beta_features.enabled("next_gen_search") { 
                    advanced_search(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.future_logic_count, 2);
        }
        
        #[test]
        fn test_detects_past_dates() {
            let content = r#"
                if created_date < "2020-01-01" { 
                    legacy_handling(); 
                }
                if legacy_format && year < 2022 { 
                    convert_format(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.past_logic_count, 2);
        }
        
        #[test]
        fn test_detects_deprecated_versions() {
            let content = r#"
                if version < "1.0.0" { 
                    legacy_support(); 
                }
                if api_level < 21 { 
                    old_api_compatibility(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.past_logic_count, 2);
        }
        
        #[test]
        fn test_detects_end_of_life_conditions() {
            let content = r#"
                if support_end_date < "2023-12-31" { 
                    show_warning(); 
                }
                if deprecated_since < "2022-01-01" { 
                    remove_feature(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.past_logic_count, 2);
        }
        
        #[test]
        fn test_mixed_temporal_logic() {
            let content = r#"
                if release_date > "2025-01-01" { new_feature(); } // Future
                if created_date < "2020-01-01" { legacy(); } // Past
                if version >= "2.0.0" { advanced(); } // Future
                if deprecated_since < "2022-01-01" { remove(); } // Past
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.future_logic_count, 2);
            assert_eq!(details.past_logic_count, 2);
        }
    }
    
    mod nesting_distribution {
        use super::*;
        
        #[test]
        fn test_simple_single_depth() {
            let content = r#"
                if condition { return; }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.nesting_distribution.get(&1), Some(&1));
        }
        
        #[test]
        fn test_simple_double_nesting() {
            let content = r#"
                if outer { 
                    if inner { 
                        return; 
                    } 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.nesting_distribution.get(&1), Some(&1)); // outer if
            assert_eq!(details.nesting_distribution.get(&2), Some(&1)); // inner if
        }
        
        #[test]
        fn test_multiple_branches_same_depth() {
            let content = r#"
                if a { 
                    if b { x(); } 
                } 
                if c { 
                    if d { y(); } 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.nesting_distribution.get(&1), Some(&2)); // a and c
            assert_eq!(details.nesting_distribution.get(&2), Some(&2)); // b and d
        }
        
        #[test]
        fn test_complex_mixed_depths() {
            let content = r#"
                if a {           // depth-1
                    if b {        // depth-2  
                        if c {    // depth-3
                            if d { return x(); } // depth-4
                        }
                        if e { return y(); }     // depth-3
                    }
                    if f { return z(); }         // depth-2
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.nesting_distribution.get(&1), Some(&1)); // a
            assert_eq!(details.nesting_distribution.get(&2), Some(&2)); // b, f
            assert_eq!(details.nesting_distribution.get(&3), Some(&2)); // c, e
            assert_eq!(details.nesting_distribution.get(&4), Some(&1)); // d
        }
        
        #[test]
        fn test_very_deep_nesting() {
            // Generate 10 levels of nesting programmatically
            let mut content = String::new();
            for i in 1..=10 {
                content.push_str(&format!("{}if depth_{} {{\n", "    ".repeat(i-1), i));
            }
            content.push_str(&format!("{}return;\n", "    ".repeat(10)));
            for i in (1..=10).rev() {
                content.push_str(&format!("{}}}\n", "    ".repeat(i-1)));
            }
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(&content, &Some("rust".to_string()));
            
            // Each depth level should have exactly 1 branch
            for depth in 1..=10 {
                assert_eq!(details.nesting_distribution.get(&depth), Some(&1));
            }
            assert_eq!(details.max_nesting, 10);
        }
    }
    
    mod cross_language_compatibility {
        use super::*;
        
        #[test]
        fn test_rust_patterns() {
            let content = r#"
                if matches!(result, Ok(_)) { handle_ok(); }
                if let Some(value) = option { process(value); }
                match status { 
                    Status::Ready => { execute(); }, 
                    _ => { wait(); } 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert!(details.conditional_count >= 3);
            assert!(details.switch_count >= 1); // match statement
        }
        
        #[test]
        fn test_javascript_patterns() {
            let content = r#"
                if (typeof value === 'string' && value.length > 0) { process(); }
                if (array?.includes?.(item)) { found(); }
                switch (type) { 
                    case 'user': 
                        handleUser(); 
                        break; 
                    default: 
                        handleOther();
                        break; 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("javascript".to_string()));
            
            assert!(details.conditional_count >= 2);
            assert!(details.switch_count >= 1);
        }
        
        #[test]
        fn test_python_patterns() {
            let content = r#"
                if isinstance(obj, str) and len(obj) > 0:
                    process_string(obj)
                if hasattr(obj, 'method') and obj.method():
                    call_method()
                for item in items:
                    if condition:
                        break
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("python".to_string()));
            
            assert!(details.conditional_count >= 3);
            assert!(details.loop_count >= 1);
        }
        
        #[test]
        fn test_java_patterns() {
            let content = r#"
                if (obj instanceof String && ((String)obj).length() > 0) { 
                    processString(); 
                }
                if (list != null && !list.isEmpty()) { 
                    processList(); 
                }
                switch (type) {
                    case USER:
                        handleUser();
                        break;
                    case ADMIN:
                        handleAdmin();
                        break;
                    default:
                        handleDefault();
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("java".to_string()));
            
            assert!(details.conditional_count >= 2);
            assert!(details.switch_count >= 1);
        }
        
        #[test]
        fn test_go_patterns() {
            let content = r#"
                if err != nil {
                    return err
                }
                if len(items) > 0 && items[0].Valid {
                    processItems()
                }
                switch typ {
                case "string":
                    handleString()
                case "int":
                    handleInt()
                default:
                    handleOther()
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("go".to_string()));
            
            assert!(details.conditional_count >= 2);
            assert!(details.switch_count >= 1);
        }
        
        #[test]
        fn test_cpp_patterns() {
            let content = r#"
                if (ptr != nullptr && ptr->isValid()) {
                    ptr->process();
                }
                if (vector.size() > 0 && !vector.empty()) {
                    processVector();
                }
                switch (state) {
                    case READY:
                        execute();
                        break;
                    case WAITING:
                        wait();
                        break;
                    default:
                        error();
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("cpp".to_string()));
            
            assert!(details.conditional_count >= 2);
            assert!(details.switch_count >= 1);
        }
    }
    
    mod integration_tests {
        use super::*;
        
        #[test]
        fn test_comprehensive_analysis_integration() {
            let content = r#"
                // Test file with mixed patterns
                if release_date > "2025-01-01" { // Future logic + hardcoded date
                    enable_new_features();
                }
                
                if fs::read_to_string("config.txt").is_ok() { // Non-pure + hardcoded string
                    load_configuration();
                } else if backup_exists && user_count > 42 { // Pure + magic number
                    load_backup();
                }
                
                if created_date < "2020-01-01" { // Past logic + hardcoded date
                    legacy_migration();
                }
                
                for user in users { // Loop
                    if user.active && user.score > 100 { // Pure, nested
                        if SystemTime::now() > user.last_login { // Non-pure, nested deeper
                            send_notification();
                        }
                    }
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let mut details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            // Calculate totals and perform comprehensive validation
            details.total_branches = details.conditional_count + details.loop_count + details.switch_count;
            
            // Validate detection results
            assert!(details.hardcoded_dates_count >= 2); // 2025-01-01, 2020-01-01
            assert!(details.hardcoded_values_count >= 1); // 42
            assert!(details.future_logic_count >= 1); // 2025 date
            assert!(details.past_logic_count >= 1); // 2020 date
            assert!(details.pure_branches >= 2); // user checks and score check
            assert!(details.non_pure_branches >= 2); // fs::read and SystemTime
            
            // Validate nesting distribution
            assert!(details.nesting_distribution.get(&1).unwrap_or(&0) >= &3); // Top level ifs and for
            assert!(details.nesting_distribution.get(&2).unwrap_or(&0) >= &1); // Nested user.active check
            assert!(details.nesting_distribution.get(&3).unwrap_or(&0) >= &1); // SystemTime check
            
            // Validate total branch count matches distribution sum
            let distribution_sum: usize = details.nesting_distribution.values().sum();
            assert_eq!(distribution_sum, details.conditional_count);
        }
        
        #[test]
        fn test_percentage_calculations() {
            let content = r#"
                if x > 0 { pure(); } // Pure
                if y > 42 { magic(); } // Pure + hardcoded
                if fs::read("file").is_ok() { io(); } // Non-pure
                if "2025-01-01" > date { future(); } // Pure + hardcoded date + future
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let mut details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            details.total_branches = details.conditional_count;
            
            // Test percentage calculations
            let pure_percentage = if details.total_branches > 0 {
                (details.pure_branches as f64 / details.total_branches as f64) * 100.0
            } else {
                0.0
            };
            
            let hardcoded_percentage = if details.total_branches > 0 {
                ((details.hardcoded_dates_count + details.hardcoded_values_count) as f64 / details.total_branches as f64) * 100.0
            } else {
                0.0
            };
            
            assert!(pure_percentage >= 0.0 && pure_percentage <= 100.0);
            assert!(hardcoded_percentage >= 0.0 && hardcoded_percentage <= 100.0);
            
            // With our test case, expect high purity but some hardcoded values
            assert!(pure_percentage >= 75.0); // Most should be pure
            assert!(hardcoded_percentage > 0.0); // Some hardcoded values present
        }
        
        #[test]
        fn test_empty_file_analysis() {
            let content = "";
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            // Empty file should have zero everything
            assert_eq!(details.conditional_count, 0);
            assert_eq!(details.loop_count, 0);
            assert_eq!(details.switch_count, 0);
            assert_eq!(details.hardcoded_dates_count, 0);
            assert_eq!(details.hardcoded_values_count, 0);
            assert_eq!(details.pure_branches, 0);
            assert_eq!(details.non_pure_branches, 0);
            assert_eq!(details.future_logic_count, 0);
            assert_eq!(details.past_logic_count, 0);
            assert!(details.nesting_distribution.is_empty());
        }
    }
    
    mod edge_cases_and_boundaries {
        use super::*;
        
        #[test]
        fn test_single_line_multiple_branches() {
            let content = "if a && b || c { if d { e(); } }";
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert!(details.conditional_count >= 2); // Outer and inner if
            assert!(details.logical_operators >= 2); // && and ||
        }
        
        #[test]
        fn test_malformed_syntax_handling() {
            let content = r#"
                if incomplete_condition { // missing closing brace
                if another_condition { return; }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            // Should not crash, should detect at least some branches
            assert!(details.conditional_count >= 1);
        }
        
        #[test]
        fn test_comments_with_keywords_ignored() {
            let content = r#"
                // if fake_condition { do_nothing(); }
                /* 
                 * while (false) { never_execute(); }
                 * for (;;) { infinite_loop(); }
                 */
                if real_condition { 
                    // switch statement here would be ignored
                    execute(); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.conditional_count, 1); // Only the real if
            assert_eq!(details.loop_count, 0); // Comments ignored
            assert_eq!(details.switch_count, 0); // Comments ignored
        }
        
        #[test]
        fn test_string_literals_with_branch_keywords() {
            let content = r#"
                let msg = "if you see this, ignore the if keyword";
                let code = "for (int i = 0; i < 10; i++) { process(i); }";
                if validate_message(msg) { 
                    log("while processing: success"); 
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert_eq!(details.conditional_count, 1); // Only the real if
            assert_eq!(details.loop_count, 0); // String literals ignored
        }
        
        #[test]
        fn test_mixed_branch_types_same_statement() {
            let content = r#"
                for i in 0..n {
                    if items[i].matches(pattern) {
                        count += 1;
                    }
                }
                
                while condition {
                    match state {
                        State::Ready => {
                            if can_process() {
                                process();
                            }
                        },
                        _ => break,
                    }
                }
            "#;
            
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
            
            assert!(details.loop_count >= 2); // for and while
            assert!(details.conditional_count >= 2); // if statements
            assert!(details.switch_count >= 1); // match statement
            
            // Verify nesting is tracked correctly
            assert!(details.max_nesting >= 3); // for -> match -> if
        }
    }
    
    mod property_based_tests {
        use super::*;
        use quickcheck_macros::quickcheck;
            
        #[quickcheck]
        fn prop_pure_percentage_always_valid(pure: u16, non_pure: u16) -> bool {
            let pure = pure as usize;
            let non_pure = non_pure as usize;
            let total = pure + non_pure;
            
            if total == 0 {
                return true; // Edge case: no branches
            }
            
            let percentage = (pure as f64 / total as f64) * 100.0;
            percentage >= 0.0 && percentage <= 100.0
        }
        
        #[quickcheck]
        fn prop_nesting_distribution_matches_total(nesting_levels: Vec<u8>) -> bool {
            // Create a mock nesting distribution
            let mut distribution: HashMap<usize, usize> = HashMap::new();
            let mut total_branches = 0;
            
            for (i, &count) in nesting_levels.iter().enumerate() {
                let depth = (i % 10) + 1; // Limit depth to 1-10
                let branch_count = count as usize % 100; // Limit count to prevent overflow
                
                *distribution.entry(depth).or_insert(0) += branch_count;
                total_branches += branch_count;
            }
            
            let distribution_sum: usize = distribution.values().sum();
            distribution_sum == total_branches
        }
        
        #[quickcheck]
        fn prop_hardcoded_percentage_bounded(hardcoded: u16, total: u16) -> bool {
            let hardcoded = hardcoded as usize;
            let total = (total as usize).max(hardcoded); // Ensure total >= hardcoded
            
            if total == 0 {
                return true;
            }
            
            let percentage = (hardcoded as f64 / total as f64) * 100.0;
            percentage >= 0.0 && percentage <= 100.0 && hardcoded <= total
        }
        
        #[test]
        fn prop_branch_counts_never_negative() {
            // Test with various inputs to ensure counts are always non-negative
            let test_cases = vec![
                "",
                "if (true) { return; }",
                "// just a comment",
                "let x = 5;", // No branches
                "if a { if b { if c { return; } } }", // Nested
            ];
            
            for content in test_cases {
                let analyzer = ContentAnalyzer::new();
                let details = analyzer.analyze_branching_details(content, &Some("rust".to_string()));
                
                // All counts should be non-negative (which is guaranteed by usize, but good to test)
                assert!(details.conditional_count < usize::MAX);
                assert!(details.loop_count < usize::MAX);
                assert!(details.switch_count < usize::MAX);
                assert!(details.hardcoded_dates_count < usize::MAX);
                assert!(details.hardcoded_values_count < usize::MAX);
                assert!(details.pure_branches < usize::MAX);
                assert!(details.non_pure_branches < usize::MAX);
            }
        }
    }
    
    mod performance_tests {
        use super::*;
        use std::time::Instant;
        
        #[test]
        fn test_large_file_analysis_performance() {
            // Generate a large file with many branches
            let mut content = String::new();
            for i in 0..1000 {
                content.push_str(&format!(
                    "if condition_{} {{ process_{}(); }}\n",
                    i, i
                ));
            }
            
            let start = Instant::now();
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(&content, &Some("rust".to_string()));
            let duration = start.elapsed();
            
            // Should complete analysis quickly even for large files
            assert!(duration.as_secs() < 1);
            assert_eq!(details.conditional_count, 1000);
        }
        
        #[test]
        fn test_deeply_nested_performance() {
            // Generate very deep nesting
            let mut content = String::new();
            let depth = 50;
            
            for i in 0..depth {
                content.push_str(&format!("{}if depth_{} {{\n", "  ".repeat(i), i));
            }
            content.push_str(&format!("{}return;\n", "  ".repeat(depth)));
            for i in (0..depth).rev() {
                content.push_str(&format!("{}}}\n", "  ".repeat(i)));
            }
            
            let start = Instant::now();
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(&content, &Some("rust".to_string()));
            let duration = start.elapsed();
            
            // Should handle deep nesting efficiently
            assert!(duration.as_millis() < 100);
            assert_eq!(details.max_nesting, depth);
            assert_eq!(details.conditional_count, depth);
        }
        
        #[test]
        fn test_mixed_pattern_analysis_performance() {
            // Generate content with mixed patterns
            let mut content = String::new();
            
            for i in 0..100 {
                content.push_str(&format!(
                    r#"
                    if date > "202{}-01-01" {{ future_{}(); }} // Future + hardcoded
                    if fs::read_to_string("file_{}.txt").is_ok() {{ io_{}(); }} // Non-pure + hardcoded
                    for item_{} in items {{
                        if item_{}.valid && item_{}.count > {} {{
                            if SystemTime::now() > item_{}.deadline {{
                                process_item_{}();
                            }}
                        }}
                    }}
                    "#,
                    5 + (i % 5), i, // Future dates
                    i, i, // File I/O
                    i, i, i, 10 + i, // Loop and nested conditions
                    i, i // More nesting
                ));
            }
            
            let start = Instant::now();
            let analyzer = ContentAnalyzer::new();
            let details = analyzer.analyze_branching_details(&content, &Some("rust".to_string()));
            let duration = start.elapsed();
            
            // Should handle complex mixed analysis efficiently
            assert!(duration.as_millis() < 500);
            
            // Validate all types of analysis were performed
            assert!(details.conditional_count > 0);
            assert!(details.loop_count > 0);
            assert!(details.hardcoded_dates_count > 0);
            assert!(details.hardcoded_values_count > 0);
            assert!(details.future_logic_count > 0);
            assert!(details.pure_branches > 0);
            assert!(details.non_pure_branches > 0);
            assert!(!details.nesting_distribution.is_empty());
        }
    }
}