//! SAGE syntax definitions.

// [TECH] This crate owns the source-facing syntax model, including AST data and source spans;
// syntax nodes should preserve enough provenance for later diagnostics.
// [ELI5] This is the compiler's record of what the programmer wrote, with locations attached so
// later stages can explain problems in the original file.
