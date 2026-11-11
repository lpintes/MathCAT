// *** Tool for translators: builds an HTML review package from a YAML file of review items ***
//
// Usage:
//   cargo run --bin review-package -- docs/sk/review/fractions.yaml [-o out.html] [--deadline 2026-12-01]
//
// Item file format (one file per topic):
//
//   topic: Fractions
//   known_issues:                  # optional, listed under "Known issues, do not report"
//     - Decimal numbers are not translated yet.
//   items:
//     - id: frac-001               # stable ID, never reused
//       note: Common fraction.     # optional note for reviewers
//       styles: [ClearSpeak]       # optional, default is ClearSpeak and SimpleSpeak
//       prefs:                     # optional preferences, e.g. ClearSpeak_Fractions: Ordinal
//         Verbosity: Verbose
//       mathml: <math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>
//       approved:                  # filled in after review, per style
//         ClearSpeak: jedna polovica
//         SimpleSpeak: jedna polovica
//
// Each item is spoken in the target language and in English, in each style. An item goes into
// the package when some style has no approved text yet or its output differs from the approved
// text. Approved, unchanged items are listed collapsed at the end.
#![allow(clippy::needless_return)]

use anyhow::{bail, Context, Result};
use clap::Parser;
use libmathcat::interface::*;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use yaml_rust::{Yaml, YamlLoader};

const ALL_STYLES: [&str; 2] = ["ClearSpeak", "SimpleSpeak"];

#[derive(Parser)]
#[command(about = "Build an HTML review package from a YAML file of review items")]
struct Options {
    #[arg(value_name = "ITEMS_FILE", help = "YAML file with the review items of one topic")]
    items_file: PathBuf,

    #[arg(short, long, help = "Output HTML file [default: target/review/<ITEMS_FILE stem>.html]")]
    output: Option<PathBuf>,

    #[arg(short, long, default_value = "sk", help = "Language under review")]
    language: String,

    #[arg(long, default_value = "en", help = "Language shown for comparison")]
    compare_language: String,

    #[arg(short, long, help = "Rules directory [default: Rules in the source tree]")]
    rules_dir: Option<PathBuf>,

    #[arg(long, help = "Reply deadline shown in the package header")]
    deadline: Option<String>,
}

struct Item {
    id: String,
    note: Option<String>,
    styles: Vec<String>,
    prefs: Vec<(String, String)>,
    mathml: String,
    approved: Vec<(String, String)>,
}

struct ItemFile {
    topic: String,
    known_issues: Vec<String>,
    items: Vec<Item>,
}

#[derive(PartialEq, Eq)]
enum Status {
    New,
    Changed,
    Approved,
}

struct StyleResult {
    style: String,
    speech: String,
    compare_speech: String,
    approved: Option<String>,
    status: Status,
}

fn yaml_scalar_to_string(value: &Yaml) -> Option<String> {
    return match value {
        Yaml::String(s) => Some(s.clone()),
        Yaml::Integer(i) => Some(i.to_string()),
        Yaml::Real(r) => Some(r.clone()),
        Yaml::Boolean(b) => Some(b.to_string()),
        _ => None,
    };
}

fn yaml_string_list(value: &Yaml, what: &str) -> Result<Vec<String>> {
    return match value {
        Yaml::BadValue => Ok(vec![]),
        Yaml::Array(list) => list.iter()
            .map(|v| yaml_scalar_to_string(v).with_context(|| format!("'{what}' must contain strings")))
            .collect(),
        _ => bail!("'{what}' must be a list"),
    };
}

fn yaml_string_map(value: &Yaml, what: &str) -> Result<Vec<(String, String)>> {
    return match value {
        Yaml::BadValue => Ok(vec![]),
        Yaml::Hash(map) => map.iter()
            .map(|(k, v)| {
                let key = yaml_scalar_to_string(k).with_context(|| format!("bad key in '{what}'"))?;
                let value = yaml_scalar_to_string(v).with_context(|| format!("bad value for '{key}' in '{what}'"))?;
                return Ok((key, value));
            })
            .collect(),
        _ => bail!("'{what}' must be a mapping"),
    };
}

fn parse_item(yaml: &Yaml, index: usize) -> Result<Item> {
    let id = yaml["id"].as_str().with_context(|| format!("item #{} has no 'id'", index + 1))?.to_string();
    let mathml = yaml["mathml"].as_str().with_context(|| format!("item {id} has no 'mathml'"))?.trim().to_string();
    let mut styles = yaml_string_list(&yaml["styles"], &format!("{id}: styles"))?;
    if styles.is_empty() {
        styles = ALL_STYLES.iter().map(|s| s.to_string()).collect();
    }
    if let Some(bad) = styles.iter().find(|s| !ALL_STYLES.contains(&s.as_str())) {
        bail!("item {id}: unknown speech style '{bad}'");
    }
    let approved = yaml_string_map(&yaml["approved"], &format!("{id}: approved"))?;
    if let Some((bad, _)) = approved.iter().find(|(s, _)| !styles.contains(s)) {
        bail!("item {id}: approved text for style '{bad}', which the item does not use");
    }
    let prefs = yaml_string_map(&yaml["prefs"], &format!("{id}: prefs"))?;
    return Ok(Item {
        id,
        note: yaml["note"].as_str().map(|s| s.trim().to_string()),
        styles,
        prefs,
        mathml,
        approved,
    });
}

fn read_item_file(path: &Path) -> Result<ItemFile> {
    let text = std::fs::read_to_string(path).with_context(|| format!("unable to read {}", path.display()))?;
    let docs = YamlLoader::load_from_str(&text).with_context(|| format!("YAML error in {}", path.display()))?;
    let doc = docs.first().with_context(|| format!("{} is empty", path.display()))?;
    let items = match &doc["items"] {
        Yaml::Array(list) => list.iter().enumerate().map(|(i, y)| parse_item(y, i)).collect::<Result<Vec<Item>>>()?,
        _ => bail!("{}: 'items' must be a list", path.display()),
    };
    let mut seen = std::collections::HashSet::new();
    for item in &items {
        if !seen.insert(item.id.as_str()) {
            bail!("duplicate item id '{}'", item.id);
        }
    }
    return Ok(ItemFile {
        topic: doc["topic"].as_str().unwrap_or("(no topic)").to_string(),
        known_issues: yaml_string_list(&doc["known_issues"], "known_issues")?,
        items,
    });
}

// Same defaults as the language tests (tests/common/mod.rs), so the package shows what the goldens check.
fn set_base_prefs() -> Result<()> {
    set_preference("DecimalSeparator", "Auto")?;
    set_preference("SpeechOverrides_CapitalLetters", "")?;
    set_preference("MathRate", "100")?;
    set_preference("PauseFactor", "100")?;
    set_preference("Verbosity", "Medium")?;
    set_preference("Impairment", "Blindness")?;
    set_preference("TTS", "None")?;
    return Ok(());
}

// Collapse runs of spaces, as the tests do before comparing.
fn normalize_speech(speech: &str) -> String {
    return speech.split(' ').filter(|s| !s.is_empty()).collect::<Vec<&str>>().join(" ");
}

fn speak(item: &Item, language: &str, style: &str) -> String {
    let result = (|| -> Result<String> {
        set_preference("Language", language)?;
        set_preference("SpeechStyle", style)?;
        let mut saved = Vec::with_capacity(item.prefs.len());
        for (name, value) in &item.prefs {
            saved.push((name.clone(), get_preference(name)?));
            set_preference(name, value)?;
        }
        let speech = set_mathml(&item.mathml).and_then(|_| get_spoken_text());
        for (name, value) in saved.iter().rev() {
            set_preference(name, value)?;
        }
        return Ok(normalize_speech(&speech?));
    })();
    return match result {
        Ok(speech) => speech,
        Err(e) => format!("ERROR: {}", errors_to_string(&e).trim()),
    };
}

fn evaluate(item: &Item, language: &str, compare_language: &str) -> Vec<StyleResult> {
    return item.styles.iter().map(|style| {
        let speech = speak(item, language, style);
        let approved = item.approved.iter().find(|(s, _)| s == style).map(|(_, text)| text.clone());
        let status = match &approved {
            None => Status::New,
            Some(text) if normalize_speech(text) == speech => Status::Approved,
            Some(_) => Status::Changed,
        };
        return StyleResult {
            style: style.clone(),
            speech,
            compare_speech: speak(item, compare_language, style),
            approved,
            status,
        };
    }).collect();
}

fn esc(text: &str) -> String {
    return html_escape::encode_text(text).to_string();
}

fn write_item(html: &mut String, item: &Item, results: &[StyleResult], language: &str, compare_language: &str) {
    let changed = results.iter().any(|r| r.status == Status::Changed);
    let _ = writeln!(html, "<h2 id=\"{0}\">{0}{1}</h2>", esc(&item.id), if changed { " (changed)" } else { "" });
    if let Some(note) = &item.note {
        let _ = writeln!(html, "<p>{}</p>", esc(note));
    }
    if !item.prefs.is_empty() {
        let prefs: Vec<String> = item.prefs.iter().map(|(k, v)| format!("{k} = {v}")).collect();
        let _ = writeln!(html, "<p>Settings: {}</p>", esc(&prefs.join(", ")));
    }
    let _ = writeln!(html, "<div class=\"math\">{}</div>", item.mathml);
    let _ = writeln!(html, "<ul>");
    for r in results {
        let _ = writeln!(html, "<li>{} ({}): <span lang=\"{}\">{}</span></li>",
                         r.style, language, language, esc(&r.speech));
        if r.status == Status::Changed {
            let _ = writeln!(html, "<li>{} ({}), previously approved: <span lang=\"{}\">{}</span></li>",
                             r.style, language, language, esc(r.approved.as_deref().unwrap_or_default()));
        }
    }
    for r in results {
        let _ = writeln!(html, "<li>{} ({}): <span lang=\"{}\">{}</span></li>",
                         r.style, compare_language, compare_language, esc(&r.compare_speech));
    }
    let _ = writeln!(html, "</ul>");
}

fn build_html(file: &ItemFile, cli: &Options) -> (String, usize, usize) {
    let mut to_review = String::new();
    let mut approved = String::new();
    let mut n_review = 0;
    let mut n_approved = 0;
    for item in &file.items {
        let results = evaluate(item, &cli.language, &cli.compare_language);
        if results.iter().all(|r| r.status == Status::Approved) {
            n_approved += 1;
            let texts: Vec<String> = results.iter().map(|r| format!("{}: {}", r.style, r.speech)).collect();
            let _ = writeln!(approved, "<li>{}: <span lang=\"{}\">{}</span></li>",
                             esc(&item.id), cli.language, esc(&texts.join("; ")));
        } else {
            n_review += 1;
            write_item(&mut to_review, item, &results, &cli.language, &cli.compare_language);
        }
    }

    let title = format!("MathCAT {} speech review: {}", cli.language, file.topic);
    let mut html = String::new();
    let _ = writeln!(html, "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">");
    let _ = writeln!(html, "<title>{}</title>", esc(&title));
    let _ = writeln!(html, "<style>.math {{ font-size: 150%; margin: 0.5em 0; }}</style>\n</head>\n<body>");
    let _ = writeln!(html, "<h1>{}</h1>", esc(&title));
    let _ = writeln!(html, "<p>Generated by MathCAT {}. Items to review: {}. Already approved: {}.</p>",
                     esc(&get_version()), n_review, n_approved);
    if let Some(deadline) = &cli.deadline {
        let _ = writeln!(html, "<p>Please reply by {}.</p>", esc(deadline));
    }
    let _ = writeln!(html, "<p>Each item is a level 2 heading with its ID. Reply only about items you want \
                            to change: one line per item with the item ID, the word concerned and your proposal. \
                            Items you do not mention count as OK.</p>");
    if !file.known_issues.is_empty() {
        let _ = writeln!(html, "<h2>Known issues, do not report</h2>\n<ul>");
        for issue in &file.known_issues {
            let _ = writeln!(html, "<li>{}</li>", esc(issue));
        }
        let _ = writeln!(html, "</ul>");
    }
    html.push_str(&to_review);
    if n_approved > 0 {
        let _ = writeln!(html, "<details>\n<summary>Approved, unchanged items ({n_approved})</summary>\n<ul>");
        html.push_str(&approved);
        let _ = writeln!(html, "</ul>\n</details>");
    }
    let _ = writeln!(html, "</body>\n</html>");
    return (html, n_review, n_approved);
}

fn main() -> Result<()> {
    let cli = Options::parse();
    let file = read_item_file(&cli.items_file)?;

    let rules_dir = match &cli.rules_dir {
        Some(path) => path.to_str().context("Rules directory path must be valid UTF-8")?.to_string(),
        None => Path::new(env!("CARGO_MANIFEST_DIR")).join("Rules").to_str()
                    .context("Rules path must be valid UTF-8")?.to_string(),
    };
    set_rules_dir(&rules_dir).map_err(|e| anyhow::anyhow!(errors_to_string(&e)))
        .with_context(|| format!("unable to use Rules dir {rules_dir}"))?;
    set_base_prefs()?;

    // The MathML is copied into the HTML as is, so broken markup would break the rest of the page.
    for item in &file.items {
        if let Err(e) = set_mathml(&item.mathml) {
            bail!("item {}: {}", item.id, errors_to_string(&e).trim());
        }
    }

    let (html, n_review, n_approved) = build_html(&file, &cli);

    let output = match &cli.output {
        Some(path) => path.clone(),
        None => {
            let stem = cli.items_file.file_stem().context("items file has no name")?;
            Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join("review").join(stem).with_extension("html")
        }
    };
    if let Some(dir) = output.parent() {
        std::fs::create_dir_all(dir).with_context(|| format!("unable to create {}", dir.display()))?;
    }
    std::fs::write(&output, html).with_context(|| format!("unable to write {}", output.display()))?;
    println!("{}: {} items to review, {} approved", output.display(), n_review, n_approved);
    return Ok(());
}
