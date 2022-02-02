pub struct Node {
  pub count: usize,
  pub children: [Option<usize>; 2],
}
impl Node {
  pub fn new() -> Self {
      Self {
          count: 0,
          children: [None, None],
      }
  }
}
