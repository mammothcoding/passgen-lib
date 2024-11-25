mod gen_engine;

/// Main [Passgen] structure.
///
/// # Examples
///
/// You can create a strong token:
///
/// ```
/// use passgenlib::Passgen;
/// let pwd = Passgen::default().generate(30);
/// ```
///
/// You can create a simple strong and usability password:
///
/// ```
/// use passgenlib::Passgen;
/// let pwd = Passgen::default_strong_and_usab().generate(8);
/// ```
pub struct Passgen {
    /// Presence of letters.
    pub enab_letters: bool,
    /// Presence of a capital letters.
    pub enab_u_letters: bool,
    /// Presence of numeric characters.
    pub enab_num: bool,
    /// Presence of special characters.
    pub enab_spec_symbs: bool,
    /// Set default field values for `Passgen` (Strong & usability).
    ///
    /// Including all characters, but
    /// the first position in the password is a capital or small letter,
    /// the last position is the symbol. Excluded ambiguous characters `"0oOiIlL1"`.
    ///
    /// ⚠️ If this rule is enabled, the other consistency rules of the generating are not taken.
    pub enab_strong_usab: bool,
}

impl Passgen {
    /// Set default field values for Passgen (*all rules on*).
    pub fn default() -> Passgen {
        Passgen {
            enab_letters: true,
            enab_u_letters: true,
            enab_num: true,
            enab_spec_symbs: true,
            enab_strong_usab: false,
        }
    }

    /// Set default field values for `Passgen` (*Strong & usability*).
    ///
    /// Including all characters, but
    /// the first position in the password is a capital or small letter,
    /// the last position is the symbol. Excluded ambiguous characters `"0oOiIlL1"`.
    ///
    /// ⚠️ If this rule is enabled, the other consistency rules of the generating are not taken.
    pub fn default_strong_and_usab() -> Passgen {
        Passgen {
            enab_letters: false,
            enab_u_letters: false,
            enab_num: false,
            enab_spec_symbs: false,
            enab_strong_usab: true,
        }
    }

    /// Set value of the field `enab_letters` for `Passgen`.
    pub fn set_enabled_letters(&mut self, value: bool) -> &mut Passgen {
        self.enab_letters = value;
        self
    }

    /// Set value of the field `enab_u_letters` for `Passgen`.
    pub fn set_enabled_uppercase_letters(&mut self, value: bool) -> &mut Passgen {
        self.enab_u_letters = value;
        self
    }

    /// Set value of the field `enab_num` for `Passgen`.
    pub fn set_enabled_numbers(&mut self, value: bool) -> &mut Passgen {
        self.enab_num = value;
        self
    }

    /// Set value of the field `enab_spec_symbs` for `Passgen`.
    pub fn set_enabled_spec_symbols(&mut self, value: bool) -> &mut Passgen {
        self.enab_spec_symbs = value;
        self
    }

    /// Set value of the field `enab_strong_usab` for `Passgen`.
    ///
    /// Including all characters, but
    /// the first position in the password is a capital or small letter,
    /// the last position is the symbol. Excluded ambiguous characters `"0oOiIlL1"`.
    ///
    /// ⚠️ If this rule is enabled, the other consistency rules of the generating are not taken.
    pub fn set_enabled_strong_usab(&mut self, value: bool) -> &mut Passgen {
        self.enab_strong_usab = value;
        self
    }

    /// Generate result. Argument "len" will not be less than 4
    pub fn generate(&mut self, len: u32) -> String {
        let res_len = if len < 4 { 4 } else { len };

        let mut pwd = self.generate_pass(res_len);
        while !self.is_valid_pwd_by_consist(pwd.clone()) {
            pwd = self.generate_pass(res_len);
        }
        pwd
    }
}

#[cfg(test)]
mod tests {
    use crate::Passgen;

    #[test]
    fn it_works() {
        assert_ne!(Passgen::default().set_enabled_letters(true).generate(4).len(), 0);
        assert_ne!(Passgen::default_strong_and_usab().generate(4).len(), 0);
    }
}
