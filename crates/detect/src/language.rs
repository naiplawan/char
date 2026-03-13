pub fn get_preferred_languages() -> Vec<hypr_language::Language> {
    get_locale_strings()
        .into_iter()
        .filter_map(|s| locale_to_language(&s))
        .collect()
}

pub fn get_current_locale_identifier() -> String {
    get_locale_strings().into_iter().next().unwrap_or_default()
}

fn locale_to_language(locale: &str) -> Option<hypr_language::Language> {
    locale.parse().ok()
}

#[cfg(target_os = "macos")]
fn get_locale_strings() -> Vec<String> {
    use objc2_foundation::NSLocale;

    let languages = NSLocale::preferredLanguages();
    languages.iter().map(|s| s.to_string()).collect()
}

#[cfg(target_os = "linux")]
fn get_locale_strings() -> Vec<String> {
    if let Ok(lang) = std::env::var("LANGUAGE") {
        let langs: Vec<String> = lang
            .split(':')
            .filter(|s| !s.is_empty())
            .map(|s| s.split('.').next().unwrap_or(s).replace('_', "-"))
            .collect();
        if !langs.is_empty() {
            return langs;
        }
    }

    for var in &["LANG", "LC_ALL", "LC_MESSAGES"] {
        if let Ok(locale) = std::env::var(var) {
            let normalized = locale
                .split('.')
                .next()
                .unwrap_or(&locale)
                .replace('_', "-");
            if !normalized.is_empty() && normalized != "C" && normalized != "POSIX" {
                return vec![normalized];
            }
        }
    }

    vec!["en-US".to_string()]
}

#[cfg(target_os = "windows")]
fn get_locale_strings() -> Vec<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    extern "system" {
        fn GetUserDefaultLocaleName(lp_locale_name: *mut u16, cch_locale_name: i32) -> i32;
    }

    // LOCALE_NAME_MAX_LENGTH = 85
    let mut buf = [0u16; 85];
    let len = unsafe { GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };

    if len > 1 {
        let locale = OsString::from_wide(&buf[..len as usize - 1])
            .to_string_lossy()
            .into_owned();
        vec![locale]
    } else {
        vec!["en-US".to_string()]
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn get_locale_strings() -> Vec<String> {
    vec!["en-US".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use hypr_language::ISO639;

    #[test]
    fn test_locale_to_language() {
        let lang = locale_to_language("en-US").unwrap();
        assert_eq!(lang.iso639(), ISO639::En);
        assert_eq!(lang.region(), Some("US"));

        let lang = locale_to_language("ko-US").unwrap();
        assert_eq!(lang.iso639(), ISO639::Ko);
        assert_eq!(lang.region(), Some("US"));

        let lang = locale_to_language("ja_JP").unwrap();
        assert_eq!(lang.iso639(), ISO639::Ja);
        assert_eq!(lang.region(), Some("JP"));

        let lang = locale_to_language("zh-Hans-CN").unwrap();
        assert_eq!(lang.iso639(), ISO639::Zh);
        assert_eq!(lang.region(), Some("CN"));

        let lang = locale_to_language("en").unwrap();
        assert_eq!(lang.iso639(), ISO639::En);
        assert_eq!(lang.region(), None);

        assert!(locale_to_language("invalid").is_none());
        assert!(locale_to_language("xx-YY").is_none());
    }

    #[test]
    fn test_get_preferred_languages() {
        let languages = get_preferred_languages();
        println!("Preferred languages: {:?}", languages);
        assert!(!languages.is_empty());
    }

    #[test]
    fn test_get_current_locale_identifier() {
        let locale = get_current_locale_identifier();
        println!("Current locale: {}", locale);
        assert!(!locale.is_empty());
    }
}
