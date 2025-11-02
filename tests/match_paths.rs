use glob_workflow_paths::match_paths;

#[test]
fn test_early_exit_doesnt_skip_critical_negations() {
    // Scenario: First path matches positive and first negation, but second negation should also be checked
    // If we exit too early on first negation match, we might miss the logic
    let patterns = &["*.md", "!README.md", "!GUIDE.md"];

    // This should be negated by the first negation pattern
    assert!(!match_paths(patterns, &["README.md"]));

    // This should be negated by the second negation pattern
    assert!(!match_paths(patterns, &["GUIDE.md"]));

    // Both negations should be processed for mixed paths
    assert!(!match_paths(patterns, &["README.md", "GUIDE.md"]));

    // Order shouldn't matter
    assert!(!match_paths(patterns, &["GUIDE.md", "README.md"]));
}

#[test]
fn test_early_exit_doesnt_skip_remaining_paths_after_negation() {
    // Scenario: First path gets negated, but second path should still be processed
    let patterns = &["*.md", "!README.md"];

    // README.md is negated, but guide.md should still match
    assert!(match_paths(patterns, &["README.md", "guide.md"]));

    // Order shouldn't matter
    assert!(match_paths(patterns, &["guide.md", "README.md"]));
}

#[test]
fn test_early_exit_with_multiple_positive_patterns() {
    // Scenario: First positive pattern doesn't match, but second should be checked
    let patterns = &["*.js", "*.md", "!README.md"];

    // Should match *.md pattern even though *.js doesn't match
    assert!(match_paths(patterns, &["guide.md"]));

    // Should match *.js pattern
    assert!(match_paths(patterns, &["app.js"]));

    // Both patterns should be considered before applying negations
    assert!(match_paths(patterns, &["app.js", "guide.md"]));
}

#[test]
fn test_negation_order_independence() {
    // Scenario: Negation patterns in different orders should give same result
    let patterns1 = &["*.md", "!README.md", "!CHANGELOG.md"];
    let patterns2 = &["*.md", "!CHANGELOG.md", "!README.md"];

    let test_paths = &["README.md", "CHANGELOG.md", "guide.md"];

    // Results should be identical regardless of negation order
    assert_eq!(match_paths(patterns1, test_paths), match_paths(patterns2, test_paths));

    // Specifically, guide.md should match in both cases
    assert!(match_paths(patterns1, &["guide.md"]));
    assert!(match_paths(patterns2, &["guide.md"]));
}

#[test]
fn test_partial_match_with_negation_doesnt_short_circuit() {
    // Scenario: Some paths match positive + get negated, others should still be processed
    let patterns = &["**/*.js", "!**/test/**"];

    let paths = &[
        "test/spec.js",   // Matches positive but negated by !**/test/**
        "src/app.js",     // Matches positive and not negated
        "docs/README.md", // Doesn't match positive
    ];

    // Should return true because src/app.js matches and isn't negated
    assert!(match_paths(patterns, paths));

    // Test with only negated files
    assert!(!match_paths(patterns, &["test/spec.js", "test/unit.js"]));
}

#[test]
fn test_complex_negation_interaction() {
    // Scenario: Multiple overlapping negations that could cause logic errors if not all processed
    let patterns = &["**/*.js", "!**/node_modules/**", "!**/test/**", "!**/dist/**"];

    // File that would be caught by multiple negations
    assert!(!match_paths(patterns, &["node_modules/test/spec.js"])); // Hit by both negations

    // Files caught by single negations
    assert!(!match_paths(patterns, &["node_modules/lib/index.js"]));
    assert!(!match_paths(patterns, &["test/unit.js"]));
    assert!(!match_paths(patterns, &["dist/bundle.js"]));

    // File that should match (not caught by any negation)
    assert!(match_paths(patterns, &["src/app.js"]));
}

#[test]
fn test_early_exit_correctness_with_empty_positive_patterns() {
    // Edge case: Only negation patterns (should return false without processing paths)
    let patterns = &["!*.tmp", "!*.log"];

    // Should return false immediately regardless of what paths are provided
    assert!(!match_paths(patterns, &["app.js"]));
    assert!(!match_paths(patterns, &["file.tmp"]));
    assert!(!match_paths(patterns, &["debug.log"]));
}

#[test]
fn test_optimization_doesnt_break_any_semantics() {
    // Comprehensive test ensuring fast path and slow path give same results for equivalent inputs

    // Test case that should use fast path (no negations)
    let fast_patterns = &["*.js", "*.md"];
    let paths = &["app.js", "README.md", "style.css"];
    let fast_result = match_paths(fast_patterns, paths);

    // Equivalent case that uses slow path (with dummy negation that won't match)
    let slow_patterns = &["*.js", "*.md", "!nonexistent.xyz"];
    let slow_result = match_paths(slow_patterns, paths);

    // Results should be identical
    assert_eq!(fast_result, slow_result);
    assert!(fast_result); // Both should be true
}
