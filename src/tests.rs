pub mod tests {
    use crate::Passgen;

    #[test]
    fn it_works() {
        assert_eq!(Passgen::new().generate(4).len(), 0);
        assert_ne!(
            Passgen::new().set_enabled_letters(true).generate(4).len(),
            0
        );
        assert_ne!(
            Passgen::default()
                .set_custom_charset("abcABC123⭕➖❎⚫⬛п₼⁂🙂")
                .generate(4)
                .len(),
            0
        );

        assert_ne!(Passgen::default_strong_and_usab().generate(4).len(), 0);
    }

    #[test]
    fn password_validation() {
        let mut generator = Passgen::default();

        // Test password setting
        generator.set_password("Test123!");
        assert_eq!(generator.get_password(), "Test123!");

        // Test validation
        assert!(generator.validate_password());

        // Test with invalid password for rules
        generator.set_enabled_spec_symbols(true);
        generator.set_password("Test123"); // No special symbol
        assert!(!generator.validate_password());
    }

    #[test]
    fn password_strength_scoring() {
        let mut generator = Passgen::default();

        // Test very weak password (short)
        generator.set_password("123");
        assert_eq!(generator.password_strength_score(), 0);

        // Test weak password (exact match)
        generator.set_password("password");
        assert_eq!(generator.password_strength_score(), 0);

        // Test password containing "password" pattern
        generator.set_password("password123");
        let score = generator.password_strength_score();
        assert!(score < 40);

        // Test better password (but still contains "123")
        generator.set_password("Password123");
        let score = generator.password_strength_score();
        assert!(score >= 30 && score <= 60);

        // Test good password with special char
        generator.set_password("P@ssw0rd123");
        let score = generator.password_strength_score();
        assert!(score >= 40 && score <= 70);

        // Test strong password
        generator.set_password("MyV3ry$tr0ngP@ssw0rd!");
        let score = generator.password_strength_score();
        assert!(score >= 70 && score <= 90);
    }

    #[test]
    fn password_strength_levels() {
        let mut generator = Passgen::default();

        generator.set_password("123");
        assert_eq!(generator.password_strength_level(), "Very Weak");

        generator.set_password("password123");
        let level = generator.password_strength_level();
        assert!(level == "Weak" || level == "Very Weak");

        generator.set_password("Password123");
        let level = generator.password_strength_level();
        assert!(level == "Fair" || level == "Weak");

        generator.set_password("P@ssw0rd123!");
        let level = generator.password_strength_level();
        assert!(level == "Strong" || level == "Fair");

        generator.set_password("V3ry$tr0ng&P@ssw0rdW1thEntropy!");
        assert_eq!(generator.password_strength_level(), "Strong");
    }

    #[test]
    fn custom_charset_validation() {
        let mut generator = Passgen::new();
        generator.set_custom_charset("abc123");

        generator.set_password("abc123");
        assert!(generator.validate_password());

        generator.set_password("abc123d"); // 'd' not in charset
        assert!(!generator.validate_password());
    }

    #[test]
    fn generated_password_stored_in_field() {
        let mut generator = Passgen::default();

        // Generate a password
        let password = generator.generate(12);

        // Check that the password is stored in the field
        assert_eq!(generator.get_password(), password);
        assert!(!generator.get_password().is_empty());

        // You can immediately get the strength score
        let score = generator.password_strength_score();
        assert!(score > 0);
    }

    #[test]
    fn multilingual_strength_levels() {
        use crate::Language;

        let mut generator = Passgen::default();
        generator.set_password("V3ry$tr0ng&P@ssw0rd!");

        // Test English
        generator.set_language(Language::English);
        assert_eq!(generator.password_strength_level(), "Strong");

        // Test Russian
        generator.set_language(Language::Russian);
        assert_eq!(generator.password_strength_level(), "Сильный");

        // Test Spanish
        generator.set_language(Language::Spanish);
        assert_eq!(generator.password_strength_level(), "Fuerte");

        // Test Chinese
        generator.set_language(Language::Chinese);
        assert_eq!(generator.password_strength_level(), "强");

        // Test Portuguese
        generator.set_language(Language::Portuguese);
        assert_eq!(generator.password_strength_level(), "Forte");

        // Test Japanese
        generator.set_language(Language::Japanese);
        assert_eq!(generator.password_strength_level(), "強い");
    }

    #[test]
    fn password_strength_with_different_passwords() {
        let mut generator = Passgen::default();

        // Very weak passwords
        generator.set_password("123");
        assert_eq!(generator.password_strength_score(), 0);

        generator.set_password("abc");
        assert_eq!(generator.password_strength_score(), 0);

        // Weak passwords (exact matches)
        generator.set_password("password");
        assert_eq!(generator.password_strength_score(), 0);

        generator.set_password("123456");
        assert_eq!(generator.password_strength_score(), 0);

        // Moderate passwords (not exact matches, but contain patterns)
        generator.set_password("Password1");
        let score3 = generator.password_strength_score();
        assert!(score3 >= 30 && score3 <= 60);

        // Good passwords
        generator.set_password("P@ssw0rd123");
        let score4 = generator.password_strength_score();
        assert!(score4 >= 40 && score4 <= 70);

        // Strong passwords
        generator.set_password("MyP@ssw0rd!2024");
        let score5 = generator.password_strength_score();
        assert!(score5 >= 60 && score5 <= 85);

        // Very strong passwords
        generator.set_password("V3ry$3cur3&P@ssw0rd!L0ng");
        let score6 = generator.password_strength_score();
        assert!(score6 >= 70 && score6 <= 95);
    }

    #[test]
    fn find_maximum_score_password() {
        // Let's try to find a password that gets close to 100 points
        // Based on our algorithm, we need:
        // 1. Long password (20+ characters) - 25 points
        // 2. All character types (lowercase, uppercase, digits, special) - 25 points
        // 3. High uniqueness - all characters unique - 20 points
        // 4. No weak patterns or sequences
        // 5. Good entropy calculation - 10 points
        // Maximum theoretical: 25 + 25 + 20 + 10 = 80 points

        let mut generator = Passgen::default();

        // Let's create an "ideal" password according to our algorithm
        // This password is 24 characters long, contains all character types,
        // has no obvious patterns, and all characters are unique
        let ideal_password = "Kp@3#mN9!qZ$7%vR*2&xY5^wL8+dF6";
        generator.set_password(ideal_password);

        let score = generator.password_strength_score();

        // With our current algorithm, the maximum is around 75-80 points
        println!("Ideal password score: {}", score);
        println!("Ideal password: {}", ideal_password);

        // This should be one of the highest possible scores
        assert!(score >= 70 && score <= 80);

        // Let's also test with a very long random password
        let long_password = "aB1@cD2#eF3$gH4%iJ5^kL6&mN7*oP8(qR9)sT0_uV1+wX2-yZ3";
        generator.set_password(long_password);
        let long_score = generator.password_strength_score();

        println!("Very long password score: {}", long_score);
        println!("Very long password: {}", long_password);

        // The score might be similar or slightly higher
        assert!(long_score >= 70 && long_score <= 80);

        // Let's see what our generator produces
        generator.generate(24);
        let generated_score = generator.password_strength_score();
        let generated_password = generator.get_password();

        println!("Generated password score: {}", generated_score);
        println!("Generated password: {}", generated_password);

        // Generated passwords should also be strong
        assert!(generated_score >= 60);
    }
}