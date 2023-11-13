pub mod codility;
pub mod computational_graph;
pub mod dynamic_prog;
pub mod gale_shapley;
pub mod leetcode;
pub mod sock_merchant;
pub mod utils;
mod hlist;

fn main() {
    println!("Sock merchant!");
    sock_merchant::run();
}
