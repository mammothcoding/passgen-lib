mod gen_engine;

/// Main structure.
pub struct Passgen {
    /// Presence of letters.
    pub enab_letters: bool,
    /// Presence of a capital letters.
    pub enab_u_letters: bool,
    /// Presence of numeric characters.
    pub enab_num: bool,
    /// Presence of special characters.
    pub enab_spec_symbs: bool,
    /// Length of the result.
    pub res_len: String,
    /// Minimal length of the result.
    pub min_res_len: u32,
    /// Maximal length of the result.
    pub max_res_len: u32,
    /// Including all characters, but
    /// the first position in the password is a capital or small letter,
    /// the last position is the symbol. Excluded ambiguous characters "0oOiIlL1".
    /// If this rule is enabled, the other consistency rules of the result are not taken.
    pub enab_strong_usab: bool,
}

impl Passgen {
    /// Set default values for Passgen.
    pub fn default() -> Passgen {
        Passgen {
            enab_letters: false,
            enab_u_letters: false,
            enab_num: false,
            enab_spec_symbs: false,
            res_len: "8".to_string(),
            min_res_len: 4,
            max_res_len: 10000,
            enab_strong_usab: true,
        }
    }

    pub fn get_rule_state(&self, rule_name: &str) -> bool {
        match rule_name {
            "letters" => self.enab_letters.clone(),
            "u_letters" => self.enab_u_letters.clone(),
            "numbs" => self.enab_num.clone(),
            "spec_symbs" => self.enab_spec_symbs.clone(),
            "convenience_criterion" => self.enab_strong_usab.clone(),
            _ => true,
        }
    }

    pub fn set_rule_state(&mut self, rule_name: &str, new_val: bool) {
        match rule_name {
            "letters" => self.enab_letters = new_val,
            "u_letters" => self.enab_u_letters = new_val,
            "numbs" => self.enab_num = new_val,
            "spec_symbs" => self.enab_spec_symbs = new_val,
            "convenience_criterion" => self.enab_strong_usab = new_val,
            _ => {}
        }
    }

    pub fn generate(&mut self) -> String {
        if self.is_valid_user_input() {
            let mut pwd = self.generate_pass();
            while !self.is_valid_pwd_by_consist(pwd.clone()) {
                pwd = self.generate_pass();
            }
            pwd
        } else {
            "error_bla".to_string().parse().unwrap()
        }

        //self.pwd_len.clear();
        //self.reset_cursor();
    }

    fn is_valid_user_input(&self) -> bool {
        let parse_res = self.res_len.parse::<u32>();
        match parse_res {
            Ok(val) => {
                if val < self.min_res_len || val > self.max_res_len {
                    false
                } else {
                    true
                }
            }
            Err(_err) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Passgen;

    #[test]
    fn it_works() {
        let result = Passgen::default().generate();
        assert_ne!(result.len(), 0);
    }
}
