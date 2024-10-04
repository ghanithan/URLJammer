use std::iter::repeat_with;

use crate::settings::SETTINGS;

pub type ArcStr = std::sync::Arc<str>;

pub fn slug_generator() -> ArcStr {
    let slug: String = repeat_with(fastrand::alphanumeric)
        .take(SETTINGS.shortner.slug_length)
        .collect();
    println!("Slug: {}", slug);
    slug.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Not;

    #[test]
    fn test_slug_generation() {
        let result = slug_generator();
        println!("Generated slug: {}", result);
        assert!(result.is_empty().not());
    }
}
