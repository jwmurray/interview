// Idiomatic Rust code using iter(), filter(), map() and collect().
pub fn idiomatic(ids: &Vec<u64>, id: u64) -> Vec<u64> {
    let result = ids
        .iter()
        .filter(|&&element| element != id) // Dereference the element directly in the filter
        .map(|&element| element) // Dereference the element to get an owned value
        .collect(); // Collect the filtered elements into a new vector
    result
}

// Idiomatic rust using reservation to preallocate memory for the result vector
pub fn idiomatic_with_reservation(ids: &Vec<u64>, id: u64) -> Vec<u64> {
    let mut result = Vec::with_capacity(ids.len()); // Preallocate memory
    ids.iter()
        .filter(|&element| element != &id) // Filter out elements matching `id`
        .for_each(|element| result.push(*element)); // Push elements into the preallocated vector
    result
}

// foo version from the interview session
pub fn foo(ids: &Vec<u64>, id: u64) -> Vec<u64> {
    // this is basically the final foo function from the interview session
    let mut elements_to_be_kept = Vec::with_capacity(ids.len());
    for element in ids {
        if element != &id {
            elements_to_be_kept.push(*element);
        }
    }
    elements_to_be_kept
}

/// Generates a vector of random u64 numbers between 0 and 9 inclusive
pub fn get_random_ids_vector(length: u64) -> Vec<u64> {
    println!("Generating random IDs vector of length: {}", length);
    // Generate random numbers between 0 and 9 inclusive
    let result = (0..length).map(|_| rand::random::<u64>() % 10).collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_sample_input_vector() -> Vec<u64> {
        vec![6, 6, 1, 2, 3, 6, 4, 5]
    }

    fn setup_large_input_vector() -> Vec<u64> {
        get_random_ids_vector(1_000_000)
    }

    #[test]
    fn test_generate_random_ids_vector_length() {
        let length = 10;
        let ids = get_random_ids_vector(length);
        assert_eq!(
            ids.len(),
            length as usize,
            "Generated vector should have the correct length"
        );
    }

    #[test]
    fn test_idiomatic() {
        let ids = setup_sample_input_vector();
        let id = 6;
        let result = idiomatic(&ids, id);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_foo() {
        let ids = setup_sample_input_vector();
        let id = 6;
        let result = foo(&ids, id);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_compare_foo_to_idiomatic() {
        let ids = setup_large_input_vector();
        let id = 6;
        let result_foo = foo(&ids, id);
        let result_idiomatic = idiomatic(&ids, id);
        assert_eq!(result_foo, result_idiomatic);
    }

    #[test]
    fn test_compare_foo_to_idiomatic_sample_vector() {
        let ids = setup_sample_input_vector();
        let id = 6;
        let result_foo = foo(&ids, id);
        let result_idiomatic = idiomatic(&ids, id);
        assert_eq!(result_foo, result_idiomatic);
    }
}
