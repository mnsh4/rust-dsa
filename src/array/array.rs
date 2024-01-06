fn main() {
    let arr = vec!["Argentina", "Japon", "Paraguay"];
    println!("{:?}", arr);

    println!("{}", arr[0]);
    println!("{}", arr[1]);
    println!("{}", arr[2]);
  

    for (i, &v) in arr.iter().enumerate() {
        println!("Array Index: {} - Array Element: {}", i, v);
    }
    println!("Array Length: {}", arr.len());
}
