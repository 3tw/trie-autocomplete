use std::str;

#[derive(Debug)]
pub struct Ascii(pub Vec<u8>);
impl IntoIterator for Ascii {
    type Item = u8;
    type IntoIter = ::std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl Clone for Ascii {
    fn clone(&self) -> Ascii {
        Ascii(self.0.clone())
    }
}
impl Ascii {
    pub fn from(value: &str) -> Ascii {
        Ascii(value.as_bytes().iter().map(|&b| b).collect::<Vec<u8>>())
    }
    pub fn push(&mut self, value: u8) {
        self.0.push(value);
    }
    pub fn to_string(&self)-> &str {
      str::from_utf8(&self.0).unwrap()
    }
}


pub const WORDS: [&str; 63] = [
  "apple",
  "banana",
  "carrot",
  "daisy",
  "eggplant",
  "fern",
  "grape",
  "honeydew",
  "iris",
  "jasmine",
  "kale",
  "lemon",
  "maple",
  "nutmeg",
  "onion",
  "pumpkin",
  "quince",
  "raspberry",
  "strawberry",
  "tomato",
  "umbrella",
  "anchor",
  "broom",
  "candle",
  "dagger",
  "eagle",
  "fan",
  "guitar",
  "harbor",
  "ink",
  "jar",
  "kite",
  "lamp",
  "monitor",
  "needle",
  "ocean",
  "paint",
  "quilt",
  "rainbow",
  "scissors",
  "tent",
  "umbrella",
  "aroma",
  "bloom",
  "chorus",
  "dream",
  "echo",
  "flower",
  "glow",
  "harmony",
  "illuminate",
  "journey",
  "kaleidoscope",
  "luminous",
  "melody",
  "nirvana",
  "oasis",
  "passion",
  "quiet",
  "radiance",
  "serenade",
  "thrive",
  "utopia"
];