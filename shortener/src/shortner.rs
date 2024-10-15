use storage::{HandleStorage, Storage};

use crate::slug_generator;

pub struct Shortener {
    pub storage_stub: Storage,
}

impl Shortener {
    pub fn new() -> Self {
        Self {
            storage_stub: Storage::init(),
        }
    }

    pub fn create(&self, destination_url: String) -> Result<String, anyhow::Error> {
        let slug = slug_generator();
        let content = r#"
            <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <link rel="stylesheet" href="/assets/main.css">

                <meta http-equiv="refresh" content="{{ time }};url={{ page.target }}"/>
                <link rel="canonical" href="{{ page.target }}"/>
                <title>{{ title }}</title>
                <style>
                    body {
                        font-family: sans-serif;
                        max-width: 40em;
                        margin: 1em auto;
                    }
                </style>
            </head>
            <body>
                <h1 class="site-title">{{ title }}</h1>


                <p>{{ page.message }}</p>

                <p>This document has moved!</p>

                <p>Redirecting to <a href="{{ page.target }}">{{ targetname }}</a> in {{ time }} seconds.</p>

            </body>
            </html>
            "#;
        _ = self
            .storage_stub
            .create_asset(slug.clone(), "index.html".to_string(), content)?;
        Ok(slug)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_shorturl() {
        let shortner = Shortener::new();
        let shortened_url = shortner
            .create("https://ghanithan.com".to_string())
            .unwrap();
        assert!(!shortened_url.is_empty())
    }
}
