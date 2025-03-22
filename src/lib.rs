// Updated foo function to use idiomatic Rust with into_iter().
// Even so, the returned vector is a new one, and the original vector is left unchanged,
// matching the logic of the foo() function below.
pub fn extract_non_matching_ids_idiomatically(ids: Vec<u64>, id: u64) -> Vec<u64> {
    ids.into_iter()
        .filter(|&element| element != id) // filter out id matches
        .collect() // collect elements into a new Vec matching the return type and return
}

pub fn foo(ids: Vec<u64>, id: u64) -> Vec<u64> {
    // this is basically the final foo function from the interview session
    let mut elements_to_be_kept = Vec::with_capacity(ids.len());
    for element in ids {
        if element != id {
            elements_to_be_kept.push(element);
        }
    }
    elements_to_be_kept
}

pub fn foo_in_place(mut ids: Vec<u64>, id: u64) -> Vec<u64> {
    let mut i = 0; // C
    while i < ids.len() {
        // N
        if let Some(val) = ids.get(i) {
            // C
            if *val == id {
                // C
                ids.remove(i); // N/2 -> N
                continue;
            }
        }
        i += 1;
    }

    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_ids_vector() -> Vec<u64> {
        vec![6, 6, 1, 2, 3, 6, 4, 5]
    }

    #[test]
    fn test_foo_with_vector_with_repeating_targets() {
        let ids = setup_ids_vector();
        let id = 6;
        let result = extract_non_matching_ids_idiomatically(ids, id);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_foo() {
        let ids = setup_ids_vector();
        let id = 6;
        let result = foo(ids, id);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}
