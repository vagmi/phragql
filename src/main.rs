#[tokio::main]
async fn main() {
  println!("the answer is {}", libphragql::add(40, 2))
}