pub mod gen_engine;
pub mod lang;

use crate::gen_engine::gen_engine::{LETTERS_CHARSET, NUM_CHARSET, SPEC_SYMB_CHARSET, U_LETTERS_CHARSET};
use crate::lang::lang::{Language, StrengthTranslations};

/// Main [Passgen] structure.
///
/// # Examples
///
/// You can create a token that includes lowercase letters and numbers up to 30 characters long:
///
/// ```
/// use passgenlib::Passgen;
/// let result = Passgen::new().set_enabled_letters(true).set_enabled_numbers(true).generate(30);
/// ```
///
/// You can create a default strong password including all literals, numbers and symbols:
///
/// ```
/// use passgenlib::Passgen;
/// let result = Passgen::default().generate(12);
/// ```
///
/// You can create a strong and usability password.
/// Including all characters, but
/// the first position in the password is a capital or small letter,
/// the last position is the symbol.
/// 🔸 Excluded ambiguous characters `"0oOiIlL1"`.
///
/// ```
/// use passgenlib::Passgen;
/// let result = Passgen::default_strong_and_usab().generate(8);
/// ```
/// You can create a set from your custom charset:
///
/// ```
/// use passgenlib::Passgen;
/// let result = Passgen::new().set_custom_charset("bla@.321").generate(8);
/// ```
///
/// YYou can validate the existing password against the added rules:
///
/// ```
/// use passgenlib::Passgen;
/// let mut generator = Passgen::default();
/// generator.set_enabled_letters(true).set_enabled_numbers(true);
/// generator.set_password("MyP@ssw0rd");
/// assert!(generator.validate_password());
/// ```
///
/// You can get password strength score:
///
/// ```
/// use passgenlib::Passgen;
/// let mut generator = Passgen::default();
/// generator.set_password("MyP@ssw0rd");
/// let score = generator.password_strength_score();
/// assert!(score >= 0 && score <= 100);
/// ```
///
/// You can get password strength level in multiple languages:
///
/// ```
/// use passgenlib::{Passgen, Language};
/// let mut generator = Passgen::default();
/// generator.set_password("MyP@ssw0rd");
///
/// // English (default)
/// assert_eq!(generator.password_strength_level(), "Strong");
///
/// // Russian
/// generator.set_language(Language::Russian);
/// assert_eq!(generator.password_strength_level(), "Сильный");
///
/// // Spanish
/// generator.set_language(Language::Spanish);
/// assert_eq!(generator.password_strength_level(), "Fuerte");
/// ```
///
/// You can generate password and immediately get its strength score:
///
/// ```
/// use passgenlib::Passgen;
/// let mut generator = Passgen::default();
/// let password = generator.generate(12);
///
/// // The generated password is stored in the password field
/// assert_eq!(generator.get_password(), password);
///
/// // You can immediately get the strength score
/// let score = generator.password_strength_score();
/// assert!(score > 0);
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

    /// Including all characters, but
    /// the first position in the password is a capital or small letter,
    /// the last position is the symbol. Excluded ambiguous characters `"0oOiIlL1"`.
    ///
    /// ⚠️ If this rule is enabled, the other consistency rules of the generating are not taken,
    /// except for a rule `custom_charset`.
    pub enab_strong_usab: bool,

    /// User defined character set.
    ///
    /// ⚠️This set of characters will exclude all other rules except for a rule `"enab_strong_usab"`.
    ///
    /// ⚙️If `"enab_strong_usab"` on too then you can generate combined strong and usability result with custom charset.
    pub custom_charset: &'static str,

    /// Current password stored for validation and strength checking.
    /// This field is automatically populated when using the `generate()` method
    /// or manually set using the `set_password()` method.
    pub password: String,

    /// Language for password strength level descriptions.
    /// Default is English.
    pub language: Language,
}

impl Passgen {
    /// Get an instance of `Passgen` without any rules.
    pub fn new() -> Passgen {
        Passgen {
            enab_letters: false,
            enab_u_letters: false,
            enab_num: false,
            enab_spec_symbs: false,
            enab_strong_usab: false,
            custom_charset: "",
            password: String::new(),
            language: Language::English,
        }
    }

    /// Set default ruleset of `Passgen` to *"all simple rules are enabled"*.
    pub fn default() -> Passgen {
        Passgen {
            enab_letters: true,
            enab_u_letters: true,
            enab_num: true,
            enab_spec_symbs: true,
            enab_strong_usab: false,
            custom_charset: "",
            password: String::new(),
            language: Language::English,
        }
    }

    /// Set default ruleset of `Passgen` to *"Strong & usability"*.
    ///
    /// Including all characters, but
    /// the first position in the password is a capital or small letter,
    /// the last position is the symbol. Excluded ambiguous characters `"0oOiIlL1"`.
    ///
    /// ⚠️ If this rule is enabled, the other consistency rules of the generating are not taken,
    /// except for a rule `custom_charset`.
    pub fn default_strong_and_usab() -> Passgen {
        Passgen {
            enab_letters: false,
            enab_u_letters: false,
            enab_num: false,
            enab_spec_symbs: false,
            custom_charset: "",
            enab_strong_usab: true,
            password: String::new(),
            language: Language::English,
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
    /// ⚠️ If this rule is enabled, the other consistency rules of the generating are not taken,
    /// except for a rule `custom_charset`.
    pub fn set_enabled_strong_usab(&mut self, value: bool) -> &mut Passgen {
        self.enab_strong_usab = value;
        self
    }

    /// Set user defined character set.
    /// You can use any Unicode characters and emoji. For example: abcABC123⭕➖❎⚫⬛n₼⁂🙂
    ///
    /// ⚠️This set of characters will exclude all other rules except for a rule `"enab_strong_usab"`.
    ///
    /// ⚙️If `"enab_strong_usab"` on too then you can generate combined strong and usability result with custom charset.
    pub fn set_custom_charset(&mut self, value: &'static str) -> &mut Passgen {
        self.custom_charset = value;
        self
    }

    /// Set password for validation and strength checking.
    /// This method is useful when you want to validate or check the strength
    /// of an existing password.
    pub fn set_password(&mut self, password: &str) -> &mut Passgen {
        self.password = password.to_string();
        self
    }

    /// Get current password.
    /// Returns the password that was either generated using `generate()` method
    /// or set using `set_password()` method.
    pub fn get_password(&self) -> &str {
        &self.password
    }

    /// Set language for password strength level descriptions.
    pub fn set_language(&mut self, language: Language) -> &mut Passgen {
        self.language = language;
        self
    }

    /// Generate result. Argument "length" will not be less than 4.
    /// The generated password is automatically stored in the `password` field
    /// for immediate validation or strength checking.
    pub fn generate(&mut self, length: u32) -> String {
        if !self.is_ruleset_clean() {
            let res_len = if length < 4 { 4 } else { length };

            let mut pwd = self.generate_pass(res_len);

            if self.custom_charset.len() == 0 {
                while !self.validate_password_rules(pwd.clone()) {
                    pwd = self.generate_pass(res_len);
                }
            }

            self.password = pwd.clone();
            pwd
        } else {
            self.password.clear();
            "".to_string()
        }
    }

    /// Validate if the current password matches the configured rules.
    pub fn validate_password(&self) -> bool {
        if self.password.is_empty() {
            return false;
        }

        if self.custom_charset.len() > 0 {
            // If custom charset is set, check if all characters are from that charset
            for ch in self.password.chars() {
                if !self.custom_charset.contains(ch) {
                    return false;
                }
            }
            return true;
        }

        self.validate_password_rules(self.password.clone())
    }

    /// Calculate password strength score (0-100).
    /// Based on multiple factors: length, character variety, entropy, and common patterns.
    pub fn password_strength_score(&self) -> u8 {
        if self.password.is_empty() {
            return 0;
        }

        let password = &self.password;
        let length = password.len();

        // Very short passwords get 0
        if length < 4 {
            return 0;
        }

        let mut score = 0i32;

        // 1. Length score (max 25 points)
        score += match length {
            0..=4 => 0,
            5..=6 => 5,
            7..=8 => 10,
            9..=10 => 15,
            11..=12 => 20,
            _ => 25,
        };

        // 2. Character variety analysis
        let mut has_lowercase = false;
        let mut has_uppercase = false;
        let mut has_digits = false;
        let mut has_special = false;
        let mut unique_chars = std::collections::HashSet::new();

        for ch in password.chars() {
            unique_chars.insert(ch);

            if ch.is_ascii_lowercase() {
                has_lowercase = true;
            } else if ch.is_ascii_uppercase() {
                has_uppercase = true;
            } else if ch.is_ascii_digit() {
                has_digits = true;
            } else if ch.is_ascii_punctuation() || "[]{}()<>".contains(ch) {
                has_special = true;
            }
        }

        // Count character types
        let char_type_count = [has_lowercase, has_uppercase, has_digits, has_special]
            .iter()
            .filter(|&&x| x)
            .count();

        // Character variety score (max 25 points)
        let mut variety_score = 0;
        if has_lowercase { variety_score += 5; }
        if has_uppercase { variety_score += 5; }
        if has_digits { variety_score += 5; }
        if has_special { variety_score += 10; }

        // Bonus for multiple character types
        match char_type_count {
            2 => variety_score += 5,
            3 => variety_score += 10,
            4 => variety_score += 15,
            _ => {}
        }

        // Cap variety score at 25
        score += variety_score.min(25);

        // 3. Unique characters ratio (max 20 points)
        let uniqueness_ratio = unique_chars.len() as f32 / length as f32;
        score += (uniqueness_ratio * 20.0) as i32;

        // 4. Check for exact matches with weak passwords (immediate 0)
        let weak_passwords = [
            "password", "123456", "qwerty", "admin", "welcome",
            "12345678", "123456789", "12345", "1234", "111111",
        ];

        let lower_pwd = password.to_lowercase();
        for weak in &weak_passwords {
            if lower_pwd == *weak {
                return 0;
            }
        }

        // 5. Check for containing weak patterns
        let weak_patterns = ["password", "123", "qwerty", "admin", "letmein"];

        let mut pattern_penalty = 0;
        for pattern in &weak_patterns {
            if lower_pwd.contains(pattern) {
                pattern_penalty += 15;
            }
        }

        // 6. Penalties for weak patterns
        let mut penalty = pattern_penalty;
        let chars: Vec<char> = password.chars().collect();

        // Check for sequential characters
        for i in 0..chars.len().saturating_sub(2) {
            let c1 = chars[i] as u32;
            let c2 = chars[i + 1] as u32;
            let c3 = chars[i + 2] as u32;

            if c2 == c1 + 1 && c3 == c2 + 1 {
                penalty += 10;
                break;
            }
        }

        // Check for repeated characters
        for i in 0..chars.len().saturating_sub(2) {
            if chars[i] == chars[i + 1] && chars[i] == chars[i + 2] {
                penalty += 10;
                break;
            }
        }

        // Penalty for too few character types
        if char_type_count < 2 {
            penalty += 10;
        }

        // Apply penalty (max 30 points penalty)
        score -= penalty.min(30);

        // 7. Simple entropy estimation (max 10 points)
        // This is a simplified calculation to avoid over-scoring
        let mut charset_size = 0;
        if has_lowercase { charset_size += 26; }
        if has_uppercase { charset_size += 26; }
        if has_digits { charset_size += 10; }
        if has_special { charset_size += 32; }

        if charset_size > 0 {
            // Very conservative entropy calculation
            // We cap it at 10 points to avoid over-scoring
            let entropy_per_char = (charset_size as f32).log2();
            let total_entropy = length as f32 * entropy_per_char;

            // Normalize to 0-10 points (very conservative)
            let entropy_score = (total_entropy / 10.0).min(10.0);
            score += entropy_score as i32;
        }

        // Ensure score is between 0 and 100
        score = score.max(0).min(100);

        score as u8
    }

    /// Get password strength level description in the selected language.
    pub fn password_strength_level(&self) -> &'static str {
        let score = self.password_strength_score();
        StrengthTranslations::get_level(self.language, score)
    }

    fn is_ruleset_clean(&self) -> bool {
        !self.enab_letters
            && !self.enab_u_letters
            && !self.enab_num
            && !self.enab_spec_symbs
            && !self.enab_strong_usab
            && self.custom_charset.len() == 0
    }

    fn validate_password_rules(&self, pass: String) -> bool {
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

#[cfg(test)]
mod tests;
