pub const DEFAULT_BASE_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RAWSSG Site</title>
    <link rel="stylesheet" href="/style/main.css">
</head>
<body>
    <main>
        {{ content }}
    </main>
    <script src="/scripts/scripts.js"></script>
</body>
</html>"#;

pub const DEFAULT_INDEX_HTML: &str = r#"<article>
    {{ content }}
</article>"#;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");