use rand::seq::SliceRandom;

pub fn random_user_agent() -> String {
    let chrome_versions = vec![
        "90.0.4430.212",
        "90.0.4430.24",
        "90.0.4430.70",
        "90.0.4430.72",
        "90.0.4430.85",
        "90.0.4430.93",
        "91.0.4472.101",
        "91.0.4472.106",
        "91.0.4472.114",
        "91.0.4472.124",
        "91.0.4472.164",
        "91.0.4472.19",
        "91.0.4472.77",
        "92.0.4515.107",
        "92.0.4515.115",
        "92.0.4515.131",
        "92.0.4515.159",
        "92.0.4515.43",
        "93.0.4556.0",
        "93.0.4577.15",
        "93.0.4577.63",
        "93.0.4577.82",
        "94.0.4606.41",
        "94.0.4606.54",
        "94.0.4606.61",
        "94.0.4606.71",
        "94.0.4606.81",
        "94.0.4606.85",
        "95.0.4638.17",
        "95.0.4638.50",
        "95.0.4638.54",
        "95.0.4638.69",
        "95.0.4638.74",
        "96.0.4664.18",
        "96.0.4664.45",
        "96.0.4664.55",
        "96.0.4664.93",
        "97.0.4692.20",
    ];
    format!("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36",chrome_versions.choose(&mut rand::thread_rng()).unwrap())
}

pub const SAFARI_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:10.0) AppleWebKit/533.20.25 (KHTML, like Gecko) Version/5.0.4 Safari/533.20.27";

pub fn supported_encodings() -> Vec<&'static str> {
    let default_supported_encodings = if cfg!(feature = "brotli") {
        vec!["gzip", "deflate", "brotli"]
    } else {
        vec!["gzip", "deflate"]
    };

    return default_supported_encodings;
}

#[cfg(test)]
mod tests {
    use super::supported_encodings;

    #[test]
    fn test_supported_encodings() {
        let result = supported_encodings();
        assert_eq!(
            if cfg!(feature = "brotli") {
                vec!["gzip", "deflate", "brotli"]
            } else {
                vec!["gzip", "deflate"]
            },
            result
        )
    }
}
