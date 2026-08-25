//! SAGE semantic analysis.

// [TECH] This crate is the semantic-analysis boundary: it will resolve names and validate meaning
// after parsing, without making runtime or backend decisions; the current crate is skeletal.
// [ELI5] This is where the compiler checks that the written pieces make sense together, rather
// than merely looking like valid syntax.
