use nipper::Document;
use std::fs;

// Header:
// number,chapter,grammar,english,freq,info,japanese_ex1,english_ex1,japanese_ex2,english_ex2,japanese_ex3,english_ex3,jlpt,bunpo,bunpro,a_dictionary_of,jpod,genki,nhk15,tae_kim
#[derive(Debug, serde::Deserialize)]
struct GrammarRecord {
    number: u32,
    chapter: Option<String>,
    grammar: String,
    english: String,
    freq: u32,
    info: String,
    japanese_ex1: String,
    english_ex1: String,
    japanese_ex2: String,
    english_ex2: String,
    japanese_ex3: String,
    english_ex3: String,
    jlpt: u32,
    bunpo: Option<u32>,
    bunpro: Option<u32>,
    a_dictionary_of: Option<String>,
    jpod: Option<u32>,
    genki: Option<u32>,
    nhk15: Option<u32>,
    tae_kim: Option<u32>,
}

fn main() {
    let doc = Document::from(&fs::read_to_string("../grammar-sanitized.html").unwrap());
    let mut rdr = csv::Reader::from_path("../Jo mako's japanese - Grammar.csv").unwrap();

    'outer: for row in rdr.deserialize().flat_map(|x| x) {
        let record: &GrammarRecord = &row;
        let reference = format!("{}　-　 {}", record.grammar, record.english);
        let headdings = doc.select("#introduction > h2");
        for mut head in headdings.iter() {
            if head.text().eq_ignore_ascii_case(&reference) {
                let value = ((record.freq as f32).log2() * 5.8).round() as u32;
                head.set_attr("freq", &format!("{}", value));
                head.set_attr("style", &format!("--freq: {}", value));
                println!("Found valid entry for {}", record.grammar);
                continue 'outer;
            }
        }
        println!("Could not find valid entry for {}", record.grammar);
    }

    doc.select("head")
        .first()
        .append_html("<link rel=\"stylesheet\" href=\"./styles/extra.css\" />");
    fs::write("../grammar.html", doc.html().as_bytes()).unwrap();
}
