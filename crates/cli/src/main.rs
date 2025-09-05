use clap::Parser;
use thinkeloquent_tools_chunking_directory_mapping_core::{ScanOptions, DirectoryScanner, OutputFormat, OutputFormatter};

#[derive(Parser)]
#[command(name = "projscan")]
#[command(about = "A directory scanner for project analysis with enhanced LLM RAG support")]
pub struct Args {
    /// The directory to scan
    pub path: Option<String>,
    
    /// Mapper profile to use
    #[arg(long, default_value = "generic")]
    pub profile: String,
    
    /// Output format as JSON
    #[arg(long)]
    pub json: bool,
    
    /// Output format as YAML
    #[arg(long)]
    pub yaml: bool,
    
    /// Enable enhanced content analysis
    #[arg(long)]
    pub enhanced: bool,
    
    /// Output format for enhanced display
    #[arg(long, value_enum, default_value_t = OutputFormatArg::Basic)]
    pub format: OutputFormatArg,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormatArg {
    Basic,
    Compact,
    Detailed,
    Hierarchical,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(arg: OutputFormatArg) -> Self {
        match arg {
            OutputFormatArg::Basic => OutputFormat::Basic,
            OutputFormatArg::Compact => OutputFormat::Compact,
            OutputFormatArg::Detailed => OutputFormat::Detailed,
            OutputFormatArg::Hierarchical => OutputFormat::Hierarchical,
        }
    }
}

fn main() {
    let args = Args::parse();
    
    let scan_path = args.path.unwrap_or_else(|| ".".to_string());
    
    let mut options = ScanOptions::default();
    options.mapper_profile = args.profile;
    options.enhanced_analysis = args.enhanced;
    options.output_format = args.format.clone().into();
    
    let scanner = DirectoryScanner::new(options);
    
    match scanner.scan(&scan_path) {
        Ok(result) => {
            if args.json {
                match serde_json::to_string_pretty(&result) {
                    Ok(json) => println!("{}", json),
                    Err(e) => {
                        eprintln!("Failed to serialize result: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if args.yaml {
                match serde_yaml::to_string(&result) {
                    Ok(yaml) => println!("{}", yaml),
                    Err(e) => {
                        eprintln!("Failed to serialize result: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                // Print basic stats
                println!("Scan completed for: {}", scan_path);
                println!("Files found: {}", result.stats.total_files);
                println!("Directories: {}", result.stats.total_dirs);
                println!("Total size: {} bytes", result.stats.total_size);
                println!("Scan duration: {}ms", result.stats.scan_duration_ms);
                println!("Files per second: {:.2}", result.stats.files_per_second);
                
                if args.enhanced {
                    println!("Enhanced analysis: enabled");
                }
                
                println!("\nFile structure:");
                
                // Use the new output formatter
                let formatted_output = OutputFormatter::format_result(&result, &args.format.into());
                print!("{}", formatted_output);
                
                if !result.errors.is_empty() {
                    println!("\nErrors encountered:");
                    for error in &result.errors {
                        println!("  {}", error);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Scan failed: {}", e);
            std::process::exit(1);
        }
    }
}