pub mod gen_engine {
    use crate::Passgen;
    use rand::{Rng, SeedableRng};
    use rand_hc::Hc128Rng;
    use rand_isaac::Isaac64Rng;

    // Letters charset.
    const LETTERS_CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";
    // Capital letters charset.
    const U_LETTERS_CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // Numeric charset.
    const NUM_CHARSET: &str = "0123456789";
    // Special symbols charset.
    const SPEC_SYMB_CHARSET: &str = ")([]{}*&^%$#@!~";
    // Simple special charset without inconvenient symbols.
    const SIMP_SYMB_CHARSET: &str = "*&%$#@!";
    // Strong & usability charset.
    // Set without ambiguous and inconvenient letters with numbers.
    const STRONG_USAB_CHARSET: &str = "ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789";
    // Strong & usability letters charset.
    // Set without ambiguous and inconvenient letters.
    const STRONG_USAB_LETTERS_CHARSET: &str = "ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz";

    impl Passgen {
        pub(crate) fn generate_pass(&mut self, res_len: u32) -> String {
            let mut isaac_seeder = Isaac64Rng::from_os_rng();
            let mut rng = Hc128Rng::from_rng(&mut isaac_seeder);
            let mut pass_assembly: Vec<char> = Vec::new();

            if self.custom_charset.len() != 0 {
                let mut cc_vec: Vec<char> = self.custom_charset.chars().into_iter().collect();
                pass_assembly.append(&mut cc_vec);
            } else if self.enab_strong_usab
                || (!self.enab_letters
                    && !self.enab_u_letters
                    && !self.enab_num
                    && !self.enab_spec_symbs)
            {
                let mut suc_vec: Vec<char> = STRONG_USAB_CHARSET.chars().into_iter().collect();
                    pass_assembly.append(&mut suc_vec);
            } else {
                if self.enab_letters {
                    let mut el_vec: Vec<char> = LETTERS_CHARSET.chars().into_iter().collect();
                    pass_assembly.append(&mut el_vec);
                }
                if self.enab_u_letters {
                    let mut eul_vec: Vec<char> = U_LETTERS_CHARSET.chars().into_iter().collect();
                    pass_assembly.append(&mut eul_vec);
                }
                if self.enab_num {
                    let mut en_vec: Vec<char> = NUM_CHARSET.chars().into_iter().collect();
                    pass_assembly.append(&mut en_vec);
                }
                if self.enab_spec_symbs {
                    let mut ess_vec: Vec<char> = SPEC_SYMB_CHARSET.chars().into_iter().collect();
                    pass_assembly.append(&mut ess_vec);
                }
            }

            let mut pass_candidate_vec: Vec<char> = Vec::new();

            if self.enab_strong_usab {
                let letters_charset: Vec<char> =
                    STRONG_USAB_LETTERS_CHARSET.chars().into_iter().collect();
                let simp_symb_charset: Vec<char> = SIMP_SYMB_CHARSET.chars().into_iter().collect();

                // gen first pass symbol from all letters
                pass_candidate_vec.push(letters_charset[rng.random_range(0..letters_charset.len())]);

                // gen main pass body
                for _ in 0..(res_len - 2) {
                    pass_candidate_vec.push(pass_assembly[rng.random_range(0..pass_assembly.len())]);
                }

                // gen last pass symbol from simple symbols
                pass_candidate_vec
                    .push(simp_symb_charset[rng.random_range(0..simp_symb_charset.len())]);
            } else {
                pass_candidate_vec = (0..res_len)
                    .map(|_| pass_assembly[rng.random_range(0..pass_assembly.len())])
                    .collect();
            }
            String::from_iter(pass_candidate_vec)
        }

        pub(crate) fn is_valid_pwd_by_consist(&self, pass: String) -> bool {
            let check_to_available_for = |symbols: &str| -> bool {
                let mut res = false;
                for ch in pass.chars() {
                    if symbols.contains(ch) {
                        res = true;
                        break;
                    }
                }
                res
            };

            // compliance check
            if self.enab_letters || self.enab_strong_usab {
                if !check_to_available_for(LETTERS_CHARSET) {
                    return false;
                }
            }
            if self.enab_u_letters || self.enab_strong_usab {
                if !check_to_available_for(U_LETTERS_CHARSET) {
                    return false;
                }
            }
            if self.enab_num || self.enab_strong_usab {
                if !check_to_available_for(NUM_CHARSET) {
                    return false;
                }
            }
            if self.enab_spec_symbs || self.enab_strong_usab {
                if !check_to_available_for(SPEC_SYMB_CHARSET) {
                    return false;
                }
            }
            true
        }
    }
}
