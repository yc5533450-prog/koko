#!/usr/bin/env python3
"""Apply the CHIMERA Turkish language front end to an upstream MIT checkout.
This changes speech and singing defaults to Turkish; prefix text with en: for English.
"""
from pathlib import Path
import sys
import shutil

BASE = Path(__file__).resolve().parent
repo = Path(sys.argv[1]).expanduser().resolve() if len(sys.argv) > 1 else (BASE / 'chimera-vocal-synth')
if not (repo / 'crates/chimera/src/phonetics/mod.rs').exists():
    sys.exit(f'CHIMERA kaynak kodu bulunamadı: {repo}\nÖnce GitHub projesini klonlayın.')

def patch(path, old, new):
    file = repo / path
    s = file.read_text(encoding='utf-8')
    if new in s:
        print('Zaten yapılmış:', path)
        return
    if old not in s:
        sys.exit(f'Güvenli yama uygulanamadı: {path} içinde beklenen kod değişmiş. Orijinal dosyalar korunuyor.')
    file.write_text(s.replace(old, new, 1), encoding='utf-8')
    print('Değiştirildi:', path)

# Preflight all expected source expressions before changing files.
modp = repo / 'crates/chimera/src/phonetics/mod.rs'
api = repo / 'crates/chimera-wasm/src/api.rs'
mod = modp.read_text(encoding='utf-8')
a = api.read_text(encoding='utf-8')
needle = [
    'pub mod russian;',
    'use chimera::phonetics::{Segment, Syllable, ipa, songs, syllables, vowel_data};',
    '"g2pPhrases" => to_json(&st.g2p.to_phrases(&arg::<String>(req, "text")?)),',
    '"g2pWord" => to_json(&st.g2p.word(&arg::<String>(req, "word")?, arg::<Option<bool>>(req, "reduce")?.unwrap_or(true))),',
    '"lyricsToSyllables" => to_json(&syllables::lyrics_to_syllables(&arg::<String>(req, "text")?, &st.g2p)),',
]
if ('pub mod turkish;' not in mod and needle[0] not in mod) or any(x not in a for x in needle[1:]):
    if 'turkish::to_phrases' not in a:
        sys.exit('Kod sürümü beklenenden farklı. Yanlış değişiklik yapmamak için yama durduruldu.')

src = repo / 'crates/chimera/src/phonetics/turkish.rs'
if src.exists():
    if src.read_bytes() != (BASE / 'turkish.rs').read_bytes():
        sys.exit('turkish.rs zaten var ve farklı. Dosyanın üzerine yazılmadı.')
else:
    shutil.copy2(BASE / 'turkish.rs', src)
patch('crates/chimera/src/phonetics/mod.rs', 'pub mod russian;', 'pub mod russian;\npub mod turkish;')
patch('crates/chimera-wasm/src/api.rs',
      'use chimera::phonetics::{Segment, Syllable, ipa, songs, syllables, vowel_data};',
      'use chimera::phonetics::{Segment, Syllable, ipa, songs, syllables, vowel_data, turkish, korean, russian};')
patch('crates/chimera-wasm/src/api.rs',
      '"g2pPhrases" => to_json(&st.g2p.to_phrases(&arg::<String>(req, "text")?)),',
      '''"g2pPhrases" => {
            let t: String = arg(req, "text")?;
            if let Some(en) = t.trim_start().strip_prefix("en:") {
                to_json(&st.g2p.to_phrases(en.trim()))
            } else {
                to_json(&turkish::to_phrases(&t))
            }
        },''')
patch('crates/chimera-wasm/src/api.rs',
      '"g2pWord" => to_json(&st.g2p.word(&arg::<String>(req, "word")?, arg::<Option<bool>>(req, "reduce")?.unwrap_or(true))),',
      '''"g2pWord" => {
            let w: String = arg(req, "word")?;
            if let Some(en) = w.trim_start().strip_prefix("en:") {
                to_json(&st.g2p.word(en.trim(), true))
            } else {
                to_json(&turkish::word(&w))
            }
        },''')
patch('crates/chimera-wasm/src/api.rs',
      '"lyricsToSyllables" => to_json(&syllables::lyrics_to_syllables(&arg::<String>(req, "text")?, &st.g2p)),',
      '''"lyricsToSyllables" => {
            let t: String = arg(req, "text")?;
            if let Some(en) = t.trim_start().strip_prefix("en:") {
                to_json(&syllables::lyrics_to_syllables(en.trim(), &st.g2p))
            } else {
                to_json(&syllables::read_lyrics(&t, &[&turkish::Turkish, &korean::Korean, &russian::Russian]))
            }
        },''')
print('\nCHIMERA Türkçe yaması eklendi. Orijinal ses motoru korunmuştur.')
print('Not: Dil varsayılan olarak Türkçedir. İngilizce metin için en: öneki kullanın.')
