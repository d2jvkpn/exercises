fn main() {
    println!("Hello, wrold!");
}

struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    // fn new() -> Self{}
    // fn push(&mut self, val: T) {}
    // fn pop(&mut self) Option<T> {}
    // fn peek(&self) Option<T> {}
    // fn is_empty(&self) bool {}
    // fn size(&self) usize {}
}

/*
  ()()()()()
  (()(()))
  n/2/2/2/2/2 interger to literal binary
  A+B*C => (A+(B*C)) [A, B, C], [(, +, (, *, ), )], + A * B C, A B C * +
*/
