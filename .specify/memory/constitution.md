<!--
Sync Impact Report:
Version change: old → 1.0.0
List of modified principles: All principles redefined to align with user requirements (Clean Code, No Testing, Rust, Loco, Sea-ORM)
Added sections: Technology Stack, Development Workflow
Removed sections: None
Templates requiring updates: plan-template.md (Constitution Check updated to prohibit testing), spec-template.md (removed testing sections), tasks-template.md (removed all test tasks)
Follow-up TODOs: None
-->

# xsia_loco Constitution

## Core Principles

### Clean Code
All code must adhere to clean code principles: readable, maintainable, no code duplication, clear naming, small functions, and proper abstraction. Code reviews must enforce these standards.

### No Testing
Absolutely no unit tests, integration tests, or end-to-end tests. Testing is prohibited and supersedes any other guidance.

### Rust Language
The project must use Rust programming language exclusively for all development.

### Loco Web Framework
Use Loco web framework for web development as specified in Cargo.toml.

### Sea-ORM
Use Sea-ORM for database interactions as specified in Cargo.toml.

## Technology Stack

Technology stack requirements: Rust, Loco web framework, Sea-ORM, and dependencies listed in Cargo.toml. No other languages or frameworks allowed.

## Development Workflow

Code must be written following clean code principles. No testing phases or gates. Focus on direct implementation and code quality through reviews.

## Governance

Constitution supersedes all other guidance. All code must comply with these principles. Amendments require consensus and documentation.

**Version**: 1.0.0 | **Ratified**: TODO(RATIFICATION_DATE): Original adoption date unknown | **Last Amended**: 2025-10-17
