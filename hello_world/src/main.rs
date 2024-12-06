fn data_paralelism_rayon() {

    use rayon::prelude::*;

    let mut data = vec![1, 2, 3, 4, 5];

    data.par_iter_mut().for_each(|x| {
        *x *= 2;
    });
    
    // from code perspective it seems trivial, but I want you to realize how much heavy lifting happens behind the hood:
    //The Rayon library uses work stealing to dynamically balance the workload among threads, 
    //providing better performance compared to a static division of work among threads.
    
    // on top it creates a separate scope to escape need to lock data
    
    
    
    // Concept of work stealing and separate scope.
    
    

    println!("{:?}", data); // [2, 4, 6, 8, 10]
}

fn main() {
    data_paralelism_rayon();
}
