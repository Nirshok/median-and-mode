use std::collections::HashMap;

fn main() {
    let vector = vec![1, 9, 9, 3, 5, 4];
    let med = median_and_mod(vector);
    dbg!{med};
}

fn median_and_mod(mut list: Vec<i32>) -> (i32, i32) {
    // Sort the list, get it's lenght and find the median
    list.sort();
    let last = list.len();
    let med = list[last / 2];
    
    // Create an empty HashMap
    let mut hmap =  HashMap::new();
    
    // Iterate over items in list, use them as keys
    // and count increases their values
    // depending on how much times they've appeared
    for i in list {
        let count = hmap.entry(i).or_insert(0);
        // Dereferrencing count to access it's value rather than pointer
        *count += 1
    }
    // Finding the maximum value by turning HashMap into iter
    
    let max = hmap.into_iter()
                        // Get the biggest value 
                        .max_by_key(|(_, v)| *v)
                        // And return it's key
                        .map(|(k, _)| k)
                        .expect("Error, no values");
    // Return a tuple with median and mode
    (med, max)
}
