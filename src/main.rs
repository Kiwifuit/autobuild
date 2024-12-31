fn main() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Hello, world!");

    println!("You can close this program by pressing CTRL + C");
    #[allow(clippy::empty_loop)]
    loop {}
}
