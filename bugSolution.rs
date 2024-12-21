fn main() {
    let mut x = 5;
    { // Use a separate scope to limit the lifetime of the mutable borrow 
        let y = &mut x; 
        *y = 10;
        println!("x = {}", x);
    }
    { // Use another separate scope 
        let z = &mut x;
        *z = 15;
        println!("x = {}", x);
    }
    
} 