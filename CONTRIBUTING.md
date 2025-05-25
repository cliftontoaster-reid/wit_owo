<!-- omit in toc -->

# Contributing to Wit OwO

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
>
> - Star the project
> - Tweet about it
> - Refer this project in your project's README
> - Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->

## Table of Contents

- [Contributing to Wit OwO](#contributing-to-wit-owo)
  - [Table of Contents](#table-of-contents)
  - [Code of Conduct](#code-of-conduct)
  - [I Have a Question](#i-have-a-question)
  - [I Want To Contribute](#i-want-to-contribute)
    - [Reporting Bugs](#reporting-bugs)
      - [Before Submitting a Bug Report](#before-submitting-a-bug-report)
      - [How Do I Submit a Good Bug Report?](#how-do-i-submit-a-good-bug-report)
    - [Suggesting Enhancements](#suggesting-enhancements)
      - [Before Submitting an Enhancement](#before-submitting-an-enhancement)
      - [How Do I Submit a Good Enhancement Suggestion?](#how-do-i-submit-a-good-enhancement-suggestion)
    - [Your First Code Contribution](#your-first-code-contribution)
      - [Setting up Rust Nightly](#setting-up-rust-nightly)
      - [Setting up Visual Studio Code](#setting-up-visual-studio-code)
      - [Project Setup](#project-setup)
    - [Improving The Documentation](#improving-the-documentation)
      - [Documentation Guidelines](#documentation-guidelines)
      - [Building Documentation Locally](#building-documentation-locally)
      - [Documentation Best Practices](#documentation-best-practices)
    - [Commit Messages](#commit-messages)
      - [Format](#format)
      - [Guidelines](#guidelines)
      - [Valid Types](#valid-types)
      - [Scope Examples](#scope-examples)
      - [Examples](#examples)
  - [Attribution](#attribution)

## Code of Conduct

This project and everyone participating in it is governed by the
[Wit OwO Code of Conduct](https://github.com/cliftontoaster-reid/wit_owo/blob//CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behaviour
to <clifton.toaster.reid@proton.me>.

## I Have a Question

> If you want to ask a question, we assume that you have read the available [Documentation](https://github.com/cliftontoaster-reid/wit_owo).

Before you ask a question, it is best to search for existing [Issues](https://github.com/cliftontoaster-reid/wit_owo/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/cliftontoaster-reid/wit_owo/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions depending on what seems relevant.

We will then take care of the issue as soon as possible.

<!--
You might want to create a separate issue tag for questions and include it in this description. People should then tag their issues accordingly.

Depending on how large the project is, you may want to outsource the questioning, e.g. to Stack Overflow or Gitter. You may add additional contact and information possibilities:
- IRC
- Slack
- Gitter
- Stack Overflow tag
- Blog
- FAQ
- Roadmap
- E-Mail List
- Forum
-->

## I Want To Contribute

> ### Legal Notice <!-- omit in toc -->
>
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project licence.

### Reporting Bugs

<!-- omit in toc -->

#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that you have read the [documentation](https://github.com/cliftontoaster-reid/wit_owo). If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/cliftontoaster-reid/wit_owo/issues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback)
  - OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
  - Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
  - Possibly your input and the output
  - Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->

#### How Do I Submit a Good Bug Report?

> You must never report security related issues, vulnerabilities or bugs including sensitive information to the issue tracker, or elsewhere in public. Instead, sensitive bugs must be sent by email to <clifton.toaster.reid@proton.me>.

<!-- You may add a PGP key to allow the messages to be sent encrypted as well. -->

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open a [Bug Report](https://github.com/cliftontoaster-reid/wit_owo/issues/new/choose) using our structured bug report template. This template will guide you through providing all the necessary information for a complete bug report.
- Explain the behaviour you would expect and the actual behaviour.
- Please provide as much context as possible and describe the _reproduction steps_ that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for Wit OwO, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->

#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Read the [documentation](https://github.com/cliftontoaster-reid/wit_owo) carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/cliftontoaster-reid/wit_owo/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If you're just targeting a minority of users, consider writing an add-on/plugin library.

<!-- omit in toc -->

#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/cliftontoaster-reid/wit_owo/issues). Please use our [Feature Request template](https://github.com/cliftontoaster-reid/wit_owo/issues/new/choose) which will guide you through providing all the necessary information for a complete enhancement suggestion.

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behaviour** and **explain which behaviour you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- You may want to **include screenshots or screen recordings** which help you demonstrate the steps or point out the part which the suggestion is related to. You can use [LICEcap](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and the built-in [screen recorder in GNOME](https://help.gnome.org/users/gnome-help/stable/screen-shot-record.html.en) or [SimpleScreenRecorder](https://github.com/MaartenBaert/ssr) on Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most Wit OwO users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

### Your First Code Contribution

To get started with your first contribution, you'll need to set up Rust Nightly and Visual Studio Code on your system. Here are the setup instructions for each platform:

#### Setting up Rust Nightly

**Windows (PowerShell):**

```powershell
# Download and install rustup
Invoke-WebRequest -Uri https://sh.rustup.rs -UseBasicParsing | Invoke-Expression

# Set nightly as default toolchain
rustup default nightly

# Add required components
rustup component add rust-src rust-analyzer clippy rustfmt
```

**macOS:**

```bash
# Install rustup using the official installer
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Restart your terminal or source the environment
source ~/.cargo/env

# Set nightly as default toolchain
rustup default nightly

# Add required components
rustup component add rust-src rust-analyzer clippy rustfmt
```

**Linux:**

```bash
# Install rustup using the official installer
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Restart your terminal or source the environment
source ~/.cargo/env

# Set nightly as default toolchain
rustup default nightly

# Add required components
rustup component add rust-src rust-analyzer clippy rustfmt
```

#### Setting up Visual Studio Code

**Windows:**

1. Download VS Code from [https://code.visualstudio.com](https://code.visualstudio.com)
2. Run the installer and follow the setup wizard
3. Install the required extensions:

- Open VS Code
- Go to the Extensions view (Ctrl+Shift+X)
- Search for **rust-analyzer** and install the "Rust Analyzer" extension
- Search for **CodeLLDB (vadimcn.vscode-lldb)** and install the debugging extension
- Search for **Even Better TOML (tamasfe.even-better-toml)** and install the TOML support extension

**macOS:**

Option 1 - Manual Installation:

1. Download VS Code from [https://code.visualstudio.com](https://code.visualstudio.com)
2. Drag the application to your Applications folder

Option 2 - Using Homebrew:

```bash
brew install --cask visual-studio-code
```

After installing VS Code using either method, install the required extensions:

```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension tamasfe.even-better-toml
```

**Linux:**

Option 1 - Using Snap:

```bash
sudo snap install code --classic
```

Option 2 - Using package manager (Ubuntu/Debian):

```bash
wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg
sudo install -o root -g root -m 644 packages.microsoft.gpg /etc/apt/trusted.gpg.d/
sudo sh -c 'echo "deb [arch=amd64,arm64,armhf signed-by=/etc/apt/trusted.gpg.d/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main" > /etc/apt/sources.list.d/vscode.list'
sudo apt update
sudo apt install code
```

Then install the required extensions:

```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension tamasfe.even-better-toml
```

#### Project Setup

1. Fork the repository on GitHub
2. Clone your fork:

   ```bash
   git clone https://github.com/YOUR_USERNAME/wit_owo.git
   cd wit_owo
   ```

3. Open the project in VS Code:

   ```bash
   code .
   ```

4. Build the project to ensure everything is working:

   ```bash
   cargo build
   ```

5. Run the tests:

   ```bash
   cargo test
   ```

You're now ready to start contributing to the Wit OwO project!

### Improving The Documentation

All documentation for this project is built automatically from comments in the source code using Rust's built-in documentation system (`rustdoc`). This means that the public API documentation you see online is generated directly from the comments on functions, structs, enums, traits, and their fields/methods.

#### Documentation Guidelines

**Function Documentation:**

````rust
/// Performs speech recognition on the provided audio data.
///
/// This function takes raw audio data and converts it to text using
/// the Wit.ai speech recognition API.
///
/// # Arguments
///
/// * `audio_data` - A byte slice containing the audio data to transcribe
/// * `content_type` - The MIME type of the audio (e.g., "audio/wav")
///
/// # Returns
///
/// Returns a `Result` containing the transcribed text on success,
/// or an error if the recognition fails.
///
/// # Examples
///
/// ```rust
/// use wit_owo::transcribe_audio;
///
/// let audio = include_bytes!("../assets/test.wav");
/// let result = transcribe_audio(audio, "audio/wav").await?;
/// println!("Transcribed: {}", result);
/// ```
pub async fn transcribe_audio(audio_data: &[u8], content_type: &str) -> Result<String, Error> {
    // Implementation here
}
````

**Struct Documentation:**

```rust
/// Configuration for the Wit.ai client.
///
/// This struct holds all the necessary configuration options for
/// communicating with the Wit.ai API, including authentication
/// and request parameters.
pub struct WitConfig {
    /// The API token for authenticating with Wit.ai
    pub api_token: String,

    /// The base URL for the Wit.ai API (defaults to official endpoint)
    pub base_url: String,

    /// Timeout duration for API requests in seconds
    pub timeout_seconds: u64,
}
```

**Enum Documentation:**

```rust
/// Represents different types of errors that can occur during API operations.
///
/// This enum categorizes errors to help users handle different failure
/// scenarios appropriately.
#[derive(Debug, Clone)]
pub enum WitError {
    /// Network-related errors (connection failures, timeouts, etc.)
    Network(String),

    /// Authentication errors (invalid token, insufficient permissions)
    Authentication(String),

    /// API response parsing errors
    ParseError(String),
}
```

#### Building Documentation Locally

To build and view the documentation locally:

```bash
# Generate documentation for the project and its dependencies
cargo doc --open

# Generate documentation for just this crate
cargo doc --no-deps --open

# Generate documentation with private items included
cargo doc --document-private-items --open
```

#### Documentation Best Practices

1. **Always document public APIs**: Every public function, struct, enum, trait, and module should have documentation
2. **Use proper formatting**: Follow Rust's documentation conventions with `///` for outer docs and `//!` for module-level docs
3. **Include examples**: Provide code examples in doc comments using triple backticks with `rust` language annotation
4. **Document parameters and return values**: Use the `# Arguments` and `# Returns` sections for clarity
5. **Add error information**: Use `# Errors` section to document when and why functions might fail
6. **Link related items**: Use square brackets `[ItemName]` to create links to other documented items
7. **Test your examples**: Documentation examples are automatically tested by `cargo test`

When you contribute documentation improvements, they will automatically appear in the generated documentation when the project is built.

### Commit Messages

We welcome the use of AI tools to help write commit messages if you're not comfortable crafting them yourself. Whether written by you or with AI assistance, all commit messages should follow our standard format to maintain consistency across the project.

#### Format

```text
type(scope): title

msg
```

#### Guidelines

- **First line (title)**: Should not exceed 80 characters (this is a suggestion, not a hard limit)
- **Type**: Must be one of the predefined types listed below
- **Scope**: Optional. Include only if the change is focused on a specific area. If the change is too broad, omit the scope
- **Message**: Provide additional context if needed on subsequent lines

#### Valid Types

- `fix`: Bug fixes
- `feat`: New features
- `chore`: Maintenance tasks, dependency updates, etc.
- `docs`: Documentation changes
- `fmt`: Code formatting changes
- `refactor`: Code refactoring without functional changes
- `test`: Adding or updating tests
- `perf`: Performance improvements
- `ci`: Changes to CI/CD configuration
- `build`: Changes to build system or dependencies

#### Scope Examples

- `dictation`: Changes to dictation-related structs or APIs
- `api`: General API changes
- `client`: Client implementation changes
- `error`: Error handling improvements
- `utils`: Utility function changes

#### Examples

**With scope:**

```text
fix(dictation): handle empty audio responses correctly

Previously, empty responses from the API would cause a panic.
Now we return a proper error with context about the empty response.
```

**Without scope (broad change):**

```text
chore: update all dependencies to latest versions

Updated tokio, serde, and other core dependencies.
Verified all tests still pass after the updates.
```

**Simple change:**

```text
fmt: apply rustfmt to all source files
```

**AI-assisted example:**

```text
feat(api): add timeout configuration for requests

Allows users to specify custom timeout values for API calls.
Defaults to 30 seconds if not specified.
```

Feel free to use AI tools like GitHub Copilot, ChatGPT, or others to help generate commit messages that follow this format. The important thing is that the final message clearly communicates what was changed and why.

## Attribution

This guide is based on the [contributing.md](https://contributing.md/generator)!
