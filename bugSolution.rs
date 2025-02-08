fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Correct way to get the last element using pop()
    if let Some(last) = vec.pop() {
        println!("Last element: {}", last);
    } else {
        println!("Vector is empty");
    }
} 