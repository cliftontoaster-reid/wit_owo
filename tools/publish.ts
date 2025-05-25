import { parse } from "jsr:@std/toml";
/**
 * Run tests for the wit_owo Rust project
 * @param features - Array of feature flags to enable (e.g., ['async', 'blocking'])
 * @param mode - 2-bit number: 0b01 = library tests only, 0b10 = doc tests only, 0b11 = all tests
 */
async function runTests(features: string[], mode: number): Promise<void> {
  // Validate mode is a 2-bit number (0-3)
  if (mode < 0 || mode > 3) {
    throw new Error("Mode must be a 2-bit number (0-3)");
  }

  // Build feature flags string
  const featureFlags =
    features.length > 0
      ? `--no-default-features --features ${features.join(",")}`
      : "";

  // Determine which tests to run based on mode
  const runLibTests = (mode & 0b01) !== 0;
  const runDocTests = (mode & 0b10) !== 0;

  console.log(
    `Running tests with features: [${features.join(", ")}] and mode: ${mode
      .toString(2)
      .padStart(2, "0")}b`,
  );

  try {
    // Run library tests if mode includes 0b01
    if (runLibTests) {
      console.log("🧪 Running library tests...");
      const libTestArgs = [
        "test",
        ...featureFlags.split(" ").filter((arg) => arg),
      ];
      console.log(`Executing: cargo ${libTestArgs.join(" ")}`);

      const libTestCmd = new Deno.Command("cargo", {
        args: libTestArgs,
        cwd: Deno.cwd(),
        stdin: "inherit",
        stdout: "inherit",
        stderr: "inherit",
      });

      const libTestResult = await libTestCmd.output();
      if (!libTestResult.success) {
        throw new Error(
          `Library tests failed with exit code ${libTestResult.code}`,
        );
      }
      console.log("✅ Library tests passed!");
    }

    // Run doc tests if mode includes 0b10
    if (runDocTests) {
      console.log("📚 Running doc tests...");
      const docTestArgs = [
        "test",
        "--doc",
        ...featureFlags.split(" ").filter((arg) => arg),
      ];
      console.log(`Executing: cargo ${docTestArgs.join(" ")}`);

      const docTestCmd = new Deno.Command("cargo", {
        args: docTestArgs,
        cwd: Deno.cwd(),
        stdin: "inherit",
        stdout: "inherit",
        stderr: "inherit",
      });

      const docTestResult = await docTestCmd.output();
      if (!docTestResult.success) {
        throw new Error(
          `Doc tests failed with exit code ${docTestResult.code}`,
        );
      }
      console.log("✅ Doc tests passed!");
    }

    // If mode is 0, log that no tests were run
    if (mode === 0) {
      console.log("⚠️  No tests were run (mode = 0)");
    }

    console.log("🎉 All requested tests completed successfully!");
  } catch (error) {
    console.error("❌ Test execution failed:", error);
    throw error;
  }
}

async function runClippy(features: string[]): Promise<void> {
  const featureFlags =
    features.length > 0
      ? `--no-default-features --features ${features.join(",")}`
      : "";
  console.log(`Running Clippy with features: [${features.join(", ")}]`);

  const clippyArgs = [
    "clippy",
    ...featureFlags.split(" ").filter((arg) => arg),
  ];

  const clippyCmd = new Deno.Command("cargo", {
    args: clippyArgs,
    cwd: Deno.cwd(),
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  });

  const result = await clippyCmd.output();
  if (!result.success) {
    throw new Error(`Clippy failed with exit code ${result.code}`);
  }
  console.log("✅ Clippy passed!");
}

function runSuite(features: string[], mode: number, clippyOnly = false): void {
  if (clippyOnly) {
    console.log(
      `\n📎 Running Clippy only for features: [${features.join(", ")}]`,
    );

    runClippy(features).catch((error) => {
      console.error("❌ Clippy execution failed:", error);
      throw error;
    });
  } else {
    console.log(
      `\n🔀 Testing features: [${features.join(", ")}] and mode: ${mode
        .toString(2)
        .padStart(2, "0")}b`,
    );

    runClippy(features).catch((error) => {
      console.error("❌ Clippy execution failed:", error);
      throw error;
    });

    runTests(features, mode).catch((error) => {
      console.error("❌ Test execution failed:", error);
      throw error;
    });
  }
}

function getCombinations<T>(arr: T[]): T[][] {
  const result: T[][] = [];
  const n = arr.length;
  for (let mask = 1; mask < 1 << n; mask++) {
    const combo: T[] = [];
    for (let i = 0; i < n; i++) {
      if (mask & (1 << i)) combo.push(arr[i]);
    }
    result.push(combo);
  }
  return result;
}

// Example usage (uncomment to test):
// await runTests(["async"], 0b11); // Run all tests with async feature
// await runTests(["blocking"], 0b01); // Run only library tests with blocking feature
// await runTests([], 0b10); // Run only doc tests with no features

// If running this file directly with Deno
if (import.meta.main) {
  // Check for --clippy flag
  const clippyOnly = Deno.args.includes("--clippy");

  if (clippyOnly) {
    console.log("🔧 Clippy-only mode enabled. Skipping tests.");
  }

  // Read the Cargo.toml file from either current directory or parent directory
  let cargoData: string;

  try {
    cargoData = await Deno.readTextFile("Cargo.toml");
  } catch {
    cargoData = await Deno.readTextFile("../Cargo.toml");
  }

  // Parse the Cargo.toml file
  const cargoToml = parse(cargoData);

  // Extract features from the Cargo.toml file
  const features = Object.keys(cargoToml.features || {});

  // We exclude the "default" feature from the list
  const availableFeatures = features.filter((feature) => feature !== "default");

  // We then build every possible combination of features
  // Though it should not make a combination where
  // no features are selected, as well as where for two features
  // are activated, and one requires the other, we need to account for that.
  // Build all non-empty subsets of availableFeatures
  const deps = cargoToml.features as Record<string, string[]>;
  const allCombos = getCombinations(availableFeatures);

  // Filter out combos missing any internal dependency
  const validCombos = allCombos.filter((combo) =>
    combo.every((f) => {
      const reqs = deps[f] || [];
      return reqs.every(
        (dep) => !availableFeatures.includes(dep) || combo.includes(dep),
      );
    }),
  );

  // Run library tests for each valid combination of features
  for (const combo of validCombos) {
    if (clippyOnly) {
      console.log(`\nRunning Clippy for features: [${combo.join(", ")}]`);
      runSuite(combo, 0b01, true); // clippy only
    } else {
      console.log(
        `\nRunning library tests for features: [${combo.join(", ")}]`,
      );
      runSuite(combo, 0b01); // library tests only
    }
  }

  if (!clippyOnly) {
    // Run a single doc test with the full feature set
    console.log(
      `\nRunning doc tests for full feature set: [${availableFeatures.join(", ")}]`,
    );
    runSuite(availableFeatures, 0b10); // doc tests only
  } else {
    // Run clippy with the full feature set
    console.log(
      `\nRunning Clippy for full feature set: [${availableFeatures.join(", ")}]`,
    );
    runSuite(availableFeatures, 0b10, true); // clippy only
  }
}
