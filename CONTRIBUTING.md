# Contributing to Dintero Rust SDK

Thank you for your interest in contributing to the Dintero Rust SDK! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and constructive in all interactions with the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/dintero-rust-sdk.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes: `cargo test --all-features`
6. Format your code: `cargo fmt`
7. Check for linting issues: `cargo clippy --all-features --all-targets`
8. Commit your changes: `git commit -m "Description of changes"`
9. Push to your fork: `git push origin feature/your-feature-name`
10. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build --all-features
```

### Testing

Run all tests:

```bash
cargo test --all-features
```

Run tests for a specific crate:

```bash
cargo test -p dintero-checkout
```

### Linting

```bash
cargo clippy --all-features --all-targets
```

### Formatting

```bash
cargo fmt
```

## Code Style

- Follow the existing code style in the repository
- Use the provided `rustfmt.toml` configuration
- Maximum line width is 100 characters
- Use meaningful variable and function names
- Keep functions focused and concise
- Use builder patterns where appropriate
- Minimize unnecessary comments - code should be self-documenting
- Add Rust doc comments for public APIs

## Documentation

- All public APIs must have Rust doc comments
- Include examples in doc comments where appropriate
- Update README.md if adding new features
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format

## Pull Request Guidelines

- Keep PRs focused on a single feature or fix
- Write clear, descriptive commit messages
- Include tests for new functionality
- Update documentation as needed
- Ensure all tests pass
- Ensure code is formatted and passes clippy
- Reference any related issues

## Commit Message Format

Use clear and descriptive commit messages:

```
Add feature X to checkout module

- Implement new endpoint Y
- Add tests for feature X
- Update documentation
```

## Testing Guidelines

- Write unit tests for new functionality
- Include integration tests where appropriate
- Mock external API calls in tests
- Ensure tests are deterministic and don't rely on external state

## Adding New Features

When adding a new feature:

1. Check if it fits into an existing module or needs a new one
2. Follow the existing patterns in the codebase
3. Use builder patterns for complex types
4. Add comprehensive documentation
5. Include examples
6. Write tests

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- OS and version
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Any relevant error messages or logs

## Questions?

If you have questions, feel free to open an issue for discussion.

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License.
