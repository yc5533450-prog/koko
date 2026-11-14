//! Turkish language front end for CHIMERA.
//! Text -> articulatory IPA segments and sung syllables, without samples.
//! Approximations: ğ lengthens the previous vowel; stress defaults to last vowel.

use super::english::Phrase;
use super::syllables::{FrontEnd, syllabify_with};
use super::{Intonation, Segment, Syllable};

pub fn lower(input: &str) -> String {
    input.chars().flat_map(|c| match c {
        'I' => "ı".chars().collect::<Vec<_>>(),
        'İ' => "i".chars().collect::<Vec<_>>(),
        _ => c.to_lowercase().collect::<Vec<_>>(),
    }).collect()
}

pub fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || "çÇğĞıİöÖşŞüÜâÂîÎûÛ".contains(c)
}
fn vowel(c: char) -> bool {
    "aeıioöuüâîû".contains(c)
}
fn sound(c: char) -> Option<&'static str> {
    Some(match c {
        'a' | 'â' => "a", 'e' => "e", 'ı' => "ɯ", 'i' | 'î' => "i",
        'o' => "o", 'ö' => "ø", 'u' | 'û' => "u", 'ü' => "y",
        'b' => "b", 'c' => "dʒ", 'ç' => "tʃ", 'd' => "d", 'f' => "f",
        'g' => "g", 'h' => "h", 'j' => "ʒ", 'k' => "k", 'l' => "l",
        'm' => "m", 'n' => "n", 'p' => "p", 'r' => "ɾ", 's' => "s",
        'ş' => "ʃ", 't' => "t", 'v' => "v", 'y' => "j", 'z' => "z",
        // Orthographic loanwords are approximated.
        'q' => "k", 'w' => "v",
        _ => return None,
    })
}

/// Read a Turkish word; 'ğ' is a lengthening cue rather than an independent consonant.
pub fn word(input: &str) -> Vec<Segment> {
    let normalized = lower(input);
    let letters: Vec<char> = normalized.chars().filter(|&c| is_letter(c)).collect();
    let stressed = letters.iter().rposition(|&c| vowel(c));
    let mut out: Vec<Segment> = Vec::new();
    for (i, &c) in letters.iter().enumerate() {
        if c == 'ğ' {
            // In many contexts soft g lengthens the preceding vowel. Its exact
            // realisation varies by surrounding vowels and dialect.
            if let Some(prev) = out.last_mut() {
                if matches!(prev.ipa.as_str(), "a" | "e" | "ɯ" | "i" | "o" | "ø" | "u" | "y") {
                    prev.dur = Some(0.19);
                }
            }
            continue;
        }
        // Loanword x is two phonemes; a single "ks" inventory entry does not exist.
        if c == 'x' {
            out.push(Segment::new("k"));
            out.push(Segment::new("s"));
            continue;
        }
        let Some(ipa) = sound(c) else { continue };
        let mut segment = Segment::new(ipa);
        if vowel(c) {
            segment.stress = Some(u8::from(stressed == Some(i)));
        }
        out.push(segment);
    }
    out
}

/// Split Turkish text into phrases used by the existing CHIMERA speech planner.
pub fn to_phrases(input: &str) -> Vec<Phrase> {
    let mut output = Vec::new();
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let flush_word = |current: &mut String, words: &mut Vec<String>| {
        if !current.is_empty() { words.push(std::mem::take(current)); }
    };
    let flush_phrase = |punct: Option<char>, words: &mut Vec<String>, output: &mut Vec<Phrase>| {
        if words.is_empty() { return; }
        let mut segments = Vec::new();
        let mut spoken = Vec::new();
        for w in words.drain(..) {
            let mut s = word(&w);
            if let Some(last) = s.last_mut() {
                last.word_end = Some(true);
                segments.append(&mut s);
                spoken.push(w);
            }
        }
        if !segments.is_empty() {
            output.push(Phrase {
                segments, words: spoken,
                intonation: if punct == Some('?') { Intonation::Question } else { Intonation::Statement },
                pause: match punct { Some('.' | '!' | '?') => 0.35, Some(_) => 0.18, None => 0.3 },
            });
        }
    };
    for c in input.chars() {
        if is_letter(c) || c == '\'' || c == '’' || c == '-' {
            current.push(c);
        } else {
            flush_word(&mut current, &mut words);
            if matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '\n') {
                flush_phrase(Some(c), &mut words, &mut output);
            }
        }
    }
    flush_word(&mut current, &mut words);
    flush_phrase(None, &mut words, &mut output);
    output
}

/// Turkish spelling per sung syllable. Each vowel is one nucleus.
/// By default V-CV splits between vowels, while V-CCV keeps the first C in coda.
fn spell_chunks(token: &str, count: usize) -> Vec<String> {
    if count <= 1 { return vec![token.to_string()]; }
    let chars: Vec<char> = token.chars().collect();
    let vowel_ix: Vec<usize> = chars.iter().enumerate()
        .filter_map(|(i, &c)| vowel(lower(&c.to_string()).chars().next().unwrap_or(c)).then_some(i))
        .collect();
    if vowel_ix.len() != count {
        let mut x = vec![token.to_string()];
        x.extend(std::iter::repeat_n("~".to_string(), count.saturating_sub(1)));
        return x;
    }
    let mut cuts = vec![0];
    for pair in vowel_ix.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        // Put the last inter-vocalic consonant into the next syllable.
        let cut = if b - a <= 1 { b } else { b - 1 };
        cuts.push(cut);
    }
    cuts.push(chars.len());
    cuts.windows(2).map(|w| chars[w[0]..w[1]].iter().collect()).collect()
}

pub struct Turkish;
impl FrontEnd for Turkish {
    fn starts(&self, c: char) -> bool { is_letter(c) }
    fn continues(&self, c: char) -> bool {
        is_letter(c) || c == '-' || c == '\'' || c == '’'
    }
    fn word(&self, token: &str) -> Vec<Syllable> {
        let parts: Vec<&str> = token.split('-').filter(|s| !s.is_empty()).collect();
        let mut output = Vec::new();
        for part in parts.iter() {
            // Turkish normally admits a single consonant as an internal onset.
            let syllables = syllabify_with(&word(part), |c| c.len() == 1);
            let spells = if parts.len() > 1 {
                let mut s = vec![part.to_string()];
                s.extend(std::iter::repeat_n("~".to_string(), syllables.len().saturating_sub(1)));
                s
            } else { spell_chunks(part, syllables.len()) };
            for (i, s) in syllables.into_iter().enumerate() {
                output.push(Syllable {
                    text: spells.get(i).cloned().unwrap_or_else(|| "~".into()),
                    onset: s.onset, nucleus: s.nucleus, coda: s.coda,
                    ..Default::default()
                });
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phonetics::syllables::FrontEnd;
    #[test]
    fn turkish_case() {
        assert_eq!(lower("İSTANBUL IĞDIR"), "istanbul ığdır");
    }
    #[test]
    fn turkish_vowels() {
        let v: Vec<String> = word("a e ı i o ö u ü").into_iter().map(|s| s.ipa).collect();
        assert_eq!(v, vec!["a", "e", "ɯ", "i", "o", "ø", "u", "y"]);
    }
    #[test]
    fn turkish_singing() {
        let s = Turkish.word("görüşürüz");
        assert_eq!(s.len(), 4);
        assert!(s.iter().all(|x| !x.nucleus.ipa.is_empty()));
    }
    #[test]
    fn turkish_question() {
        assert_eq!(to_phrases("Nasılsın? Sağ ol!").len(), 2);
        assert_eq!(to_phrases("Nasılsın?")[0].intonation, Intonation::Question);
    }
}
