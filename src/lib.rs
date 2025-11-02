pub fn match_pattern(pattern: &str, path: &str) -> bool {
    match_recursive(pattern.chars().collect(), path.chars().collect(), 0, 0)
}

fn match_recursive(pattern: Vec<char>, path: Vec<char>, p_idx: usize, path_idx: usize) -> bool {
    // If we've reached the end of both pattern and path, it's a match
    if p_idx >= pattern.len() && path_idx >= path.len() {
        return true;
    }

    // If pattern is exhausted but path isn't, no match
    if p_idx >= pattern.len() {
        return false;
    }

    let current_char = pattern[p_idx];

    match current_char {
        '*' => {
            // * matches any sequence of characters except '/'
            // Try matching zero characters
            if match_recursive(pattern.clone(), path.clone(), p_idx + 1, path_idx) {
                return true;
            }

            // Try matching one or more characters (but not '/')
            let mut i = path_idx;
            while i < path.len() && path[i] != '/' {
                i += 1;
                if match_recursive(pattern.clone(), path.clone(), p_idx + 1, i) {
                    return true;
                }
            }
            false
        }
        '?' => {
            // ? matches zero or one of the preceding character
            if p_idx == 0 {
                return false; // ? can't be at the beginning
            }

            // Try matching zero occurrences of the preceding character
            // This means we skip both the preceding char and the ? in the pattern
            if match_recursive(pattern.clone(), path.clone(), p_idx + 1, path_idx) {
                return true;
            }

            // Try matching one occurrence of the preceding character
            // The preceding character should already be matched, so we just need to continue
            // after the ? without consuming any path characters
            false
        }
        _ => {
            // Check if the next character is '?' - if so, handle it specially
            if p_idx + 1 < pattern.len() && pattern[p_idx + 1] == '?' {
                // Current character might appear zero or one time

                // Try zero occurrences - skip both current char and ?
                if match_recursive(pattern.clone(), path.clone(), p_idx + 2, path_idx) {
                    return true;
                }

                // Try one occurrence - match current char and skip ?
                if path_idx < path.len() && path[path_idx] == current_char {
                    return match_recursive(pattern.clone(), path.clone(), p_idx + 2, path_idx + 1);
                }

                false
            } else {
                // Regular character matching
                if path_idx >= path.len() || path[path_idx] != current_char {
                    return false;
                }
                match_recursive(pattern.clone(), path.clone(), p_idx + 1, path_idx + 1)
            }
        }
    }
}
