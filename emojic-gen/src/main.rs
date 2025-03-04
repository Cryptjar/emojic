#![feature(assoc_char_funcs)]
#![recursion_limit = "256"]

mod emoji;
mod gemoji;
mod strutil;

use emoji::Emojis;
use emoji::Group;
use emoji::Subgroup;
use inflections::case::to_snake_case;
use lazy_static::lazy_static;
use serde::Serialize;
use std::fmt;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
};

// mod constants;
use tera::Context;
use tera::Tera;

const EMOJI_URL: &str = "https://unicode.org/Public/emoji/13.1/emoji-test.txt";

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.tpl") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

struct Emojik(&'static str);

impl fmt::Display for Emojik {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{})", self.0)
    }
}

fn main() {
    println!("Fetching...");
    //let emoji_text = strutil::fetch_data(EMOJI_URL);
    let a = gemoji::fetch_gemoji();
    let mut e = fetch_emojis().unwrap();
    //dbg!(&a);

    println!("Sorting...");
    e.sort();

    let constants = generate_constants(&e);
    save_flat_constants(&constants);
    save_grouped_constants(&constants);

    save_aliasses(generate_aliases(&mut e, &a));
}

fn read_lines<'a>(content: &Vec<u8>, mut f: impl FnMut(&mut str) -> ()) {
    let reader = BufReader::new(&content[..]);
    for line in reader.lines().into_iter() {
        match line {
            Ok(mut l) => f(&mut l),
            Err(_) => (),
        }
    }
}

fn fetch_emojis() -> Result<Emojis, String> {
    let emoji_text = strutil::fetch_data(EMOJI_URL);

    let mut emojis: Emojis = Emojis::default();
    let mut current_group: String = String::new();
    let mut current_sub_group: String = String::new();

    read_lines(&emoji_text.unwrap(), |line| {
        let line = line.trim();
        println!("Process: {:?}", line);
        if line.is_empty() {
            // Just ignore it
        } else if line.starts_with("# group:") {
            let name = line.replace("# group:", "").trim().to_owned();
            emojis.append(name.to_owned()).unwrap();
            current_group = name.to_owned();
        } else if line.starts_with("# subgroup:") {
            let name = line.replace("# subgroup:", "").trim().to_owned();
            emojis
                .get_group(&current_group)
                .unwrap()
                .append(name.to_owned())
                .unwrap();
            current_sub_group = name.to_owned();
        } else if !line.starts_with('#') {
            //println!("Current group : {} subgroup {}", current_group,current_sub_group);
            emojis
                .get_group(&current_group)
                .unwrap()
                .get_subgroup(&current_sub_group)
                .unwrap()
                .append_line(line);
        }
    });
    Ok(emojis)
}

fn emojis_for_sub_group_list(sub: &Subgroup) -> Vec<String> {
    sub.constants
        .iter()
        .flat_map(|c| sub.get_emoji(c).unwrap().default_grapheme())
        .map(|s| s.to_string())
        .take(3)
        .collect()
}

fn emojis_for_subgroup(sub: &Subgroup) -> String {
    emojis_for_sub_group_list(sub).join("")
}

fn emojis_for_group(grp: &Group) -> String {
    grp.subgroups
        .iter()
        .take(3)
        .flat_map(|sub| emojis_for_sub_group_list(sub).get(0).cloned())
        .collect()
}

#[derive(Debug, Clone, Serialize)]
struct GroupedConstant<'a> {
    pub identifier: &'a str,
    pub preview_emojis: String,
    pub subgroups: Vec<SubgroupConstant<'a>>,
}

#[derive(Debug, Clone, Serialize)]
struct SubgroupConstant<'a> {
    pub identifier: &'a str,
    pub preview_emojis: String,
    pub emojis: Vec<EmojiConstant<'a>>,
}

#[derive(Debug, Clone, Serialize)]
struct EmojiConstant<'a> {
    pub identifier: &'a str,
    pub preview_emojis: String,
    pub source_code: String,
    pub full_list_accessors: Vec<String>,
    pub default_list_accessors: Vec<String>,
}

fn generate_constants(e: &Emojis) -> Vec<GroupedConstant> {
    // Collect all groups
    e.groups
        .iter()
        .map(|g| {
            // Collect all subgroups
            let subgroups = g
                .subgroups
                .iter()
                .map(|s| {
                    // Collect all emojis
                    let emojis = s
                        .emoji_iter()
                        .map(|emoji| {
                            println!("Writing emoji {:?}", emoji.identifier());

                            let full_list_accessors = emoji
                                .full_emoji_list()
                                .into_iter()
                                .map(|(acc, _, _)| acc)
                                .collect();

                            let default_list_accessors = emoji
                                .default_emoji_list()
                                .into_iter()
                                .map(|(acc, _, _)| acc)
                                .collect();

                            EmojiConstant {
                                identifier: emoji.identifier(),
                                preview_emojis: emoji.graphemes(),
                                source_code: emoji.to_source_code(),
                                full_list_accessors,
                                default_list_accessors,
                            }
                        })
                        .collect();

                    SubgroupConstant {
                        identifier: &s.identifier,
                        preview_emojis: emojis_for_subgroup(s),
                        emojis,
                    }
                })
                .collect();

            GroupedConstant {
                identifier: &g.identifier,
                preview_emojis: emojis_for_group(g),
                subgroups,
            }
        })
        .collect()
}

fn generate_aliases(emoji: &mut Emojis, gemojis: &HashMap<String, String>) -> String {
    let mut aliasses: Vec<String> = Vec::new();
    let mut emoji_map: HashMap<String, String> = HashMap::new();
    let mut emoji_map_by_grapheme: HashMap<String, String> = HashMap::new();

    emoji.groups.iter_mut().for_each(|g| {
        g.subgroups.iter_mut().for_each(|s| {
            s.constants.iter().for_each(|c| {
                let em = s.get_emoji(c).unwrap();
                let alias = gemoji::make_alias(&to_snake_case(&em.identifier()));

                // Add the graphemes of all variants
                for (const_accessor, pub_accessor, grapheme) in em.full_emoji_list() {
                    emoji_map_by_grapheme.insert(grapheme.to_string(), const_accessor);
                }

                // Add an alias for the default name
                if let Some(def) = em.default_grapheme() {
                    aliasses.push(alias.clone());
                    emoji_map.insert(alias, emoji_map_by_grapheme[def].clone());
                }
            });
        })
    });

    gemojis.iter().for_each(|(key, val)| {
        if !emoji_map.contains_key(key) {
            if let Some(emoji) = emoji_map_by_grapheme.get(val) {
                emoji_map.insert(key.clone(), emoji.clone());
                aliasses.push(key.clone());
            } else {
                println!("Couldn't find emoji for {:?} ({})", key, val);
            }
        }
    });

    aliasses[..].sort();
    aliasses = aliasses
        .iter_mut()
        .map(|al| {
            format!(
                "(\"{}\" , &crate::flat::{} as &crate::Emoji),\n",
                al,
                emoji_map.get(al).unwrap()
            )
        })
        .collect::<Vec<String>>();

    aliasses[..].join("")
}

fn save_flat_constants(constants: &[GroupedConstant]) {
    let mut context = Context::new();

    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();

    let today = format!("{}", now);
    context.insert("Link", EMOJI_URL);
    context.insert("Date", &today);
    context.insert("Constants", &constants);

    let bytes = TEMPLATES
        .render("flat.tpl", &context)
        .expect("Failed to render flat");
    File::create("./flat.rs")
        .unwrap()
        .write_all(bytes.as_bytes());
}

fn save_grouped_constants(constants: &[GroupedConstant]) {
    let mut context = Context::new();

    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();

    let today = format!("{}", now);
    context.insert("Link", EMOJI_URL);
    context.insert("Date", &today);
    context.insert("Constants", &constants);

    let bytes = TEMPLATES
        .render("grouped.tpl", &context)
        .expect("Failed to render grouped");
    File::create("./grouped.rs")
        .unwrap()
        .write_all(bytes.as_bytes());
}

fn save_aliasses(aliasses: String) {
    let mut context = Context::new();

    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();

    let today = format!("{}", now);
    context.insert("Link", gemoji::GEMOJI_URL);
    context.insert("Date", &today);
    context.insert("Data", &aliasses);

    let bytes = TEMPLATES
        .render("alias.tpl", &context)
        .expect("Failed to render alias");
    File::create("./alias.rs")
        .unwrap()
        .write_all(bytes.as_bytes());
}
