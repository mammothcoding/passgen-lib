use passgenlib::Passgen;

fn main() {
    println!("{}", Passgen::default_strong_and_usab().generate(8));
    println!("{}", Passgen::default().generate(8));
    println!(
        "{}",
        Passgen::default()
            .set_custom_charset("bla@321.")
            .generate(8)
    );
}
