fn main() {
    color_eyre::install().unwrap();
    if atty::is(atty::Stream::Stdout) {
        println!("i'm a tty");
    }
    println!("Hello, world!");
}
