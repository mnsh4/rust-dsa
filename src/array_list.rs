use std::fmt::Debug;

type Array<T> = Vec<T>;

// #[derive(Debug)]
struct ArrayList<T> {
    pub length: usize,
    inner: Array<T>,
}

impl<T: Debug + Default + Clone + PartialEq> ArrayList<T> {
    pub fn new() -> ArrayList<T> {
        ArrayList {
            length: 0,
            inner: vec![T::default(); 5],
        }
    }
}

fn main() {
    let my_list: ArrayList<i32> = ArrayList::new(); // Crear una nueva instancia de ArrayList<i32>
    println!("Length: {}", my_list.length);
}
