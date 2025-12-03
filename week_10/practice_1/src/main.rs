fn main() {
    
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let _v2 = v.clone();
    /*here 2 variables own heap value,
    2 pointers to the same content is not allowed in rust*/

    /*Rust is very smart in terms of memory access, so it detects a race condition
    as 2 variables point to same heap*/

    println!("{:?}",v);
}