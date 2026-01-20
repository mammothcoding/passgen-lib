pub mod lang {
    /// Supported languages for password strength level description
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Language {
        /// English - default language
        English,
        /// Chinese (Simplified)
        Chinese,
        /// Spanish
        Spanish,
        /// Hindi
        Hindi,
        /// Arabic
        Arabic,
        /// Portuguese
        Portuguese,
        /// Bengali
        Bengali,
        /// Russian
        Russian,
        /// Japanese
        Japanese,
        /// Punjabi
        Punjabi,
        /// German
        German,
        /// Korean
        Korean,
        /// French
        French,
        /// Turkish
        Turkish,
        /// Italian
        Italian,
    }

    /// Translations for password strength levels
    pub struct StrengthTranslations;

    impl StrengthTranslations {
        /// Get the strength level description in the selected language
        pub fn get_level(language: Language, score: u8) -> &'static str {
            match score {
                0..=20 => match language {
                    Language::English => "Very Weak",
                    Language::Chinese => "非常弱",
                    Language::Spanish => "Muy Débil",
                    Language::Hindi => "बहुत कमजोर",
                    Language::Arabic => "ضعيف جداً",
                    Language::Portuguese => "Muito Fraco",
                    Language::Bengali => "খুব দুর্বল",
                    Language::Russian => "Очень слабый",
                    Language::Japanese => "非常に弱い",
                    Language::Punjabi => "ਬਹੁਤ ਕਮਜ਼ੋਰ",
                    Language::German => "Sehr Schwach",
                    Language::Korean => "매우 약함",
                    Language::French => "Très Faible",
                    Language::Turkish => "Çok Zayıf",
                    Language::Italian => "Molto Debole",
                },
                21..=40 => match language {
                    Language::English => "Weak",
                    Language::Chinese => "弱",
                    Language::Spanish => "Débil",
                    Language::Hindi => "कमजोर",
                    Language::Arabic => "ضعيف",
                    Language::Portuguese => "Fraco",
                    Language::Bengali => "দুর্বল",
                    Language::Russian => "Слабый",
                    Language::Japanese => "弱い",
                    Language::Punjabi => "ਕਮਜ਼ੋਰ",
                    Language::German => "Schwach",
                    Language::Korean => "약함",
                    Language::French => "Faible",
                    Language::Turkish => "Zayıf",
                    Language::Italian => "Debole",
                },
                41..=60 => match language {
                    Language::English => "Fair",
                    Language::Chinese => "一般",
                    Language::Spanish => "Aceptable",
                    Language::Hindi => "मध्यम",
                    Language::Arabic => "مقبول",
                    Language::Portuguese => "Razoável",
                    Language::Bengali => "মধ্যম",
                    Language::Russian => "Удовлетворительный",
                    Language::Japanese => "普通",
                    Language::Punjabi => "ਸ਼ਾਤ",
                    Language::German => "Mittel",
                    Language::Korean => "보통",
                    Language::French => "Moyen",
                    Language::Turkish => "Orta",
                    Language::Italian => "Discreto",
                },
                61..=80 => match language {
                    Language::English => "Strong",
                    Language::Chinese => "强",
                    Language::Spanish => "Fuerte",
                    Language::Hindi => "मजबूत",
                    Language::Arabic => "قوي",
                    Language::Portuguese => "Forte",
                    Language::Bengali => "শক্তিশালী",
                    Language::Russian => "Сильный",
                    Language::Japanese => "強い",
                    Language::Punjabi => "ਮਜ਼ਬੂਤ",
                    Language::German => "Stark",
                    Language::Korean => "강함",
                    Language::French => "Fort",
                    Language::Turkish => "Güçlü",
                    Language::Italian => "Forte",
                },
                81..=100 => match language {
                    Language::English => "Very Strong",
                    Language::Chinese => "非常强",
                    Language::Spanish => "Muy Fuerte",
                    Language::Hindi => "बहुत मजबूत",
                    Language::Arabic => "قوي جداً",
                    Language::Portuguese => "Muito Forte",
                    Language::Bengali => "খুব শক্তিশালী",
                    Language::Russian => "Очень сильный",
                    Language::Japanese => "非常に強い",
                    Language::Punjabi => "ਬਹੁਤ ਮਜ਼ਬੂਤ",
                    Language::German => "Sehr Stark",
                    Language::Korean => "매우 강함",
                    Language::French => "Très Fort",
                    Language::Turkish => "Çok Güçlü",
                    Language::Italian => "Molto Forte",
                },
                _ => "Unknown",
            }
        }
    }
}
