fn main() {
    let mut var = 5;
    let func = test;
    
    func(&mut var);
    
    print!("remplaced var: {}", var);
    
}

fn test(x: &mut i32) -> () {
    return *x = 10;
}