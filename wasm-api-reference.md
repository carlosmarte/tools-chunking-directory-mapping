# WASM API Reference

This document describes the comprehensive WASM API for the Thinkeloquent Tools Chunking Directory Mapping library, providing 100% functionality coverage of the core library.

## Complete Function List

### Core Scanning Functions

#### `scan_directory(path: string, options?: string): Promise<WasmScanResult>`
Enhanced directory scanning with multiple output formats included in the result.
- **Parameters:**
  - `path`: Directory path to scan
  - `options`: Optional JSON string of `WasmScanOptions`
- **Returns:** `WasmScanResult` containing raw result plus formatted/JSON/YAML outputs

#### `scan_directory_simple(path: string, options?: string): Promise<ScanResult>`
Basic directory scanning returning just the core scan result.
- **Parameters:**
  - `path`: Directory path to scan  
  - `options`: Optional JSON string of `ScanOptions`
- **Returns:** Raw `ScanResult` object

### Output Formatting Functions

#### `format_scan_result(result_json: string, format_type: string): Promise<string>`
Format scan results using different output formats.
- **Parameters:**
  - `result_json`: JSON string of scan result
  - `format_type`: One of "basic", "compact", "detailed", "hierarchical"
- **Returns:** Formatted string output

#### `scan_result_to_yaml(result_json: string): Promise<string>`
Convert scan result to YAML format.
- **Parameters:**
  - `result_json`: JSON string of scan result
- **Returns:** YAML formatted string

### File Analysis Functions

#### `analyze_file_content(file_path: string, content: string, language?: string): Promise<EnhancedFileInfo>`
Analyze individual file content for enhanced information.
- **Parameters:**
  - `file_path`: Path to the file being analyzed
  - `content`: File content as string
  - `language`: Optional programming language hint
- **Returns:** Enhanced file information object

#### `analyze_branching_details(content: string, language?: string): Promise<BranchingDetails>`
Analyze code complexity and branching patterns.
- **Parameters:**
  - `content`: Code content to analyze
  - `language`: Optional programming language hint  
- **Returns:** Detailed branching analysis object

### Result Processing Functions

#### `get_scan_statistics(result_json: string): Promise<ScanStats>`
Extract statistics from scan results.
- **Parameters:**
  - `result_json`: JSON string of scan result
- **Returns:** Statistics object

#### `get_scan_errors(result_json: string): Promise<string[]>`
Extract error information from scan results.
- **Parameters:**
  - `result_json`: JSON string of scan result
- **Returns:** Array of error messages

### Configuration Functions

#### `create_scan_options(mapper_profile?, enhanced_analysis?, output_format?, max_depth?, include_hidden?, follow_symlinks?): Promise<string>`
Create properly formatted scan options JSON.
- **Parameters:** All optional configuration parameters
- **Returns:** JSON string of scan options

#### `validate_scan_options(options_json: string): Promise<boolean>`
Validate scan options format.
- **Parameters:**
  - `options_json`: JSON string to validate
- **Returns:** Boolean indicating if options are valid

### Utility Functions

#### `get_supported_output_formats(): string[]`
Get list of supported output formats.
- **Returns:** Array of format names

#### `get_supported_mapper_profiles(): string[]`
Get list of supported mapper profiles.
- **Returns:** Array of profile names

#### `get_version(): string`
Get library version.
- **Returns:** Version string

#### `get_build_info(): BuildInfo`
Get detailed build information.
- **Returns:** Object with version, build date, git SHA, etc.

## Data Types

### WasmScanOptions
```typescript
interface WasmScanOptions {
  mapper_profile?: string;        // "generic" or "enhanced"
  enhanced_analysis?: boolean;    // Enable detailed analysis
  output_format?: string;         // "basic", "compact", "detailed", "hierarchical"
  max_depth?: number;            // Maximum directory depth
  include_hidden?: boolean;       // Include hidden files
  follow_symlinks?: boolean;      // Follow symbolic links
  ignore_patterns?: string[];     // Glob patterns to ignore
}
```

### WasmScanResult
```typescript
interface WasmScanResult {
  result: ScanResult;           // Raw scan result
  formatted_output?: string;    // Pre-formatted text output
  json_output?: string;         // JSON formatted output
  yaml_output?: string;         // YAML formatted output
}
```

### ScanResult
```typescript
interface ScanResult {
  files: FileEntry[];           // List of discovered files
  directories: DirectoryNode[]; // Directory structure
  stats: ScanStats;            // Scanning statistics
  errors: string[];            // Any errors encountered
}
```

### ScanStats
```typescript
interface ScanStats {
  total_files: number;
  total_dirs: number;
  total_size: number;
  scan_duration_ms: number;
  files_per_second: number;
}
```

### EnhancedFileInfo
```typescript
interface EnhancedFileInfo {
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
```

### BranchingDetails
```typescript
interface BranchingDetails {
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
```

## Feature Coverage Comparison

| Feature | CLI | Core | WASM | Coverage |
|---------|-----|------|------|----------|
| Directory Scanning | ✅ | ✅ | ✅ | 100% |
| Output Formats (Basic/Compact/Detailed/Hierarchical) | ✅ | ✅ | ✅ | 100% |
| JSON Output | ✅ | ✅ | ✅ | 100% |
| YAML Output | ✅ | ✅ | ✅ | 100% |
| Enhanced Analysis | ✅ | ✅ | ✅ | 100% |
| File Content Analysis | ❌ | ✅ | ✅ | 100% |
| Branching Details Analysis | ❌ | ✅ | ✅ | 100% |
| Configuration Options | ✅ | ✅ | ✅ | 100% |
| Error Handling | ✅ | ✅ | ✅ | 100% |
| Statistics Reporting | ✅ | ✅ | ✅ | 100% |
| Profile Selection | ✅ | ✅ | ✅ | 100% |
| Individual Component Access | ❌ | ✅ | ✅ | 100% |

## Usage Examples

### Basic Directory Scan
```javascript
import init, { scan_directory } from './pkg/thinkeloquent_tools_chunking_directory_mapping_wasm.js';

await init();
const result = await scan_directory('/path/to/directory');
console.log(result.formatted_output);
```

### Advanced Configuration
```javascript
const options = await create_scan_options(
  'enhanced',    // mapper_profile
  true,          // enhanced_analysis
  'detailed',    // output_format
  10,            // max_depth
  false,         // include_hidden
  true           // follow_symlinks
);

const result = await scan_directory('/path/to/directory', options);
```

### Individual File Analysis
```javascript
const fileContent = await fetch('/path/to/file.js').then(r => r.text());
const analysis = await analyze_file_content('/path/to/file.js', fileContent, 'javascript');
console.log('Complexity:', analysis.complexity_score);

const branchingDetails = await analyze_branching_details(fileContent, 'javascript');
console.log('Cyclomatic complexity:', branchingDetails.cyclomatic_complexity);
```

### Output Format Conversion
```javascript
const result = await scan_directory_simple('/path/to/directory');
const resultJson = JSON.stringify(result);

const yamlOutput = await scan_result_to_yaml(resultJson);
const hierarchicalOutput = await format_scan_result(resultJson, 'hierarchical');
const compactOutput = await format_scan_result(resultJson, 'compact');
```

This WASM implementation now provides **100% feature coverage** of the core library, making all functionality available to web applications and JavaScript environments.