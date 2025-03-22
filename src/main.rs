use interview::{foo, get_random_ids_vector, idiomatic, idiomatic_with_reservation};
use std::time::Instant; // Assuming coderpad is a module containing the functions to be tested

type ExtractIdsFunction = fn(&Vec<u64>, u64) -> Vec<u64>;

// fn test_runner()
fn main() {
    let big_vector = get_random_ids_vector(1_000_000_000);
    let id_to_remove = 6;

    // Define an array of function pointers, with foo first
    let functions: [(&str, ExtractIdsFunction); 3] = [
        ("Foo", foo),
        ("Idiomatic", idiomatic),
        ("Idiomatic with reservation", idiomatic_with_reservation),
    ];

    println!("Running performance tests ...");
    // Store the result of the first function for comparison
    let mut reference_result: Option<Vec<u64>> = None;
    // Iterate over the functions and measure their execution times
    for (name, func) in functions.iter() {
        let start = Instant::now();
        let result = func(&big_vector, id_to_remove);
        let duration = start.elapsed();

        // Print the duration in seconds with 3 decimal places
        println!(
            "\t{:.3} seconds for {} version",
            duration.as_secs_f64(),
            name,
        );

        // Ensure results match
        if let Some(ref ref_result) = reference_result {
            assert_eq!(result, *ref_result, "{} version did not match!", name);
        } else {
            reference_result = Some(result); // save reference result
        }
    }

    // Define an array of function pointers, with foo first
    let functions: [(&str, ExtractIdsFunction); 3] = [
        ("Idiomatic with reservation", idiomatic_with_reservation),
        ("Idiomatic", idiomatic),
        ("Foo", foo),
    ];

    println!("\nRunning performance tests ...");
    // Store the result of the first function for comparison
    let mut reference_result: Option<Vec<u64>> = None;
    // Iterate over the functions and measure their execution times
    for (name, func) in functions.iter() {
        let start = Instant::now();
        let result = func(&big_vector, id_to_remove);
        let duration = start.elapsed();

        // Print the duration in seconds with 3 decimal places
        println!(
            "\t{:.3} seconds for {} version",
            duration.as_secs_f64(),
            name,
        );

        // Ensure results match
        if let Some(ref ref_result) = reference_result {
            assert_eq!(result, *ref_result, "{} version did not match!", name);
        } else {
            reference_result = Some(result); // save reference result
        }
    }
}
