use encoding_rs::{Encoding, BIG5, GB18030, UTF_16BE, UTF_16LE, UTF_8, WINDOWS_1252};
use serde::{Deserialize, Serialize};

const DETECT_SAMPLE: usize = 256 * 1024;
const PREVIEW_SAMPLE: usize = 32 * 1024;
const PREVIEW_LINES: usize = 10;

#[derive(Debug, Clone)]
pub struct DecodeOutcome {
    pub text: String,
    pub encoding: String,
    pub low_confidence: bool,
    pub used_hint: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodingPreview {
    pub encoding: String,
    pub display_name: String,
    pub preview: String,
    pub recommended: bool,
}

struct Scored {
    enc: &'static Encoding,
    score: i64,
    titles: u32,
    cjk: u32,
    latin: u32,
    replacement_ratio: f64,
    had_errors: bool,
}

const CANDIDATES: &[&'static Encoding] = &[
    UTF_8,
    GB18030,
    BIG5,
    UTF_16LE,
    UTF_16BE,
    WINDOWS_1252,
];

pub fn detect_and_decode(
    bytes: &[u8],
    hint: Option<&str>,
    is_title: impl Fn(&str) -> bool,
) -> Result<DecodeOutcome, String> {
    if bytes.is_empty() {
        return Ok(DecodeOutcome {
            text: String::new(),
            encoding: label_of(UTF_8).to_string(),
            low_confidence: false,
            used_hint: false,
        });
    }

    if let Some((enc, bom_len)) = detect_bom(bytes) {
        let (text, _) = decode_to_string(enc, &bytes[bom_len..]);
        return Ok(DecodeOutcome {
            text: strip_bom_char(text),
            encoding: label_of(enc).to_string(),
            low_confidence: false,
            used_hint: false,
        });
    }

    if let Ok(text) = std::str::from_utf8(bytes) {
        return Ok(DecodeOutcome {
            text: strip_bom_char(text.to_string()),
            encoding: label_of(UTF_8).to_string(),
            low_confidence: false,
            used_hint: false,
        });
    }

    if let Some(label) = hint {
        if let Some(enc) = encoding_from_label(label) {
            let (text, _) = decode_to_string(enc, bytes);
            return Ok(DecodeOutcome {
                text: strip_bom_char(text),
                encoding: label_of(enc).to_string(),
                low_confidence: false,
                used_hint: true,
            });
        }
    }

    let mut scored = score_candidates(sample(bytes, DETECT_SAMPLE), &is_title);
    scored.sort_by(|a, b| b.score.cmp(&a.score));
    let best = scored.first().ok_or_else(|| "无法识别文本编码".to_string())?;
    let low_confidence = is_low_confidence(best);
    let (text, _) = decode_to_string(best.enc, bytes);

    Ok(DecodeOutcome {
        text: strip_bom_char(text),
        encoding: label_of(best.enc).to_string(),
        low_confidence,
        used_hint: false,
    })
}

pub fn decode_with_label(bytes: &[u8], label: &str) -> Result<DecodeOutcome, String> {
    let enc = encoding_from_label(label).ok_or_else(|| format!("不支持的编码：{label}"))?;
    let (text, _) = decode_to_string(enc, bytes);
    Ok(DecodeOutcome {
        text: strip_bom_char(text),
        encoding: label_of(enc).to_string(),
        low_confidence: false,
        used_hint: false,
    })
}

pub fn preview_encodings(bytes: &[u8], is_title: impl Fn(&str) -> bool) -> Vec<EncodingPreview> {
    let sample_bytes = sample(bytes, PREVIEW_SAMPLE);
    let mut scored = score_candidates(sample_bytes, &is_title);
    if scored.is_empty() {
        return Vec::new();
    }
    scored.sort_by(|a, b| b.score.cmp(&a.score));
    let best_label = label_of(scored[0].enc);

    let mut previews: Vec<EncodingPreview> = CANDIDATES
        .iter()
        .map(|enc| {
            let (text, _) = decode_to_string(enc, sample_bytes);
            let encoding = label_of(enc).to_string();
            EncodingPreview {
                display_name: display_name(&encoding),
                recommended: encoding == best_label,
                preview: first_lines(&text, PREVIEW_LINES),
                encoding,
            }
        })
        .collect();
    previews.sort_by_key(|p| !p.recommended);
    previews
}

fn score_candidates(bytes: &[u8], is_title: impl Fn(&str) -> bool) -> Vec<Scored> {
    CANDIDATES
        .iter()
        .map(|enc| {
            let (text, had_errors) = decode_to_string(enc, bytes);
            score_text(enc, &text, had_errors, &is_title)
        })
        .collect()
}

fn score_text(
    enc: &'static Encoding,
    text: &str,
    had_errors: bool,
    is_title: impl Fn(&str) -> bool,
) -> Scored {
    let mut replacement = 0u32;
    let mut cjk = 0u32;
    let mut latin = 0u32;
    let mut control = 0u32;
    let mut total = 0u32;

    for ch in text.chars() {
        total += 1;
        if ch == '\u{FFFD}' || ch == '\0' {
            replacement += 1;
        } else if is_cjk(ch) {
            cjk += 1;
        } else if ch.is_ascii_alphabetic() {
            latin += 1;
        } else if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
            control += 1;
        }
    }

    let titles = text
        .replace('\r', "")
        .split('\n')
        .filter(|line| is_title(line))
        .count() as u32;

    let total = total.max(1) as f64;
    let replacement_ratio = replacement as f64 / total;

    let mut score = 0i64;
    score += cjk as i64 * 4;
    score += titles as i64 * 800;
    score += latin as i64;
    score -= replacement as i64 * 50;
    score -= control as i64 * 20;
    if had_errors && replacement > 2 {
        score -= 200;
    }

    Scored {
        enc,
        score,
        titles,
        cjk,
        latin,
        replacement_ratio,
        had_errors,
    }
}

fn is_low_confidence(best: &Scored) -> bool {
    if best.titles >= 1 {
        return false;
    }
    if best.cjk >= 30 && best.replacement_ratio < 0.01 {
        return false;
    }
    if best.latin >= 50 && best.replacement_ratio < 0.01 && best.cjk < 10 {
        return false;
    }
    if best.replacement_ratio > 0.03 {
        return true;
    }
    if best.cjk < 8 && best.latin < 20 {
        return true;
    }
    best.had_errors && best.replacement_ratio > 0.01
}

fn detect_bom(bytes: &[u8]) -> Option<(&'static Encoding, usize)> {
    if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
        return Some((UTF_8, 3));
    }
    if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
        return Some((UTF_16LE, 2));
    }
    if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
        return Some((UTF_16BE, 2));
    }
    None
}

fn decode_to_string(enc: &'static Encoding, bytes: &[u8]) -> (String, bool) {
    let (cow, had_errors) = enc.decode_without_bom_handling(bytes);
    (cow.into_owned(), had_errors)
}

fn sample(bytes: &[u8], n: usize) -> &[u8] {
    let len = bytes.len().min(n);
    &bytes[..len]
}

fn strip_bom_char(text: String) -> String {
    text.strip_prefix('\u{FEFF}')
        .map(|s| s.to_string())
        .unwrap_or(text)
}

fn is_cjk(ch: char) -> bool {
    matches!(
        ch,
        '\u{4E00}'..='\u{9FFF}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{3000}'..='\u{303F}'
            | '\u{FF00}'..='\u{FFEF}'
    )
}

fn first_lines(text: &str, n: usize) -> String {
    text.replace('\r', "")
        .split('\n')
        .take(n)
        .map(|line| {
            let line = line.replace('\0', "");
            let count = line.chars().count();
            if count > 80 {
                format!("{}…", line.chars().take(80).collect::<String>())
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn label_of(enc: &'static Encoding) -> &'static str {
    if enc == UTF_8 {
        "UTF-8"
    } else if enc == GB18030 {
        "GB18030"
    } else if enc == BIG5 {
        "Big5"
    } else if enc == UTF_16LE {
        "UTF-16LE"
    } else if enc == UTF_16BE {
        "UTF-16BE"
    } else if enc == WINDOWS_1252 {
        "windows-1252"
    } else {
        enc.name()
    }
}

pub fn display_name(label: &str) -> String {
    match label {
        "UTF-8" => "UTF-8".to_string(),
        "GB18030" => "GB18030（简体中文）".to_string(),
        "Big5" => "Big5（繁体中文）".to_string(),
        "UTF-16LE" => "UTF-16 LE".to_string(),
        "UTF-16BE" => "UTF-16 BE".to_string(),
        "windows-1252" => "Windows-1252".to_string(),
        _ => label.to_string(),
    }
}

pub fn encoding_from_label(label: &str) -> Option<&'static Encoding> {
    match label.trim().to_ascii_uppercase().as_str() {
        "UTF-8" | "UTF8" => Some(UTF_8),
        "GB18030" | "GBK" | "GB2312" => Some(GB18030),
        "BIG5" | "BIG-5" => Some(BIG5),
        "UTF-16LE" | "UTF16LE" | "UTF-16" => Some(UTF_16LE),
        "UTF-16BE" | "UTF16BE" => Some(UTF_16BE),
        "WINDOWS-1252" | "CP1252" => Some(WINDOWS_1252),
        _ => Encoding::for_label(label.as_bytes()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn looks_like_title(line: &str) -> bool {
        let t = line.trim();
        t.contains('第') && (t.contains('章') || t.contains('节'))
    }

    fn detect(bytes: &[u8]) -> DecodeOutcome {
        detect_and_decode(bytes, None, looks_like_title).unwrap()
    }

    #[test]
    fn utf8_plain() {
        let text = "第一章 开始\n你好，世界";
        let out = detect(text.as_bytes());
        assert_eq!(out.encoding, "UTF-8");
        assert!(out.text.contains("你好"));
        assert!(!out.low_confidence);
    }

    #[test]
    fn utf8_bom() {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice("序章\n正文".as_bytes());
        let out = detect(&bytes);
        assert_eq!(out.encoding, "UTF-8");
        assert!(out.text.starts_with("序章"));
    }

    #[test]
    fn gb18030_chinese_novel() {
        let src = "第一章 风起\n这是一段简体中文小说内容，用于测试编码识别。";
        let (bytes, _, _) = GB18030.encode(src);
        let out = detect(&bytes);
        assert_eq!(out.encoding, "GB18030");
        assert!(out.text.contains("风起"));
        assert!(!out.low_confidence);
    }

    #[test]
    fn big5_traditional() {
        let src = "第1章 開始\n這是繁體中文小說內容。";
        let (bytes, _, _) = BIG5.encode(src);
        let out = detect(&bytes);
        assert_eq!(out.encoding, "Big5");
        assert!(out.text.contains("繁體"));
    }

    fn encode_utf16le(s: &str) -> Vec<u8> {
        s.encode_utf16().flat_map(|u| u.to_le_bytes()).collect()
    }

    #[test]
    fn utf16le_bom() {
        let src = "第一章 测试\n你好";
        let mut bytes = vec![0xFF, 0xFE];
        bytes.extend_from_slice(&encode_utf16le(src));
        let out = detect(&bytes);
        assert_eq!(out.encoding, "UTF-16LE");
        assert!(out.text.contains("测试"));
    }

    #[test]
    fn utf16le_no_bom() {
        let src = "第一章 无BOM\n你好";
        let out = detect(&encode_utf16le(src));
        assert_eq!(out.encoding, "UTF-16LE");
        assert!(out.text.contains("无BOM"));
    }

    #[test]
    fn hint_is_used_when_not_utf8() {
        let src = "第一章 提示\n编码提示应直接生效。";
        let (bytes, _, _) = GB18030.encode(src);
        let out = detect_and_decode(&bytes, Some("GB18030"), looks_like_title).unwrap();
        assert_eq!(out.encoding, "GB18030");
        assert!(out.used_hint);
    }

    #[test]
    fn utf8_wins_over_gbk_hint() {
        let src = "第一章 已是UTF8\n内容";
        let out = detect_and_decode(src.as_bytes(), Some("GB18030"), looks_like_title).unwrap();
        assert_eq!(out.encoding, "UTF-8");
        assert!(!out.used_hint);
    }

    #[test]
    fn force_label_roundtrip() {
        let src = "第2章 强制\n简体";
        let (bytes, _, _) = GB18030.encode(src);
        let out = decode_with_label(&bytes, "GBK").unwrap();
        assert_eq!(out.encoding, "GB18030");
        assert!(out.text.contains("强制"));
    }
}
