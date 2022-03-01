use {
    std::fmt,
    serde::{
        Serialize,
        Deserialize
    }
};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Locale {
    English,
    French,
    Spanish,
    MexicanSpanish,
    German,
    Italian,
    Japanese,
    BrazilianPortugese,
    Russian,
    Polish,
    Korean,
    ChineseTraditional,
    ChineseSimplified,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tmp = match self {
            Locale::English => "en",
            Locale::French => "fr",
            Locale::Spanish => "es",
            Locale::MexicanSpanish => "es-mx",
            Locale::German => "de",
            Locale::Italian => "it",
            Locale::Japanese => "ja",
            Locale::BrazilianPortugese => "pt-br",
            Locale::Russian => "ru",
            Locale::Polish => "pl",
            Locale::Korean => "ko",
            Locale::ChineseTraditional => "zh-cht",
            Locale::ChineseSimplified => "zh-chs",
        };

        write!(f, "{}", tmp)
    }
}