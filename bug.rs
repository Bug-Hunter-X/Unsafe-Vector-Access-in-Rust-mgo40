fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This is incorrect way to get the last element.
    let last_element = vec.get(vec.len() - 1).copied().unwrap_or(0);
    println!("Last element: {}", last_element);

    // Correct way to get the last element using pop()
    if let Some(last) = vec.pop() {
        println!("Last element (correct): {}", last);
    } else {
        println!("Vector is empty");
    }
}