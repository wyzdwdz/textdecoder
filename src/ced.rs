use std::ffi::CStr;
use std::os::raw::c_char;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum Language {
        ENGLISH,
        DANISH,
        DUTCH,
        FINNISH,
        FRENCH,
        GERMAN,
        HEBREW,
        ITALIAN,
        JAPANESE,
        KOREAN,
        NORWEGIAN,
        POLISH,
        PORTUGUESE,
        RUSSIAN,
        SPANISH,
        SWEDISH,
        CHINESE,
        CZECH,
        GREEK,
        ICELANDIC,
        LATVIAN,
        LITHUANIAN,
        ROMANIAN,
        HUNGARIAN,
        ESTONIAN,
        TG_UNKNOWN_LANGUAGE,
        UNKNOWN_LANGUAGE,
        BULGARIAN,
        CROATIAN,
        SERBIAN,
        IRISH, // UI only.
        GALICIAN,
        TAGALOG, // Tagalog (tl) + Filipino (fil),
        TURKISH,
        UKRAINIAN,
        HINDI,
        MACEDONIAN,
        BENGALI,
        INDONESIAN,
        LATIN, // UI only.
        MALAY,
        MALAYALAM,
        WELSH, // UI only.
        NEPALI,
        TELUGU,
        ALBANIAN,
        TAMIL,
        BELARUSIAN,
        JAVANESE, // UI only.
        OCCITAN,  // UI only.
        URDU,
        BIHARI,
        GUJARATI,
        THAI,
        ARABIC,
        CATALAN,
        ESPERANTO,
        BASQUE,
        INTERLINGUA, // UI only.
        KANNADA,
        PUNJABI,
        SCOTS_GAELIC, // UI only.
        SWAHILI,
        SLOVENIAN,
        MARATHI,
        MALTESE,
        VIETNAMESE,
        FRISIAN, // UI only.
        SLOVAK,
        CHINESE_T, // This is added to solve the problem of
        // distinguishing Traditional and Simplified
        // Chinese when the encoding is UTF8.
        FAROESE,   // UI only.
        SUNDANESE, // UI only.
        UZBEK,
        AMHARIC,
        AZERBAIJANI,
        GEORGIAN,
        TIGRINYA, // UI only.
        PERSIAN,
        BOSNIAN, // UI only. LangId language: CROATIAN (28)
        SINHALESE,
        NORWEGIAN_N,  // UI only. LangId language: NORWEGIAN (10)
        PORTUGUESE_P, // UI only. LangId language: PORTUGUESE (12)
        PORTUGUESE_B, // UI only. LangId language: PORTUGUESE (12)
        XHOSA,        // UI only.
        ZULU,         // UI only.
        GUARANI,
        SESOTHO, // UI only.
        TURKMEN, // UI only.
        KYRGYZ,
        BRETON,         // UI only.
        TWI,            // UI only.
        YIDDISH,        // UI only.
        SERBO_CROATIAN, // UI only. LangId language: SERBIAN (29)
        SOMALI,         // UI only.
        UIGHUR,
        KURDISH,
        MONGOLIAN,
        ARMENIAN,
        LAOTHIAN,
        SINDHI,
        RHAETO_ROMANCE, // UI only.
        AFRIKAANS,
        LUXEMBOURGISH, // UI only.
        BURMESE,
        KHMER,
        TIBETAN,
        DHIVEHI, // sometimes spelled Divehi, lang of Maldives
        CHEROKEE,
        SYRIAC, // UI only.
        LIMBU,  // UI only.
        ORIYA,
        ASSAMESE,    // UI only.
        CORSICAN,    // UI only.
        INTERLINGUE, // UI only.
        KAZAKH,
        LINGALA,   // UI only.
        MOLDAVIAN, // UI only. LangId language: ROMANIAN (22)
        PASHTO,
        QUECHUA, // UI only.
        SHONA,   // UI only.
        TAJIK,
        TATAR,                                // UI only.
        TONGA,                                // UI only.
        YORUBA,                               // UI only.
        CREOLES_AND_PIDGINS_ENGLISH_BASED,    // UI only.
        CREOLES_AND_PIDGINS_FRENCH_BASED,     // UI only.
        CREOLES_AND_PIDGINS_PORTUGUESE_BASED, // UI only.
        CREOLES_AND_PIDGINS_OTHER,            // UI only.
        MAORI,                                // UI only.
        WOLOF,                                // UI only.
        ABKHAZIAN,                            // UI only.
        AFAR,                                 // UI only.
        AYMARA,                               // UI only.
        BASHKIR,                              // UI only.
        BISLAMA,                              // UI only.
        DZONGKHA,                             // UI only.
        FIJIAN,                               // UI only.
        GREENLANDIC,                          // UI only.
        HAUSA,                                // UI only.
        HAITIAN_CREOLE,                       // UI only.
        INUPIAK,                              // UI only.
        INUKTITUT,
        KASHMIRI,    // UI only.
        KINYARWANDA, // UI only.
        MALAGASY,    // UI only.
        NAURU,       // UI only.
        OROMO,       // UI only.
        RUNDI,       // UI only.
        SAMOAN,      // UI only.
        SANGO,       // UI only.
        SANSKRIT,
        SISWANT,     // UI only.
        TSONGA,      // UI only.
        TSWANA,      // UI only.
        VOLAPUK,     // UI only.
        ZHUANG,      // UI only.
        KHASI,       // UI only.
        SCOTS,       // UI only.
        GANDA,       // UI only.
        MANX,        // UI only.
        MONTENEGRIN, // UI only. LangId language: SERBIAN (29)
        NUM_LANGUAGES, // Always keep this at the end. It is not a
                     // valid Language enum. It is only used to
                     // indicate the total number of Languages.
                     // NOTE: If you add a language, you will break a unittest. See the note
                     // at the top of this enum.
    }

    #[namespace = "CompactEncDet"]
    #[repr(i32)]
    #[derive(Debug)]
    enum TextCorpusType {
        WEB_CORPUS,
        XML_CORPUS,
        QUERY_CORPUS, // Use this for vanilla plaintext
        EMAIL_CORPUS,
        NUM_CORPA, // always last
    }

    #[repr(i32)]
    #[derive(Debug)]
    enum Encoding {
        ISO_8859_1,         // Teragram ASCII
        ISO_8859_2,         // Teragram Latin2
        ISO_8859_3,         // in BasisTech but not in Teragram
        ISO_8859_4,         // Teragram Latin4
        ISO_8859_5,         // Teragram ISO-8859-5
        ISO_8859_6,         // Teragram Arabic
        ISO_8859_7,         // Teragram Greek
        ISO_8859_8,         // Teragram Hebrew
        ISO_8859_9,         // in BasisTech but not in Teragram
        ISO_8859_10,        // in BasisTech but not in Teragram
        JAPANESE_EUC_JP,    // Teragram EUC_JP
        JAPANESE_SHIFT_JIS, // Teragram SJS
        JAPANESE_JIS,       // Teragram JIS
        CHINESE_BIG5,       // Teragram BIG5
        CHINESE_GB,         // Teragram GB
        CHINESE_EUC_CN,     // Misnamed. Should be EUC_TW. Was Basis Tech
        // CNS11643EUC, before that Teragram EUC-CN(!)
        // See //i18n/basistech/basistech_encodings.h
        KOREAN_EUC_KR,   // Teragram KSC
        UNICODE,         // Teragram Unicode
        CHINESE_EUC_DEC, // Misnamed. Should be EUC_TW. Was Basis Tech
        // CNS11643EUC, before that Teragram EUC.
        CHINESE_CNS, // Misnamed. Should be EUC_TW. Was Basis Tech
        // CNS11643EUC, before that Teragram CNS.
        CHINESE_BIG5_CP950, // Teragram BIG5_CP950
        JAPANESE_CP932,     // Teragram CP932
        UTF8,
        UNKNOWN_ENCODING,
        ASCII_7BIT, // ISO_8859_1 with all characters <= 127.
        // Should be present only in the crawler
        // and in the repository,
        // *never* as a result of Document::encoding().
        RUSSIAN_KOI8_R, // Teragram KOI8R
        RUSSIAN_CP1251, // Teragram CP1251

        //----------------------------------------------------------
        // These are _not_ output from teragram. Instead, they are as
        // detected in the headers of usenet articles.
        MSFT_CP1252,     // 27: CP1252 aka MSFT euro ascii
        RUSSIAN_KOI8_RU, // CP21866 aka KOI8-U, used for Ukrainian.
        // Misnamed, this is _not_ KOI8-RU but KOI8-U.
        // KOI8-U is used much more often than KOI8-RU.
        MSFT_CP1250, // CP1250 aka MSFT eastern european
        ISO_8859_15, // aka ISO_8859_0 aka ISO_8859_1 euroized
        //----------------------------------------------------------

        //----------------------------------------------------------
        // These are in BasisTech but not in Teragram. They are
        // needed for new interface languages. Now detected by
        // research langid
        MSFT_CP1254, // used for Turkish
        MSFT_CP1257, // used in Baltic countries
        //----------------------------------------------------------

        //----------------------------------------------------------
        //----------------------------------------------------------
        // New encodings detected by Teragram
        ISO_8859_11, // aka TIS-620, used for Thai
        MSFT_CP874,  // used for Thai
        MSFT_CP1256, // used for Arabic

        //----------------------------------------------------------
        // Detected as ISO_8859_8 by Teragram, but can be found in META tags
        MSFT_CP1255,   // Logical Hebrew Microsoft
        ISO_8859_8_I,  // Iso Hebrew Logical
        HEBREW_VISUAL, // Iso Hebrew Visual
        //----------------------------------------------------------

        //----------------------------------------------------------
        // Detected by research langid
        CZECH_CP852,
        CZECH_CSN_369103, // aka ISO_IR_139 aka KOI8_CS
        MSFT_CP1253,      // used for Greek
        RUSSIAN_CP866,
        //----------------------------------------------------------

        //----------------------------------------------------------
        // Handled by iconv in glibc
        ISO_8859_13,
        ISO_2022_KR,
        GBK,
        GB18030,
        BIG5_HKSCS,
        ISO_2022_CN,

        //-----------------------------------------------------------
        // Detected by xin liu's detector
        // Handled by transcoder
        // (Indic encodings)
        TSCII,
        TAMIL_MONO,
        TAMIL_BI,
        JAGRAN,

        MACINTOSH_ROMAN,
        UTF7,
        BHASKAR,    // Indic encoding - Devanagari
        HTCHANAKYA, // 56 Indic encoding - Devanagari

        //-----------------------------------------------------------
        // These allow a single place (inputconverter and outputconverter)
        // to do UTF-16 <==> UTF-8 bulk conversions and UTF-32 <==> UTF-8
        // bulk conversions, with interchange-valid checking on input and
        // fallback if needed on ouput.
        UTF16BE, // big-endian UTF-16
        UTF16LE, // little-endian UTF-16
        UTF32BE, // big-endian UTF-32
        UTF32LE, // little-endian UTF-32
        //-----------------------------------------------------------

        //-----------------------------------------------------------
        // An encoding that means "This is not text, but it may have some
        // simple ASCII text embedded". Intended input conversion (not yet
        // implemented) is to keep strings of >=4 seven-bit ASCII characters
        // (follow each kept string with an ASCII space), delete the rest of
        // the bytes. This will pick up and allow indexing of e.g. captions
        // in JPEGs. No output conversion needed.
        BINARYENC,
        //-----------------------------------------------------------

        //-----------------------------------------------------------
        // Some Web pages allow a mixture of HZ-GB and GB-2312 by using
        // ~{ ... ~} for 2-byte pairs, and the browsers support this.
        HZ_GB_2312,
        //-----------------------------------------------------------

        //-----------------------------------------------------------
        // Some external vendors make the common input error of
        // converting MSFT_CP1252 to UTF8 *twice*. No output conversion needed.
        UTF8UTF8,
        //-----------------------------------------------------------

        //-----------------------------------------------------------
        // Handled by transcoder for tamil language specific font
        // encodings without the support for detection at present.
        TAM_ELANGO,     // Elango - Tamil
        TAM_LTTMBARANI, // Barani - Tamil
        TAM_SHREE,      // Shree - Tamil
        TAM_TBOOMIS,    // TBoomis - Tamil
        TAM_TMNEWS,     // TMNews - Tamil
        TAM_WEBTAMIL,   // Webtamil - Tamil
        //-----------------------------------------------------------

        //-----------------------------------------------------------
        // Shift_JIS variants used by Japanese cell phone carriers.
        KDDI_SHIFT_JIS,
        DOCOMO_SHIFT_JIS,
        SOFTBANK_SHIFT_JIS,
        // ISO-2022-JP variants used by KDDI and SoftBank.
        KDDI_ISO_2022_JP,
        SOFTBANK_ISO_2022_JP,
        //-----------------------------------------------------------
        NUM_ENCODINGS, // Always keep this at the end. It is not a
                       // valid Encoding enum, it is only used to
                       // indicate the total number of Encodings.
    }

    unsafe extern "C++" {
        include!("compact_enc_det/compact_enc_det.h");

        type Language;

        #[namespace = "CompactEncDet"]
        type TextCorpusType;

        type Encoding;

        #[namespace = "CompactEncDet"]
        unsafe fn DetectEncoding(
            text: *const c_char,
            text_length: i32,
            url_hint: *const c_char,
            http_charset_hint: *const c_char,
            meta_charset_hint: *const c_char,
            encoding_hint: i32,
            language_hint: Language,
            corpus_type: TextCorpusType,
            ignore_7bit_mail_encodings: bool,
            bytes_consumed: *mut i32,
            is_reliable: *mut bool,
        ) -> Encoding;

        fn MimeEncodingName(enc: Encoding) -> *const c_char;
    }
}

pub use ffi::Encoding;
pub use ffi::Language;
pub use ffi::TextCorpusType;

pub fn detect_encoding(
    text: &[u8],
    url_hint: &str,
    http_charset_hint: &str,
    meta_charset_hint: &str,
    encoding_hint: Encoding,
    language_hint: Language,
    corpus_type: TextCorpusType,
    ignore_7bit_mail_encodings: bool,
) -> (Encoding, i32, bool) {
    let text_ptr: *const c_char = text.as_ptr() as *const c_char;
    let url_hint_ptr: *const c_char = url_hint.as_ptr() as *const c_char;
    let http_charset_hint_ptr: *const c_char = http_charset_hint.as_ptr() as *const c_char;
    let meta_charset_hint_ptr: *const c_char = meta_charset_hint.as_ptr() as *const c_char;

    let mut bytes_consumed: i32 = 0;
    let mut is_reliable: bool = false;

    unsafe {
        let encoding = ffi::DetectEncoding(
            text_ptr,
            text.len() as i32,
            url_hint_ptr,
            http_charset_hint_ptr,
            meta_charset_hint_ptr,
            encoding_hint.repr,
            language_hint,
            corpus_type,
            ignore_7bit_mail_encodings,
            &mut bytes_consumed,
            &mut is_reliable,
        );

        (encoding, bytes_consumed, is_reliable)
    }
}

pub fn mime_encoding_name(enc: Encoding) -> String {
    let str_ptr: *const c_char = ffi::MimeEncodingName(enc);

    unsafe {
        let str = CStr::from_ptr(str_ptr);
        str.to_string_lossy().to_string()
    }
}
