use rand::Rng;

#[allow(dead_code)]
pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[allow(dead_code)]
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
        .collect()
}

#[allow(dead_code)]
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.len() > 3
}

#[allow(dead_code)]
pub fn format_file_path(user_id: &str, path: &str) -> String {
    format!("user-{}/{}", user_id, path.trim_start_matches('/'))
}
