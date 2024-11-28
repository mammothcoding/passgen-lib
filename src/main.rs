use rand::{Rng, SeedableRng};
use rand_isaac::Isaac64Rng;
use rand_seeder::Seeder;
use passgenlib::Passgen;
use rand::distributions::Alphanumeric;
use rand_seeder::rand_core::RngCore;

fn main() {
    /*println!("{}", Passgen::default_strong_and_usab().generate(8));
    println!("{}", Passgen::default().generate(8));
    println!(
        "{}",
        Passgen::default()
            .set_custom_charset("bla@321.")
            .generate(8)
    );*/


    /*(0..res_len)
                    .map(|_| pass_charset[Isaac64Rng::seed_from_u64(pass_charset.len() as u64).gen::<usize>()])
                    .collect()*/
    /*for _ in 0..(res_len) {
        pass_candidate_vec.push(pass_charset[Isaac64Rng::seed_from_u64(pass_charset.len() as u64).gen::<usize>()]);
    }
    String::from_utf8(pass_candidate_vec).unwrap()*/


    const STRONG_USAB_LETTERS_CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz";
    let mut pass_candidate_vec: Vec<u8> = Vec::new();
    let letters_charset: Vec<u8> =
        STRONG_USAB_LETTERS_CHARSET.into_iter().cloned().collect();

    //pass_candidate_vec.push([Isaac64Rng::from_seed(Seeder::from(letters_charset)).gen()]);

    //let mut rng: Isaac64Rng = Seeder::from("ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz").make_rng();
    //println!("{}", rng.gen_range(0 .. 100));
    //let mut rng = Isaac64Rng::seed_from_u64(12343534534535);//.gen::<usize>();
    let mut rng = Isaac64Rng::from_entropy();
    println!("{}", rng.gen_range(0 .. 100));
    println!("{}", rng.next_u64());
    println!("{}", rng.next_u64());

}
