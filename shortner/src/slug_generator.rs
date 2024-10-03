use std::iter::repeat_with;

pub type ArcStr = std::sync::Arc<str>;

pub fn slug_generator() -> ArcStr {
    let slug: String = repeat_with(fastrand::alphanumeric).take(5).collect();
    println!("Slug: {}", slug);
    slug.into()
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use super::*;

    #[test]
    fn slug_generation_works() {
        let result = slug_generator();
        println!("Generated slug: {}", result);
        assert!(result.is_empty().not());
    }
}
