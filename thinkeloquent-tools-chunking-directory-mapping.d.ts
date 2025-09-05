/**
 * TypeScript definitions for thinkeloquent-tools-chunking-directory-mapping WASM module
 * Provides 100% feature coverage of the core Rust library
 */

export interface WasmScanOptions {
  mapper_profile?: 'generic' | 'enhanced';
  enhanced_analysis?: boolean;
  output_format?: 'basic' | 'compact' | 'detailed' | 'hierarchical';
  max_depth?: number;
  include_hidden?: boolean;
  follow_symlinks?: boolean;
  ignore_patterns?: string[];
}

export interface FileEntry {
  path: string;
  name: string;
  size: number;
  modified: number; // Unix timestamp
  is_dir: boolean;
  tags: string[];
  metadata?: Record<string, any>;
  enhanced_info?: EnhancedFileInfo;
}

export interface EnhancedFileInfo {
  language?: string;
  framework?: string;
  line_count?: number;
  complexity_score?: number;
  importance_score?: number;
  content_summary?: string;
  exports: string[];
  imports: string[];
  dependencies: string[];
  related_files: string[];
  purpose?: string;
  last_author?: string;
  change_frequency?: string;
  api_surface: string[];
}

export interface DirectoryNode {
  path: string;
  name: string;
  children: DirectoryNode[];
  files: FileEntry[];
}

export interface ScanStats {
  total_files: number;
  total_dirs: number;
  total_size: number;
  scan_duration_ms: number;
  files_per_second: number;
}

export interface ScanResult {
  files: FileEntry[];
  directories: DirectoryNode[];
  stats: ScanStats;
  errors: string[];
}

export interface WasmScanResult {
  result: ScanResult;
  formatted_output?: string;
  json_output?: string;
  yaml_output?: string;
}

export interface BranchingDetails {
  conditional_count: number;
  loop_count: number;
  switch_count: number;
  max_nesting: number;
  logical_operators: number;
  cyclomatic_complexity: number;
  cognitive_complexity: number;
  hardcoded_dates_count: number;
  hardcoded_values_count: number;
  pure_branches: number;
  non_pure_branches: number;
  future_logic_count: number;
  past_logic_count: number;
  total_branches: number;
}

export interface BuildInfo {
  version: string;
  name: string;
  authors: string;
  description: string;
  build_date: string;
  git_sha: string;
}

/**
 * Initialize the WASM module
 */
export default function init(input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module): Promise<void>;

/**
 * Enhanced directory scanning with multiple output formats included in the result
 */
export function scan_directory(path: string, options?: string): Promise<WasmScanResult>;

/**
 * Basic directory scanning returning just the core scan result
 */
export function scan_directory_simple(path: string, options?: string): Promise<ScanResult>;

/**
 * Format scan results using different output formats
 */
export function format_scan_result(result_json: string, format_type: 'basic' | 'compact' | 'detailed' | 'hierarchical'): Promise<string>;

/**
 * Convert scan result to YAML format
 */
export function scan_result_to_yaml(result_json: string): Promise<string>;

/**
 * Analyze individual file content for enhanced information
 */
export function analyze_file_content(file_path: string, content: string, language?: string): Promise<EnhancedFileInfo>;

/**
 * Analyze code complexity and branching patterns
 */
export function analyze_branching_details(content: string, language?: string): Promise<BranchingDetails>;

/**
 * Extract statistics from scan results
 */
export function get_scan_statistics(result_json: string): Promise<ScanStats>;

/**
 * Extract error information from scan results
 */
export function get_scan_errors(result_json: string): Promise<string[]>;

/**
 * Create properly formatted scan options JSON
 */
export function create_scan_options(
  mapper_profile?: string,
  enhanced_analysis?: boolean,
  output_format?: string,
  max_depth?: number,
  include_hidden?: boolean,
  follow_symlinks?: boolean
): Promise<string>;

/**
 * Validate scan options format
 */
export function validate_scan_options(options_json: string): Promise<boolean>;

/**
 * Get list of supported output formats
 */
export function get_supported_output_formats(): string[];

/**
 * Get list of supported mapper profiles
 */
export function get_supported_mapper_profiles(): string[];

/**
 * Get library version
 */
export function get_version(): string;

/**
 * Get detailed build information
 */
export function get_build_info(): BuildInfo;

/**
 * Helper class for working with scan options
 */
export class ScanOptionsBuilder {
  private options: WasmScanOptions;

  constructor() {
    this.options = {};
  }

  mapperProfile(profile: 'generic' | 'enhanced'): ScanOptionsBuilder {
    this.options.mapper_profile = profile;
    return this;
  }

  enhancedAnalysis(enabled: boolean): ScanOptionsBuilder {
    this.options.enhanced_analysis = enabled;
    return this;
  }

  outputFormat(format: 'basic' | 'compact' | 'detailed' | 'hierarchical'): ScanOptionsBuilder {
    this.options.output_format = format;
    return this;
  }

  maxDepth(depth: number): ScanOptionsBuilder {
    this.options.max_depth = depth;
    return this;
  }

  includeHidden(include: boolean): ScanOptionsBuilder {
    this.options.include_hidden = include;
    return this;
  }

  followSymlinks(follow: boolean): ScanOptionsBuilder {
    this.options.follow_symlinks = follow;
    return this;
  }

  ignorePatterns(patterns: string[]): ScanOptionsBuilder {
    this.options.ignore_patterns = patterns;
    return this;
  }

  build(): string {
    return JSON.stringify(this.options);
  }
}

/**
 * Helper class for processing scan results
 */
export class ScanResultProcessor {
  constructor(private result: ScanResult) {}

  static fromJson(json: string): ScanResultProcessor {
    return new ScanResultProcessor(JSON.parse(json));
  }

  getFilesByExtension(extension: string): FileEntry[] {
    return this.result.files.filter(file => file.name.endsWith(extension));
  }

  getFilesBySize(minSize?: number, maxSize?: number): FileEntry[] {
    return this.result.files.filter(file => {
      if (minSize !== undefined && file.size < minSize) return false;
      if (maxSize !== undefined && file.size > maxSize) return false;
      return true;
    });
  }

  getLargestFiles(count: number = 10): FileEntry[] {
    return [...this.result.files]
      .sort((a, b) => b.size - a.size)
      .slice(0, count);
  }

  getFilesByLanguage(language: string): FileEntry[] {
    return this.result.files.filter(file => 
      file.enhanced_info?.language === language
    );
  }

  getTotalComplexityScore(): number {
    return this.result.files.reduce((total, file) => 
      total + (file.enhanced_info?.complexity_score || 0), 0
    );
  }

  async formatAs(format: 'basic' | 'compact' | 'detailed' | 'hierarchical'): Promise<string> {
    return format_scan_result(JSON.stringify(this.result), format);
  }

  async toYaml(): Promise<string> {
    return scan_result_to_yaml(JSON.stringify(this.result));
  }
}