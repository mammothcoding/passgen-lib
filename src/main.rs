use passgenlib::Passgen;

fn main() {
    println!("{}", Passgen::default_strong_and_usab().generate(8));
}
