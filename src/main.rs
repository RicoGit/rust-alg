pub mod dynamic_prog;
pub mod gale_shapley;
pub mod leetcode;
pub mod sock_merchant;
pub mod utils;
pub mod codility;
pub mod computational_graph;

fn main() {
    println!("Sock merchant!");
    sock_merchant::run();
}
