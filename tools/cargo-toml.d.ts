/**
 * TypeScript definitions for Cargo.toml configuration file
 * Based on the Cargo Book specification: https://doc.rust-lang.org/cargo/reference/manifest.html
 */

export interface CargoToml {
  /** Package configuration section */
  package?: PackageConfig;

  /** Workspace configuration section */
  workspace?: WorkspaceConfig;

  /** Dependencies section */
  dependencies?: Record<string, DependencySpec>;

  /** Development dependencies section */
  "dev-dependencies"?: Record<string, DependencySpec>;

  /** Build dependencies section */
  "build-dependencies"?: Record<string, DependencySpec>;

  /** Target-specific dependencies */
  target?: Record<string, TargetConfig>;

  /** Feature configuration */
  features?: Record<string, string[]>;

  /** Binary targets */
  bin?: BinaryTarget[];

  /** Library target */
  lib?: LibraryTarget;

  /** Example targets */
  example?: ExampleTarget[];

  /** Test targets */
  test?: TestTarget[];

  /** Benchmark targets */
  bench?: BenchTarget[];

  /** Build script configuration */
  build?: string;

  /** Patch section for dependency patching */
  patch?: Record<string, Record<string, DependencySpec>>;

  /** Profile configurations */
  profile?: Record<string, ProfileConfig>;

  /** Badges configuration */
  badges?: Record<string, BadgeConfig>;
}

export interface PackageConfig {
  /** Package name */
  name: string;

  /** Package version */
  version: string;

  /** Rust edition (2015, 2018, 2021, 2024) */
  edition?: "2015" | "2018" | "2021" | "2024";

  /** Minimum Rust version required */
  "rust-version"?: string;

  /** Package authors */
  authors?: string[];

  /** Package license */
  license?: string;

  /** License file path */
  "license-file"?: string;

  /** Package description */
  description?: string;

  /** Homepage URL */
  homepage?: string;

  /** Documentation URL */
  documentation?: string;

  /** Repository URL */
  repository?: string;

  /** README file path */
  readme?: string | boolean;

  /** Package keywords */
  keywords?: string[];

  /** Package categories */
  categories?: string[];

  /** Workspace path */
  workspace?: string;

  /** Build script path */
  build?: string | boolean;

  /** Links to a native library */
  links?: string;

  /** Files to exclude from package */
  exclude?: string[];

  /** Files to include in package */
  include?: string[];

  /** Whether to publish to a registry */
  publish?: boolean | string[];

  /** Package metadata */
  // deno-lint-ignore no-explicit-any
  metadata?: Record<string, any>;

  /** Default run target */
  "default-run"?: string;

  /** Autobins configuration */
  autobins?: boolean;

  /** Autoexamples configuration */
  autoexamples?: boolean;

  /** Autotests configuration */
  autotests?: boolean;

  /** Autobenches configuration */
  autobenches?: boolean;

  /** Resolver version */
  resolver?: "1" | "2";
}

export interface WorkspaceConfig {
  /** Workspace members */
  members?: string[];

  /** Default workspace members */
  "default-members"?: string[];

  /** Excluded workspace members */
  exclude?: string[];

  /** Workspace resolver version */
  resolver?: "1" | "2";

  /** Workspace-wide dependencies */
  dependencies?: Record<string, DependencySpec>;

  /** Workspace package configuration */
  package?: Partial<PackageConfig>;

  /** Workspace metadata */
  // deno-lint-ignore no-explicit-any
  metadata?: Record<string, any>;
}

export type DependencySpec = string | SimpleDependency | DetailedDependency;

export interface SimpleDependency {
  /** Dependency version */
  version?: string;

  /** Optional features to enable */
  features?: string[];

  /** Whether dependency is optional */
  optional?: boolean;

  /** Whether to use default features */
  "default-features"?: boolean;
}

export interface DetailedDependency extends SimpleDependency {
  /** Dependency version (required for registry dependencies) */
  version?: string;

  /** Git repository URL */
  git?: string;

  /** Git branch */
  branch?: string;

  /** Git tag */
  tag?: string;

  /** Git revision */
  rev?: string;

  /** Local path */
  path?: string;

  /** Registry name */
  registry?: string;

  /** Package name (if different from key) */
  package?: string;

  /** Registry index URL */
  "registry-index"?: string;
}

export interface TargetConfig {
  /** Target-specific dependencies */
  dependencies?: Record<string, DependencySpec>;

  /** Target-specific dev dependencies */
  "dev-dependencies"?: Record<string, DependencySpec>;

  /** Target-specific build dependencies */
  "build-dependencies"?: Record<string, DependencySpec>;
}

export interface BinaryTarget {
  /** Binary name */
  name: string;

  /** Source file path */
  path?: string;

  /** Whether to build by default */
  test?: boolean;

  /** Whether to test by default */
  doctest?: boolean;

  /** Whether to benchmark by default */
  bench?: boolean;

  /** Whether to document by default */
  doc?: boolean;

  /** Whether to include in check */
  "proc-macro"?: boolean;

  /** Harness configuration */
  harness?: boolean;

  /** Edition override */
  edition?: string;

  /** Required features */
  "required-features"?: string[];
}

export interface LibraryTarget {
  /** Library name */
  name?: string;

  /** Source file path */
  path?: string;

  /** Whether to test by default */
  test?: boolean;

  /** Whether to test documentation */
  doctest?: boolean;

  /** Whether to benchmark by default */
  bench?: boolean;

  /** Whether to document by default */
  doc?: boolean;

  /** Whether this is a proc-macro crate */
  "proc-macro"?: boolean;

  /** Harness configuration */
  harness?: boolean;

  /** Edition override */
  edition?: string;

  /** Library crate types */
  "crate-type"?: (
    | "bin"
    | "lib"
    | "rlib"
    | "dylib"
    | "cdylib"
    | "staticlib"
    | "proc-macro"
  )[];

  /** Required features */
  "required-features"?: string[];
}

export interface ExampleTarget {
  /** Example name */
  name: string;

  /** Source file path */
  path?: string;

  /** Whether to test by default */
  test?: boolean;

  /** Whether to document by default */
  doc?: boolean;

  /** Edition override */
  edition?: string;

  /** Example crate types */
  "crate-type"?: ("bin" | "lib")[];

  /** Required features */
  "required-features"?: string[];
}

export interface TestTarget {
  /** Test name */
  name: string;

  /** Source file path */
  path?: string;

  /** Harness configuration */
  harness?: boolean;

  /** Edition override */
  edition?: string;

  /** Required features */
  "required-features"?: string[];
}

export interface BenchTarget {
  /** Benchmark name */
  name: string;

  /** Source file path */
  path?: string;

  /** Harness configuration */
  harness?: boolean;

  /** Edition override */
  edition?: string;

  /** Required features */
  "required-features"?: string[];
}

export interface ProfileConfig {
  /** Optimization level */
  "opt-level"?: number | "s" | "z";

  /** Debug information */
  debug?: boolean | number;

  /** Split debug information */
  "split-debuginfo"?: "packed" | "unpacked" | "off";

  /** Strip symbols */
  strip?: boolean | "symbols" | "debuginfo";

  /** Debug assertions */
  "debug-assertions"?: boolean;

  /** Overflow checks */
  "overflow-checks"?: boolean;

  /** Link-time optimization */
  lto?: boolean | "thin" | "fat" | "off";

  /** Panic strategy */
  panic?: "unwind" | "abort";

  /** Incremental compilation */
  incremental?: boolean;

  /** Code generation units */
  "codegen-units"?: number;

  /** Relocation model */
  rpath?: boolean;

  /** Package-specific settings */
  package?: Record<string, Partial<ProfileConfig>>;

  /** Build override settings */
  "build-override"?: Partial<ProfileConfig>;
}

export interface BadgeConfig {
  /** Repository badge */
  repository?: string;

  /** Branch */
  branch?: string;

  /** Service */
  service?: string;

  /** Additional badge properties */
  // deno-lint-ignore no-explicit-any
  [key: string]: any;
}

export default CargoToml;
