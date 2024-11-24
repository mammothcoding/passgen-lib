pub mod gen_engine {
    use rand::Rng;
    use crate::Passgen;

    // Letters charset.
    const LETTERS_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    // Capital letters charset.
    const U_LETTERS_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // Numeric charset.
    const NUM_CHARSET: &[u8] = b"0123456789";
    // Special symbols charset.
    const SPEC_SYMB_CHARSET: &[u8] = b")([]{}*&^%$#@!~";
    // Simple special charset without inconvenient symbols.
    const SIMP_SYMB_CHARSET: &[u8] = b"*&%$#@!";
    // Strong & usability charset.
    // Set without ambiguous and inconvenient letters with numbers.
    const STRONG_USAB_CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789";
    // Strong & usability letters charset.
    // Set without ambiguous and inconvenient letters.
    const STRONG_USAB_LETTERS_CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz";

    impl Passgen {
        pub(crate) fn generate_pass(&mut self) -> String {
            let mut rng = rand::thread_rng();
            let mut pass_assembly: Vec<&[u8]> = Vec::new();

            if self.enab_strong_usab
                || (!self.enab_letters && !self.enab_u_letters && !self.enab_num && !self.enab_spec_symbs)
            {
                self.enab_strong_usab = true;
                pass_assembly.push(STRONG_USAB_CHARSET);
            } else {
                if self.enab_letters {
                    pass_assembly.push(LETTERS_CHARSET);
                }
                if self.enab_u_letters {
                    pass_assembly.push(U_LETTERS_CHARSET);
                }
                if self.enab_num {
                    pass_assembly.push(NUM_CHARSET);
                }
                if self.enab_spec_symbs {
                    pass_assembly.push(SPEC_SYMB_CHARSET);
                }
            }

            let pass_charset: Vec<u8> = pass_assembly.into_iter().flatten().cloned().collect();
            let mut pass_candidate_vec: Vec<u8> = Vec::new();
            let pass_processing_len: u32 = self.res_len.parse::<u32>().unwrap();

            if self.enab_strong_usab {
                let letters_charset: Vec<u8> = STRONG_USAB_LETTERS_CHARSET.into_iter().cloned().collect();
                let simp_symb_charset: Vec<u8> = SIMP_SYMB_CHARSET.into_iter().cloned().collect();

                // gen first pass symbol from all letters
                pass_candidate_vec.push(letters_charset[rng.gen_range(0..letters_charset.len())]);

                // gen main pass body
                for _ in 0..(pass_processing_len - 2) {
                    pass_candidate_vec.push(pass_charset[rng.gen_range(0..pass_charset.len())]);
                }

                // gen last pass symbol from simple symbols
                pass_candidate_vec
                    .push(simp_symb_charset[rng.gen_range(0..simp_symb_charset.len())]);

                String::from_utf8(pass_candidate_vec).unwrap()
            } else {
                (0..pass_processing_len)
                    .map(|_| pass_charset[rng.gen_range(0..pass_charset.len())] as char)
                    .collect()
            }
        }

        pub(crate) fn is_valid_pwd_by_consist(&self, pass: String) -> bool {
            let pwd_in_bytes = pass.clone().into_bytes();

            let check_to_available_for = |symbols: &[u8]| -> bool {
                let mut res = false;
                for ch in &pwd_in_bytes {
                    if symbols.contains(&ch) {
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
