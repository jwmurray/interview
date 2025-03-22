# Comparison of Rust code for extracting elements of a large vector:

The foo() function is similar to the interview result.
The idiomatic() uses an iter() block to collect the results.
The idiomatic_with_reservation() allocates the vector prior to running the iter().

I also tested this with into_iter().  I did not try it with an array, which might help.


```
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

```
## Performance Measurements

I moved the functions into lib.rs and put the testing code in main.rs.

I then ran some performance tests with a 1 Billion item array of intergers from 0-9, includsive.

### Increases in the idiomatic code
Adding reservation to the idimoatic version improved the performance consistently.

The idiomatic version with reservation consistently performs in the same range as the for loop from the interview.

### Assembler in godbolt
Comparing the assembler in Godbolt, the for loop version is slightly smaller.

### Caching
With a large array problem like this, caching efficiency and branch prediction will play large factors in performance.  I ran the code on an arm and also on an x86.  The x86 performed more consistently because the arm is my macbook, with lots of inconsistent background processes running.  Running this on my Raspberry pi would be an interewsting comparison.

At first, the idiomatic code could not come close to the for loop we wrote in the interview quesiton.  I changed the code to use more references in the arguments.  I also tried iter(), into_iter(), reservation, non-reservation, and varied the length of the input array.

I moved the functions into lib.rs and run the tests from main.rs.  

The main function runs the tests twice to vary the order to see if caching is playing a factor.

I could write some more tests.  I will probably stop here.  If I were writing production code, I would look into some more issues.

It would also be interesting to do some Rayon testing and to write a test function that would exercise rayon.

## Performance tests

### x86 Ubuntu server, 16 core, 64 GB RAM

```

~ cargo run --release
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/interview`
Generating random IDs vector of length: 1000000000
Running performance tests ...
	2.804 seconds for Foo version
	2.989 seconds for Idiomatic version
	2.799 seconds for Idiomatic with reservation version

Running performance tests ...
	2.835 seconds for Idiomatic with reservation version
	2.976 seconds for Idiomatic version
	2.791 seconds for Foo version
```


### MacOS, M2, 2023, 32 GB RAM -- very inconsistent results due to the nature of running on MacOS with other processes
```
	[jmurray@mac]~/code/rust/magnite/interview (main …6) 0 [11:14:56]
~ cargo run --release
    Finished `release` profile [optimized] target(s) in 0.06s
     Running `target/release/interview`
Generating random IDs vector of length: 1000000000
Running performance tests ...
	4.096 seconds for Foo version
	3.481 seconds for Idiomatic version
	3.229 seconds for Idiomatic with reservation version

Running performance tests ...
	3.397 seconds for Idiomatic with reservation version
	4.114 seconds for Idiomatic version
	3.456 seconds for Foo version

```

###  Raspberry Pi 5, Cortex-A76, aarch64, 4 core, 2.4 GHz, 8 GB RAM
With only 8 GB, it did not like using 1 Billion element arrays.  Dropping it down to 100 Million worked:

```
	[jmurray@raspberrypi]~/code/rust/magnite/interview (main ✚1) 0 [11:22:29]
~ cargo run --release
   Compiling interview v0.1.0 (/home/jmurray/code/rust/magnite/interview)
    Finished `release` profile [optimized] target(s) in 0.26s
     Running `target/release/interview`
Generating random IDs vector of length: 100000000
Running performance tests ...
	0.280 seconds for Foo version
	0.316 seconds for Idiomatic version
	0.291 seconds for Idiomatic with reservation version

Running performance tests ...
	0.291 seconds for Idiomatic with reservation version
	0.316 seconds for Idiomatic version
	0.280 seconds for Foo version
```

