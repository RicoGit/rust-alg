pub mod dynamic_prog;
pub mod gale_shapley;
pub mod leetcode;
pub mod sock_merchant;
pub mod utils;

fn main() {
    println!("Sock merchant!");
    sock_merchant::run();
}
