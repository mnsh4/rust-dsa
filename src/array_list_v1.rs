/*

 * Wrapper that uses arrays under the hood
 * push/pop/access has O(1)
 * enqueue/deque has O(N)
 * constructor specifies initial size


TypeScript example:
export default class ArrayList<T> {
    public length: number;
    constructor() {}
    prepend(item: T): void {}
    insertAt(item: T, idx: number): void {}
    append(item: T): void {}
    remove(item: T): T | undefined {}
    get(idx: number): T | undefined {}
    removeAt(idx: number): T | undefined {}
}
 */

trait ArrayList<T> {
    fn length(&self) -> usize;
}

struct MyArrayList<T> {
    data: Vec<T>,
}

impl<T> ArrayList<T> for MyArrayList<T> {
    fn length(&self) -> usize {
        self.data.len()
    }

    fn new() -> ArrayList<T> {
        ArrayList {
            length: 0,
            inner: vec![T::default(); 5],
        }
    }
}

fn main() {
    let my_list = MyArrayList {
        data: vec![1, 2, 3, 5],
    };
    println!("Length: {}", my_list.length());
}
