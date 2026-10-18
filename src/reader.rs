//! HTML parsing, semantic extraction, and AST representation.

use scraper::{Html, Selector};
use crate::document::*;


/// The reader pipeline that fetches and parses a web page into a VellumDocument.
pub struct Reader {
    strip_selector: Selector,
}

impl Reader {
    pub fn new() -> Self {
        Reader {
            strip_selector: Selector::parse("script, style, nav, aside, footer, header, form, button")
                .expect("invalid selector"),
        }
    }

    /// Parse an HTML document string into a VellumDocument.
    pub fn parse_html(&self, html: &str, base_url: &str) -> VellumDocument {
        let document = Html::parse_document(html);
        let mut doc = VellumDocument::new();

        doc.title = document
            .select(&Selector::parse("title").unwrap())
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_else(|| "Untitled".to_string());

        self.extract_author(&document, &mut doc);
        self.extract_content(&document, base_url, &mut doc);

        doc
    }

    /// Fetch and parse a URL over HTTPS.
    pub fn fetch(&self, url: &str) -> Result<VellumDocument, String> {
        let parsed = url::Url::parse(url).map_err(|e| format!("Invalid URL: {}", e))?;
        let scheme = parsed.scheme();

        if scheme != "https" {
            return Err("Vellum requires HTTPS connections. Use http:// URLs at your own risk.".to_string());
        }

        let response = ureq::get(parsed.as_str()).call();

        match response {
            Ok(req) => {
                let status = req.status();
                let mut body = String::new();
                req.into_reader().read_to_string(&mut body).map_err(|e| e.to_string())?;
                if status >= 400 {
                    return Err(format!("HTTP error {}: {}", status, body.chars().take(200).collect::<String>()));
                }
                Ok(self.parse_html(&body, parsed.as_str()))
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    fn extract_author(&self, document: &Html, doc: &mut VellumDocument) {
        if let Some(author) = document.select(&Selector::parse("meta[name=\"author\"]").unwrap()).next() {
            if let Some(content) = author.attr("content") {
                if !content.is_empty() {
                    doc.author = Some(content.to_string());
                    return;
                }
            }
        }
        if let Some(canonical) = document.select(&Selector::parse("link[rel=\"canonical\"]").unwrap()).next() {
            if let Some(href) = canonical.attr("href") {
                if let Ok(url) = url::Url::parse(href) {
                    doc.author = Some(url.domain().unwrap_or("Unknown").to_string());
                    return;
                }
            }
        }
    }

    fn extract_content(&self, document: &Html, base_url: &str, doc: &mut VellumDocument) {
        let content = document.select(&Selector::parse("main, article, [role=\"main\"], #content, .content, .post, .article")
            .unwrap())
            .next()
            .or_else(|| document.select(&Selector::parse("body").unwrap()).next());

        let content = match content {
            Some(el) => el,
            None => return,
        };

        self.extract_links(&content, base_url, doc);
        self.process_blocks(&content, doc);
    }

    fn extract_links(&self, element: &scraper::ElementRef, base_url: &str, doc: &mut VellumDocument) {
        for (idx, link) in element.select(&Selector::parse("a").unwrap()).enumerate() {
            let href = link.attr("href").unwrap_or("").to_string();
            let text = link.inner_html();
            if href.is_empty() { continue; }
            let full_url = self.resolve_url(&href, base_url);
            doc.links.push(LinkInfo {
                index: idx,
                url: full_url,
                label: text,
                row: 0,
                col: 0,
            });
        }
    }

    fn resolve_url(&self, href: &str, base_url: &str) -> String {
        if href.is_empty() { return String::new(); }
        if href.starts_with("//") {
            if let Ok(parsed) = url::Url::parse(base_url) { return format!("{}://{}", parsed.scheme(), href); }
            return href.to_string();
        }
        if href.starts_with('/') {
            if let Ok(parsed) = url::Url::parse(base_url) { return format!("{}{}", parsed.scheme(), href); }
            return href.to_string();
        }
        if href.starts_with('#') {
            if let Ok(parsed) = url::Url::parse(base_url) { return parsed.join(href).map(|u| u.to_string()).unwrap_or(href.to_string()); }
            return href.to_string();
        }
        if let Ok(parsed) = url::Url::parse(href) { return parsed.to_string(); }
        if let Ok(parsed) = url::Url::parse(base_url) { return parsed.join(href).map(|u| u.to_string()).unwrap_or(href.to_string()); }
        href.to_string()
    }

    fn process_blocks(&self, element: &scraper::ElementRef, doc: &mut VellumDocument) {
        for child in element.child_elements() {
            let tag = child.value().name();
            match tag {
                "h1" => { self.process_heading(&child, 1, doc); }
                "h2" => { self.process_heading(&child, 2, doc); }
                "h3" => { self.process_heading(&child, 3, doc); }
                "h4" => { self.process_heading(&child, 4, doc); }
                "h5" => { self.process_heading(&child, 5, doc); }
                "h6" => { self.process_heading(&child, 6, doc); }
                "p" => { self.process_paragraph(&child, doc); }
                "pre" | "code" => { self.process_code_block(&child, doc); }
                "blockquote" => { self.process_blockquote(&child, doc); }
                "img" => { self.process_image(&child, doc); }
                "table" => { self.process_table(&child, doc); }
                "li" => { self.process_list(&child, doc); }
                "div" => {
                    if let Some(cls) = child.attr("class") {
                        if cls.contains("article") || cls.contains("post") || cls.contains("entry") {
                            self.process_blocks(&child, doc);
                        }
                    } else {
                        self.process_blocks(&child, doc);
                    }
                }
                _ => {}
            }
        }
    }

    fn process_heading(&self, element: &scraper::ElementRef, level: u8, doc: &mut VellumDocument) {
        let text = element.inner_html();
        let stripped = self.strip_styles(&text);
        if !stripped.trim().is_empty() {
            let style = if level <= 2 { TextStyle::Bold } else { TextStyle::Normal };
            doc.blocks.push(Block::heading(level, &stripped, style));
        }
    }

    fn process_paragraph(&self, element: &scraper::ElementRef, doc: &mut VellumDocument) {
        let text = element.inner_html();
        let stripped = self.strip_styles(&text);
        if !stripped.is_empty() {
            doc.blocks.push(Block::paragraph(vec![
                Span::text(&stripped, TextStyle::Normal),
            ]));
        }
    }

    fn process_code_block(&self, element: &scraper::ElementRef, doc: &mut VellumDocument) {
        let code = element.inner_html();
        let language = element.attr("class")
            .and_then(|c| c.split_whitespace().find(|p| p.starts_with("language-")))
            .map(|s| s.trim_start_matches("language-").to_string());
        if !code.trim().is_empty() {
            doc.blocks.push(Block::code_block(language.as_deref(), &code));
        }
    }

    fn process_blockquote(&self, element: &scraper::ElementRef, doc: &mut VellumDocument) {
        let text = element.inner_html();
        let stripped = self.strip_styles(&text);
        if !stripped.trim().is_empty() {
            doc.blocks.push(Block::blockquote(vec![
                Span::text(stripped.trim(), TextStyle::Normal),
            ]));
        }
    }

    fn process_image(&self, element: &scraper::ElementRef, doc: &mut VellumDocument) {
        let src = element.attr("src").unwrap_or("").to_string();
        let alt = element.attr("alt").unwrap_or("").to_string();
        if src.is_empty() { return; }
        let full_url = self.resolve_url(&src, "");
        let width = element.attr("width")
            .and_then(|w| w.parse().ok())
            .unwrap_or(0);
        let height = element.attr("height")
            .and_then(|h| h.parse().ok())
            .unwrap_or(0);
        if !full_url.is_empty() {
            doc.blocks.push(Block::image(&full_url, &alt, width, height));
        }
    }

    fn process_table(&self, element: &scraper::ElementRef, doc: &mut VellumDocument) {
        let mut headers = Vec::new();
        let mut rows = Vec::new();
        for (row_idx, row) in element.select(&Selector::parse("tr").unwrap()).enumerate() {
            let cells: Vec<String> = row.select(&Selector::parse("th, td").unwrap())
                .map(|c| c.inner_html())
                .collect();
            if row_idx == 0 {
                for c in cells { if !c.is_empty() { headers.push(c); } }
            } else {
                rows.push(cells);
            }
        }
        if !headers.is_empty() || !rows.is_empty() {
            doc.blocks.push(Block::table(headers, rows));
        }
    }

    fn process_list(&self, element: &scraper::ElementRef, doc: &mut VellumDocument) {
        let items: Vec<String> = element.select(&Selector::parse("li").unwrap())
            .map(|li| li.inner_html())
            .filter(|t| !t.is_empty())
            .collect();
        if !items.is_empty() {
            let spans: Vec<Span> = items.into_iter().map(|s| Span::text(&s, TextStyle::Normal)).collect();
            doc.blocks.push(Block::paragraph(spans));
        }
    }

    fn strip_styles(&self, html: &str) -> String {
        let doc = Html::parse_document(html);
        let mut result = String::new();
        for node in doc.select(&Selector::parse("*").unwrap()) {
            let tag = node.value().name();
            if tag == "script" || tag == "style" { continue; }
            if tag == "div" || tag == "br" || tag == "p" || tag == "li" {
                result.push(' ');
            } else {
                if let Some(text) = node.inner_html().split_whitespace().next() {
                    result.push_str(text);
                }
            }
        }
        let words: Vec<&str> = result.split_whitespace().collect();
        words.join(" ")
    }
}

impl Default for Reader {
    fn default() -> Self {
        Self::new()
    }
}
