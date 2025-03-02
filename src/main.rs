use core::fmt::{Display, Formatter, Result};
use owo_colors::{OwoColorize, Rgb};
use std::io;
use tokio;

#[tokio::main]
async fn main() {
    let instrument = Instrument::new(
        &InstrumentType::Guitar,
        &TuningType::Standard,
        &NotePitch::new(&NaturalNote::C, &None, 4),
        6,
        24,
    );
    print!("{}", instrument);
    let scale = Scale::new(
        &NoteName::new(&NaturalNote::C, &None),
        &ScaleDef::new_minor_pentatonic(),
    );
    print!("{}", scale.definition);
    print!("{}", scale);
    let chord = Chord::new(
        &NoteName::new(&NaturalNote::C, &None),
        &ChordDef::new_minor_eleven(),
    );
    print!("{}", chord.definition);
    print!("{}", chord);
    // let audio_engine = AudioEngine::new();
    //  audio_engine.play_audio(vec![440.0], 2.0).await;
    RunTime::start(&mut RunTime::new()).await;
}

struct Instrument {
    instrument_type: InstrumentType,
    tuning_type: TuningType,
    root_note: NotePitch,
    string_count: usize,
    fret_count: usize,
    tuning: Vec<NotePitch>,
    fretboard: Vec<Vec<NoteDisplay>>,
}

impl Display for Instrument {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let white_rgb = Rgb {
            0: 255,
            1: 255,
            2: 255,
        };
        let marked_frets: Vec<usize> = vec![1, 3, 5, 7, 9, 12, 15, 17, 19, 21, 24];
        let mut marked = false;
        for i in (0..self.string_count).rev() {
            write!(f, "{} ", (self.string_count - i).color(white_rgb))?;
            for j in 0..self.fret_count {
                marked = false;
                for mark in &marked_frets {
                    if j == *mark {
                        marked = true;
                    }
                }
                match marked {
                    false => {
                        match &self.fretboard[i][j].note_pitch.note_name.accidental {
                            None => write!(f, "{} ", self.fretboard[i][j])?,
                            Some(_accidental) => write!(f, "{}", self.fretboard[i][j])?,
                        };
                    }
                    true => {
                        match &self.fretboard[i][j].note_pitch.note_name.accidental {
                            None => write!(f, "\x1b[4m{} \x1b[0m", self.fretboard[i][j])?,
                            Some(_accidental) => {
                                write!(f, "\x1b[4m{}\x1b[0m", self.fretboard[i][j])?
                            }
                        };
                    }
                }
                write!(f, " ")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  ")?;
        for i in 0..self.fret_count {
            match i {
                0..=9 => write!(f, "{}   ", i.color(white_rgb))?,
                10.. => write!(f, "{}  ", i.color(white_rgb))?,
                _ => panic!("unexpected fret_count"),
            };
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl Instrument {
    fn new(
        instrument_type: &InstrumentType,
        tuning_type: &TuningType,
        root_note: &NotePitch,
        string_count: usize,
        fret_count: usize,
    ) -> Self {
        let mut instrument = Instrument {
            instrument_type: instrument_type.clone(),
            tuning_type: tuning_type.clone(),
            root_note: root_note.clone(),
            string_count,
            fret_count: fret_count + 1,
            tuning: Vec::new(),
            fretboard: Vec::new(),
        };
        Instrument::calculate_tuning(&mut instrument);
        Instrument::calculate_notes(&mut instrument);
        instrument
    }

    fn calculate_notes(self: &mut Self) {
        let mut notes: Vec<Vec<NoteDisplay>> = Vec::new();
        for i in 0..self.string_count {
            let mut musical_string: Vec<NoteDisplay> = Vec::new();
            for j in 0..self.fret_count {
                musical_string.push(NoteDisplay {
                    note_pitch: NotePitch::find_note(&self.tuning[i], j.try_into().unwrap()),
                    is_displayed: true,
                });
            }
            notes.push(musical_string.clone());
        }
        self.fretboard = notes;
    }

    fn calculate_tuning(self: &mut Self) {
        self.tuning.push(self.root_note.clone());
        match self.instrument_type {
            InstrumentType::Guitar => match self.tuning_type {
                TuningType::Standard => {
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 5));
                    self.tuning.push(NotePitch::find_note(&self.tuning[1], 5));
                    self.tuning.push(NotePitch::find_note(&self.tuning[2], 5));
                    self.tuning.push(NotePitch::find_note(&self.tuning[3], 4));
                    self.tuning.push(NotePitch::find_note(&self.tuning[4], 5));
                }
                TuningType::Drop => {
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 7));
                    self.tuning.push(NotePitch::find_note(&self.tuning[1], 5));
                    self.tuning.push(NotePitch::find_note(&self.tuning[2], 5));
                    self.tuning.push(NotePitch::find_note(&self.tuning[3], 4));
                    self.tuning.push(NotePitch::find_note(&self.tuning[4], 5));
                }
                TuningType::Open => {
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 7));
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 12));
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 16));
                    self.tuning.push(NotePitch::find_note(&self.tuning[3], 3));
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 24));
                }
                _ => todo!(),
            },
            InstrumentType::Bass => match self.tuning_type {
                TuningType::Standard => {
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 5));
                    self.tuning.push(NotePitch::find_note(&self.tuning[1], 5));
                    self.tuning.push(NotePitch::find_note(&self.tuning[2], 5));
                }
                TuningType::Drop => {
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 7));
                    self.tuning.push(NotePitch::find_note(&self.tuning[1], 5));
                    self.tuning.push(NotePitch::find_note(&self.tuning[2], 5));
                }
                TuningType::Open => {
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 7));
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 12));
                    self.tuning.push(NotePitch::find_note(&self.tuning[0], 16));
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn show_all(self: &mut Self) {
        for i in 0..self.string_count {
            for j in 0..self.fret_count {
                self.fretboard[i][j].is_displayed = true;
            }
        }
    }

    fn show_notes(self: &mut Self, notes: &Vec<NoteName>) {
        for i in 0..self.string_count {
            for j in 0..self.fret_count {
                self.fretboard[i][j].is_displayed = false;
            }
        }
        for note in notes {
            for i in 0..self.string_count {
                for j in 0..self.fret_count {
                    if NoteName::to_number(&note)
                        == NoteName::to_number(&self.fretboard[i][j].note_pitch.note_name)
                    {
                        self.fretboard[i][j].is_displayed = true;
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct NoteDisplay {
    note_pitch: NotePitch,
    is_displayed: bool,
}

/*
impl NoteDisplay {

}
*/

impl Display for NoteDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.is_displayed {
            true => {
                let rgb = NoteName::to_rgb(&self.note_pitch.note_name);
                write!(f, "{}", self.note_pitch.get_name().color(rgb))?;
                Ok(())
            }
            false => {
                match &self.note_pitch.note_name.accidental {
                    Some(_accidental) => write!(f, "   ")?,
                    None => write!(f, "  ")?,
                }

                Ok(())
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct NotePitch {
    note_name: NoteName,
    octave: i8,
}

impl Display for NotePitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let rgb = NoteName::to_rgb(&self.note_name);
        match &self.note_name.accidental {
            Some(_accidental) => write!(f, "{}", self.get_name().color(rgb)),
            None => write!(f, "\u{2002}{}", self.get_name().color(rgb)),
        }
    }
}

impl NotePitch {
    fn new(natural_note: &NaturalNote, accidental: &Option<Accidental>, octave: i8) -> Self {
        NotePitch {
            note_name: NoteName {
                natural_note: natural_note.clone(),
                accidental: accidental.clone(),
            },
            octave,
        }
    }

    fn get_name(self: &Self) -> String {
        let note_and_accidental: String = NoteName::get_name(&self.note_name);
        let name: String = format!("{}{}", note_and_accidental, self.octave);
        name
    }

    fn from_number(note_number: i8, octave: i8) -> NotePitch {
        match note_number {
            0 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::C,
                    accidental: None,
                },
                octave,
            },
            1 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::C,
                    accidental: Some(Accidental::Sharp),
                },
                octave,
            },
            2 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::D,
                    accidental: None,
                },
                octave,
            },
            3 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::D,
                    accidental: Some(Accidental::Sharp),
                },
                octave,
            },
            4 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::E,
                    accidental: None,
                },
                octave,
            },
            5 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::F,
                    accidental: None,
                },
                octave,
            },
            6 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::F,
                    accidental: Some(Accidental::Sharp),
                },
                octave,
            },
            7 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::G,
                    accidental: None,
                },
                octave,
            },
            8 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::G,
                    accidental: Some(Accidental::Sharp),
                },
                octave,
            },
            9 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::A,
                    accidental: None,
                },
                octave,
            },
            10 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::A,
                    accidental: Some(Accidental::Sharp),
                },
                octave,
            },
            11 => NotePitch {
                note_name: NoteName {
                    natural_note: NaturalNote::B,
                    accidental: None,
                },
                octave,
            },
            _ => panic!("unexpected note number"),
        }
    }

    fn to_number(note_pitch: &NotePitch) -> (i8, i8) {
        let mut octave: i8 = note_pitch.octave;
        let mut number: i8 = match note_pitch.note_name.natural_note {
            NaturalNote::C => {
                0 + match note_pitch.note_name.accidental {
                    Some(Accidental::Flat) => panic!("unexpected accidental"),
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::D => {
                2 + match note_pitch.note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::E => {
                4 + match note_pitch.note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => panic!("unexpected accidental"),
                    None => 0,
                }
            }
            NaturalNote::F => {
                5 + match note_pitch.note_name.accidental {
                    Some(Accidental::Flat) => panic!("unexpected accidental"),
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::G => {
                7 + match note_pitch.note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::A => {
                9 + match note_pitch.note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::B => {
                11 + match note_pitch.note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => panic!("unexpected accidental"),
                    None => 0,
                }
            }
        };
        while number > 11 {
            number = number - 11;
            octave = octave + 1;
        }
        (number, octave)
    }

    fn find_note(open_note: &NotePitch, distance: i8) -> NotePitch {
        let (x, y) = match distance {
            i if i > 0 => NotePitch::add(&open_note, distance),
            i if i < 0 => NotePitch::minus(&open_note, distance),
            0 => return open_note.clone(),
            _ => panic!("unexpected distance"),
        };
        NotePitch::from_number(x, y)
    }

    fn up_step(start_note: &NotePitch, step: &Step) -> NotePitch {
        let to_add = Step::to_number(step);
        let (number, octave) = NotePitch::add(start_note, to_add);
        NotePitch::from_number(number, octave)
    }

    fn down_step(start_note: &NotePitch, step: &Step) -> NotePitch {
        let to_subtract = Step::to_number(step);
        let (number, octave) = NotePitch::minus(start_note, to_subtract);
        NotePitch::from_number(number, octave)
    }

    fn add(start_note: &NotePitch, to_add: i8) -> (i8, i8) {
        let (note_number, octave) = NotePitch::to_number(&start_note);
        let mut octave = octave;
        let mut number = note_number + to_add;
        while number >= 12 {
            number = number - 12;
            octave = octave + 1;
        }
        (number, octave)
    }

    fn minus(start_note: &NotePitch, to_subtract: i8) -> (i8, i8) {
        let (note_number, octave) = NotePitch::to_number(&start_note);
        let mut octave = octave;
        let mut number: i8 = note_number - to_subtract;
        while number < 0 {
            number = number + 12;
            octave = octave - 1;
        }
        (number, octave)
    }
}

#[derive(PartialEq, Clone, Debug)]
struct NoteName {
    natural_note: NaturalNote,
    accidental: Option<Accidental>,
}

impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let rgb = NoteName::to_rgb(self); // Assuming this function returns an 1	 struct
        write!(f, "{}", self.get_name().color(rgb))?;
        Ok(())
    }
}

impl NoteName {
    fn new(natural_note: &NaturalNote, accidental: &Option<Accidental>) -> Self {
        NoteName {
            natural_note: natural_note.clone(),
            accidental: accidental.clone(),
        }
    }

    fn get_name(self: &Self) -> String {
        let natural_note = match self.natural_note {
            NaturalNote::A => "A".to_string(),
            NaturalNote::B => "B".to_string(),
            NaturalNote::C => "C".to_string(),
            NaturalNote::D => "D".to_string(),
            NaturalNote::E => "E".to_string(),
            NaturalNote::F => "F".to_string(),
            NaturalNote::G => "G".to_string(),
        };
        let accidental = match self.accidental {
            Some(Accidental::Sharp) => "♯".to_string(),
            Some(Accidental::Flat) => "♭".to_string(),
            None => "".to_string(),
        };
        let name = natural_note + &accidental;
        name
    }

    fn from_number(note_number: i8) -> NoteName {
        match note_number {
            0 => NoteName {
                natural_note: NaturalNote::C,
                accidental: None,
            },
            1 => NoteName {
                natural_note: NaturalNote::C,
                accidental: Some(Accidental::Sharp),
            },
            2 => NoteName {
                natural_note: NaturalNote::D,
                accidental: None,
            },
            3 => NoteName {
                natural_note: NaturalNote::D,
                accidental: Some(Accidental::Sharp),
            },
            4 => NoteName {
                natural_note: NaturalNote::E,
                accidental: None,
            },
            5 => NoteName {
                natural_note: NaturalNote::F,
                accidental: None,
            },
            6 => NoteName {
                natural_note: NaturalNote::F,
                accidental: Some(Accidental::Sharp),
            },
            7 => NoteName {
                natural_note: NaturalNote::G,
                accidental: None,
            },
            8 => NoteName {
                natural_note: NaturalNote::G,
                accidental: Some(Accidental::Sharp),
            },
            9 => NoteName {
                natural_note: NaturalNote::A,
                accidental: None,
            },
            10 => NoteName {
                natural_note: NaturalNote::A,
                accidental: Some(Accidental::Sharp),
            },
            11 => NoteName {
                natural_note: NaturalNote::B,
                accidental: None,
            },
            _ => panic!("unexpected note number"),
        }
    }

    fn to_number(note_name: &NoteName) -> i8 {
        let mut number: i8 = match note_name.natural_note {
            NaturalNote::C => {
                0 + match note_name.accidental {
                    Some(Accidental::Flat) => panic!("unexpected accidental"),
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::D => {
                2 + match note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::E => {
                4 + match note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => panic!("unexpected accidental"),
                    None => 0,
                }
            }
            NaturalNote::F => {
                5 + match note_name.accidental {
                    Some(Accidental::Flat) => panic!("unexpected accidental"),
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::G => {
                7 + match note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::A => {
                9 + match note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::B => {
                11 + match note_name.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => panic!("unexpected accidental"),
                    None => 0,
                }
            }
        };
        if number > 11 {
            number = number - 11;
        }
        number
    }

    fn up_step(start_note: &NoteName, step: &Step) -> NoteName {
        let to_add = Step::to_number(step);
        let number = NoteName::add(start_note, to_add);
        NoteName::from_number(number)
    }

    fn down_step(start_note: &NoteName, step: &Step) -> NoteName {
        let to_subtract = Step::to_number(step);
        let number = NoteName::minus(start_note, to_subtract);
        NoteName::from_number(number)
    }

    fn add(start_note: &NoteName, to_add: i8) -> i8 {
        let note_number = NoteName::to_number(&start_note);
        let mut number = note_number + to_add;
        while number >= 12 {
            number = number - 12;
        }
        number
    }

    fn minus(start_note: &NoteName, to_subtract: i8) -> i8 {
        let note_number = NoteName::to_number(&start_note);
        let mut number: i8 = note_number - to_subtract;
        while number < 0 {
            number = number + 12;
        }
        number
    }

    fn to_rgb(note_name: &NoteName) -> Rgb {
        match note_name.natural_note {
            NaturalNote::C => match note_name.accidental {
                Some(Accidental::Flat) => panic!("unexpected accidental"),
                Some(Accidental::Sharp) => Rgb {
                    0: 191,
                    1: 64,
                    2: 191,
                },
                None => Rgb { 0: 191, 1: 0, 2: 0 },
            },
            NaturalNote::D => match note_name.accidental {
                Some(Accidental::Flat) => Rgb {
                    0: 191,
                    1: 64,
                    2: 191,
                },
                Some(Accidental::Sharp) => Rgb {
                    0: 198,
                    1: 255,
                    2: 0,
                },
                None => Rgb {
                    0: 255,
                    1: 191,
                    2: 64,
                },
            },
            NaturalNote::E => match note_name.accidental {
                Some(Accidental::Flat) => Rgb {
                    0: 198,
                    1: 255,
                    2: 0,
                },
                Some(Accidental::Sharp) => panic!("unexpected accidental"),
                None => Rgb {
                    0: 244,
                    1: 67,
                    2: 54,
                },
            },
            NaturalNote::F => match note_name.accidental {
                Some(Accidental::Flat) => panic!("unexpected accidental"),
                Some(Accidental::Sharp) => Rgb {
                    0: 255,
                    1: 0,
                    2: 191,
                },
                None => Rgb {
                    0: 255,
                    1: 0,
                    2: 255,
                },
            },
            NaturalNote::G => match note_name.accidental {
                Some(Accidental::Flat) => Rgb {
                    0: 255,
                    1: 0,
                    2: 191,
                },
                Some(Accidental::Sharp) => Rgb { 0: 0, 1: 191, 2: 0 },
                None => Rgb { 0: 0, 1: 255, 2: 0 },
            },
            NaturalNote::A => match note_name.accidental {
                Some(Accidental::Flat) => Rgb { 0: 0, 1: 191, 2: 0 },
                Some(Accidental::Sharp) => Rgb {
                    0: 64,
                    1: 191,
                    2: 191,
                },
                None => Rgb {
                    0: 165,
                    1: 255,
                    2: 235,
                },
            },
            NaturalNote::B => match note_name.accidental {
                Some(Accidental::Flat) => Rgb {
                    0: 64,
                    1: 191,
                    2: 191,
                },
                Some(Accidental::Sharp) => panic!("unexpected accidental"),
                None => Rgb {
                    0: 64,
                    1: 31,
                    2: 255,
                },
            },
        }
    }

    fn from_string(input: String) -> NoteName {
        let input: String = input.to_uppercase();
        match input.as_str() {
            "A" => NoteName {
                natural_note: NaturalNote::A,
                accidental: None,
            },
            "A#" => NoteName {
                natural_note: NaturalNote::A,
                accidental: Some(Accidental::Sharp),
            },
            "B" => NoteName {
                natural_note: NaturalNote::B,
                accidental: None,
            },
            "C" => NoteName {
                natural_note: NaturalNote::C,
                accidental: None,
            },
            "C#" => NoteName {
                natural_note: NaturalNote::C,
                accidental: Some(Accidental::Sharp),
            },
            "D" => NoteName {
                natural_note: NaturalNote::D,
                accidental: None,
            },
            "D#" => NoteName {
                natural_note: NaturalNote::D,
                accidental: Some(Accidental::Sharp),
            },
            "E" => NoteName {
                natural_note: NaturalNote::E,
                accidental: None,
            },
            "F" => NoteName {
                natural_note: NaturalNote::F,
                accidental: None,
            },
            "F#" => NoteName {
                natural_note: NaturalNote::F,
                accidental: Some(Accidental::Sharp),
            },
            "G" => NoteName {
                natural_note: NaturalNote::G,
                accidental: None,
            },
            "G#" => NoteName {
                natural_note: NaturalNote::G,
                accidental: Some(Accidental::Sharp),
            },
            _ => {
                println!("Enter a new key (e.g., C, D#, F#):");
                let mut reinput = String::new();
                io::stdin()
                    .read_line(&mut reinput)
                    .expect("Failed to read input");
                let key = NoteName::from_string(reinput.trim().to_string());
                return key;
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum NaturalNote {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(PartialEq, Clone, Debug)]
enum TuningType {
    Open,
    Drop,
    Standard,
    Custom,
}

impl TuningType {
    fn from_string(input: String) -> Self {
        let input_uppercase = input.to_uppercase();
        /*let mut count = 0;
        let mut indices = Vec::new();
        for (index, ch) in input.char_indices() {
            if ch == ' ' {
                count += 1;
                indices.push(index);
            }
        }
        if count > 0 {

        }*/
        match input_uppercase.as_str() {
            "OPEN" => TuningType::Open,
            "DROP" => TuningType::Drop,
            "STANDARD" => TuningType::Standard,
            "CUSTOM" => TuningType::Custom,
            _ => {
                println!("Enter a tuning (e.g., Standard, Drop D, Open G):");
                let mut reinput = String::new();
                io::stdin()
                    .read_line(&mut reinput)
                    .expect("Failed to read input");
                let tuning = TuningType::from_string(reinput.trim().to_string());
                return tuning;
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum InstrumentType {
    Guitar,
    Bass,
    Mandolin,
    Banjo,
    Ukelelle,
}

#[derive(PartialEq, Clone, Debug)]
struct StepValue {
    value: i8,
}

#[derive(PartialEq, Clone, Debug)]
enum Step {
    Whole(StepValue),
    Half(StepValue),
    OneAndAHalf(StepValue),
}

impl Step {
    fn new_whole() -> Self {
        Step::Whole(StepValue { value: 2 })
    }

    fn new_half() -> Self {
        Step::Half(StepValue { value: 1 })
    }

    fn new_one_and_a_half() -> Self {
        Step::OneAndAHalf(StepValue { value: 3 })
    }

    fn to_number(self: &Self) -> i8 {
        match self {
            Step::Whole(step_value) => step_value.value,
            Step::Half(step_value) => step_value.value,
            Step::OneAndAHalf(step_value) => step_value.value,
        }
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let display_character = match self {
            Step::Whole(_step_value) => "W".to_string(),
            Step::Half(_step_value) => "H".to_string(),
            Step::OneAndAHalf(_step_value) => "3/2".to_string(),
        };
        write!(f, "{}", display_character)?;
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
struct ScaleDef {
    name: String,
    steps: Vec<Step>,
}

impl ScaleDef {
    fn new_major() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        ScaleDef {
            name: "Major".to_string(),
            steps,
        }
    }

    fn new_ionian() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        ScaleDef {
            name: "Ionian".to_string(),
            steps,
        }
    }

    fn new_dorian() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Dorian".to_string(),
            steps,
        }
    }

    fn new_phrygian() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Phrygian".to_string(),
            steps,
        }
    }

    fn new_lydian() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        ScaleDef {
            name: "Lydian".to_string(),
            steps,
        }
    }

    fn new_mixolydian() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Mixolydian".to_string(),
            steps,
        }
    }

    fn new_aeolian() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Aeolian".to_string(),
            steps,
        }
    }

    fn new_locrian() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Locrian".to_string(),
            steps,
        }
    }

    fn new_natural_minor() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Natural Minor".to_string(),
            steps,
        }
    }

    fn new_harmonic_minor() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_half());
        ScaleDef {
            name: "Harmonic Minor".to_string(),
            steps,
        }
    }

    fn new_melodic_minor_ascending() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        ScaleDef {
            name: "Melodic Minor Ascending".to_string(),
            steps,
        }
    }

    fn new_melodic_minor_descending() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Melodic Minor Descending".to_string(),
            steps,
        }
    }

    fn new_chromatic() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        ScaleDef {
            name: "Chromatic".to_string(),
            steps,
        }
    }

    fn new_whole_tone() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Whole Tone".to_string(),
            steps,
        }
    }

    fn new_major_pentatonic() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_one_and_a_half());
        ScaleDef {
            name: "Major Pentatonic".to_string(),
            steps,
        }
    }

    fn new_minor_pentatonic() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Minor Pentatonic".to_string(),
            steps,
        }
    }

    fn new_blues() -> Self {
        let mut steps: Vec<Step> = Vec::new();
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        ScaleDef {
            name: "Blues".to_string(),
            steps,
        }
    }
}

impl Display for ScaleDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let white_rgb = Rgb {
            0: 255,
            1: 255,
            2: 255,
        };
        let scale_str: &str = "scale: ";
        write!(
            f,
            "{} {}",
            self.name.color(white_rgb),
            scale_str.color(white_rgb)
        )?;
        for step in &self.steps {
            write!(f, "{} ", step.color(white_rgb))?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Scale {
    definition: ScaleDef,
    name: String,
    notes: Vec<NoteName>,
}

impl Scale {
    fn new(root_note: &NoteName, definition: &ScaleDef) -> Self {
        let mut notes: Vec<NoteName> = Vec::new();
        notes.push(root_note.clone());
        let mut count: usize = 0;
        for step in &definition.steps {
            notes.push(NoteName::up_step(&notes[count], &step));
            count = count + 1;
        }
        Scale {
            definition: definition.clone(),
            name: format!("{} {}", &notes[0].get_name(), definition.name),
            notes,
        }
    }
    fn from_string(key: &NoteName, input: String) -> Self {
        let input: String = input.to_uppercase();
        match input.as_str() {
            "MAJOR" => Scale::new(&key, &ScaleDef::new_major()),
            "IONIAN" => Scale::new(&key, &ScaleDef::new_ionian()),
            "DORIAN" => Scale::new(&key, &ScaleDef::new_dorian()),
            "PHRYGIAN" => Scale::new(&key, &ScaleDef::new_phrygian()),
            "LYDIAN" => Scale::new(&key, &ScaleDef::new_lydian()),
            "MIXOLYDIAN" => Scale::new(&key, &ScaleDef::new_mixolydian()),
            "AEOLIAN" => Scale::new(&key, &ScaleDef::new_aeolian()),
            "LOCRIAN" => Scale::new(&key, &ScaleDef::new_locrian()),
            "NATURAL MINOR" => Scale::new(&key, &ScaleDef::new_natural_minor()),
            "HARMONIC MINOR" => Scale::new(&key, &ScaleDef::new_harmonic_minor()),
            "MELODIC MINOR ASCENDING" => Scale::new(&key, &ScaleDef::new_melodic_minor_ascending()),
            "MELODIC MINOR DESCENDING" => {
                Scale::new(&key, &ScaleDef::new_melodic_minor_descending())
            }
            "CHROMATIC" => Scale::new(&key, &ScaleDef::new_chromatic()),
            "WHOLE TONE" => Scale::new(&key, &ScaleDef::new_whole_tone()),
            "MAJOR PENTATONIC" => Scale::new(&key, &ScaleDef::new_major_pentatonic()),
            "MINOR PENTATONIC" => Scale::new(&key, &ScaleDef::new_minor_pentatonic()),
            "BLUES" => Scale::new(&key, &ScaleDef::new_blues()),
            _ => {
                println!("Enter a scale (e.g., Major, Minor, Dorian):");
                let mut reinput = String::new();
                io::stdin()
                    .read_line(&mut reinput)
                    .expect("Failed to read input");
                let scale = Scale::from_string(&key, reinput.trim().to_string());
                return scale;
            }
        }
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let white_rgb = Rgb {
            0: 255,
            1: 255,
            2: 255,
        };
        let scale_str: &str = "scale: ";
        write!(
            f,
            "{} {}",
            self.name.color(white_rgb),
            scale_str.color(white_rgb)
        )?;
        for note in &self.notes {
            write!(f, "{} ", note)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Accidental {
    Sharp,
    Flat,
}

impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Accidental::Sharp => write!(f, "♯")?,
            Accidental::Flat => write!(f, "♭")?,
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Interval {
    accidental: Option<Accidental>,
    interval: i8,
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.accidental {
            Some(Accidental::Sharp) => write!(f, "♯{}", self.interval)?,
            Some(Accidental::Flat) => write!(f, "♭{}", self.interval)?,
            None => write!(f, "{}", self.interval)?,
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
struct ChordDef {
    name: String,
    naming_convention: String,
    intervals: Vec<Interval>,
}

impl Display for ChordDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let white_rgb = Rgb {
            0: 255,
            1: 255,
            2: 255,
        };
        let chord_str: &str = "chord: ";
        write!(
            f,
            "{} {}",
            self.name.color(white_rgb),
            chord_str.color(white_rgb)
        )?;
        for interval in &self.intervals {
            write!(f, "{} ", interval.color(white_rgb))?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl ChordDef {
    fn new_major() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        ChordDef {
            name: "Major".to_string(),
            naming_convention: "".to_string(),
            intervals,
        }
    }

    fn new_minor() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        ChordDef {
            name: "Minor".to_string(),
            naming_convention: "m".to_string(),
            intervals,
        }
    }

    fn new_diminished() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 5,
        });
        ChordDef {
            name: "Diminished".to_string(),
            naming_convention: "dim".to_string(),
            intervals,
        }
    }

    fn new_augmented() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Sharp),
            interval: 5,
        });
        ChordDef {
            name: "Augmented".to_string(),
            naming_convention: "aug".to_string(),
            intervals,
        }
    }

    fn new_suspended_second() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 2,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        ChordDef {
            name: "Suspended 2".to_string(),
            naming_convention: "sus2".to_string(),
            intervals,
        }
    }

    fn new_suspended_four() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 4,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        ChordDef {
            name: "Suspended 4".to_string(),
            naming_convention: "sus4".to_string(),
            intervals,
        }
    }

    fn new_power() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        ChordDef {
            name: "Power".to_string(),
            naming_convention: "5".to_string(),
            intervals,
        }
    }

    fn new_major_seven() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 7,
        });
        ChordDef {
            name: "Major 7".to_string(),
            naming_convention: "maj7".to_string(),
            intervals,
        }
    }

    fn new_minor_seven() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        ChordDef {
            name: "Minor 7".to_string(),
            naming_convention: "m7".to_string(),
            intervals,
        }
    }

    fn new_dominant_seven() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        ChordDef {
            name: "Dominant 7".to_string(),
            naming_convention: "7".to_string(),
            intervals,
        }
    }

    fn new_minor_major_seven() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 7,
        });
        ChordDef {
            name: "Minor Major 7".to_string(),
            naming_convention: "m(Maj7)".to_string(),
            intervals,
        }
    }

    fn new_six() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 6,
        });
        ChordDef {
            name: "6".to_string(),
            naming_convention: "6".to_string(),
            intervals,
        }
    }

    fn new_minor_six() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 6,
        });
        ChordDef {
            name: "Minor 6".to_string(),
            naming_convention: "m6".to_string(),
            intervals,
        }
    }
    fn new_nine() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 9,
        });
        ChordDef {
            name: "9".to_string(),
            naming_convention: "9".to_string(),
            intervals,
        }
    }

    fn new_minor_nine() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 9,
        });
        ChordDef {
            name: "Minor 9".to_string(),
            naming_convention: "m9".to_string(),
            intervals,
        }
    }

    fn new_add_nine() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 9,
        });
        ChordDef {
            name: "Add 9".to_string(),
            naming_convention: "add9".to_string(),
            intervals,
        }
    }

    fn new_seven_suspended_four() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 4,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        ChordDef {
            name: "7 Suspended 4".to_string(),
            naming_convention: "7sus4".to_string(),
            intervals,
        }
    }

    fn new_dimished_seven() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 5,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 6,
        });
        ChordDef {
            name: "Diminished 7".to_string(),
            naming_convention: "dim7".to_string(),
            intervals,
        }
    }

    fn new_half_diminished() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        ChordDef {
            name: "Half Diminished".to_string(),
            naming_convention: "7♭5".to_string(),
            intervals,
        }
    }

    fn new_plus_seven() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Sharp),
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        ChordDef {
            name: "Plus 7".to_string(),
            naming_convention: "+7".to_string(),
            intervals,
        }
    }

    fn new_minor_eleven() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 9,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 11,
        });
        ChordDef {
            name: "Minor 11".to_string(),
            naming_convention: "m11".to_string(),
            intervals,
        }
    }

    fn new_augmented_major_seven() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Sharp),
            interval: 5,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 7,
        });
        ChordDef {
            name: "Augmented Major 7".to_string(),
            naming_convention: "Maj7♯5".to_string(),
            intervals,
        }
    }

    fn new_dominant_seven_flat_nine() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 9,
        });
        ChordDef {
            name: "Dominant 7 Flat 9".to_string(),
            naming_convention: "7♭9".to_string(),
            intervals,
        }
    }

    fn new_altered_dominant_seven() -> Self {
        let mut intervals: Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental: None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Sharp),
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Flat),
            interval: 7,
        });
        intervals.push(Interval {
            accidental: Some(Accidental::Sharp),
            interval: 9,
        });
        ChordDef {
            name: "Altered Dominant 7".to_string(),
            naming_convention: "7♯5♯9".to_string(),
            intervals,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Chord {
    definition: ChordDef,
    notes: Vec<NoteName>,
    name: String,
    short_name: String,
}

impl Display for Chord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let white_rgb = Rgb {
            0: 255,
            1: 255,
            2: 255,
        };
        let open_bracket: &str = "(";
        let closed_bracket: &str = ")";
        let chord_str: &str = "chord: ";
        write!(
            f,
            "{} {}{}{} {}",
            self.name.color(white_rgb),
            open_bracket.color(white_rgb),
            self.short_name.color(white_rgb),
            closed_bracket.color(white_rgb),
            chord_str.color(white_rgb)
        )?;
        for note in &self.notes {
            write!(f, "{} ", note)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl Chord {
    fn new(root_note: &NoteName, definition: &ChordDef) -> Self {
        let scale = Scale::new(root_note, &ScaleDef::new_major());
        let mut notes: Vec<NoteName> = Vec::new();
        let scale_length = scale.notes.len();

        for interval in &definition.intervals {
            let interval_value = interval.interval as usize;

            let i: usize = if interval_value > scale_length {
                interval_value % scale_length
            } else {
                interval_value - 1
            };

            let base_note = &scale.notes[i];
            let note = match interval.accidental {
                None => base_note.clone(),
                Some(Accidental::Flat) => NoteName::down_step(base_note, &Step::new_half()),
                Some(Accidental::Sharp) => NoteName::up_step(base_note, &Step::new_half()),
            };

            notes.push(note);
        }

        Chord {
            definition: definition.clone(),
            notes,
            name: format!("{} {}", NoteName::get_name(root_note), definition.name),
            short_name: format!(
                "{}{}",
                NoteName::get_name(root_note),
                definition.naming_convention
            ),
        }
    }

    fn from_string(key: &NoteName, input: String) -> Self {
        let input_uppercase: String = input.to_uppercase();
        match input_uppercase.as_str() {
            "MAJOR" => Chord::new(&key, &ChordDef::new_major()),
            "MINOR" => Chord::new(&key, &ChordDef::new_minor()),
            "DIMINISHED" => Chord::new(&key, &ChordDef::new_diminished()),
            "AUGMENTED" => Chord::new(&key, &ChordDef::new_augmented()),
            "SUSPENDED TWO" => Chord::new(&key, &ChordDef::new_suspended_second()),
            "SUSPENDED FOUR" => Chord::new(&key, &ChordDef::new_suspended_four()),
            "POWER" => Chord::new(&key, &ChordDef::new_power()),
            "MAJOR SEVEN" => Chord::new(&key, &ChordDef::new_major_seven()),
            "MINOR SEVEN" => Chord::new(&key, &ChordDef::new_minor_seven()),
            "DOMINANT SEVEN" => Chord::new(&key, &ChordDef::new_dominant_seven()),
            "MINOR MAJOR SEVEN" => Chord::new(&key, &ChordDef::new_minor_major_seven()),
            "SIX" => Chord::new(&key, &ChordDef::new_six()),
            "MINOR SIX" => Chord::new(&key, &ChordDef::new_minor_six()),
            "NINE" => Chord::new(&key, &ChordDef::new_nine()),
            "MINOR NINE" => Chord::new(&key, &ChordDef::new_minor_nine()),
            "ADD NINE" => Chord::new(&key, &ChordDef::new_add_nine()),
            "SEVEN SUSPENDED FOUR" => Chord::new(&key, &ChordDef::new_seven_suspended_four()),
            "DIMINISHED SEVEN" => Chord::new(&key, &ChordDef::new_dimished_seven()),
            "HALF DIMINISHED" => Chord::new(&key, &ChordDef::new_half_diminished()),
            "PLUS SEVEN" => Chord::new(&key, &ChordDef::new_plus_seven()),
            "MINOR ELEVEN" => Chord::new(&key, &ChordDef::new_minor_eleven()),
            "AUGMENTED MAJOR SEVEN" => Chord::new(&key, &ChordDef::new_augmented_major_seven()),
            "DOMINANT SEVEN FLAT NINE" => {
                Chord::new(&key, &ChordDef::new_dominant_seven_flat_nine())
            }
            "ALTERED DOMINANT SEVEN" => Chord::new(&key, &ChordDef::new_altered_dominant_seven()),
            _ => {
                println!("Enter a chord (e.g., Cmaj7, Dm, G7):");
                let mut reinput = String::new();
                io::stdin()
                    .read_line(&mut reinput)
                    .expect("Failed to read input");
                let chord = Chord::from_string(&key, reinput.trim().to_string());
                return chord;
            }
        }
    }
}

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Device, OutputCallbackInfo, SampleRate, StreamConfig};
use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Duration;

struct AudioEngine {
    device: Arc<Device>,
    config: StreamConfig,
}

impl AudioEngine {
    fn new() -> Self {
        let host = cpal::default_host();
        let device = Arc::new(
            host.default_output_device()
                .expect("No output device available"),
        );
        let config = StreamConfig {
            channels: 2,
            sample_rate: SampleRate(44100),
            buffer_size: BufferSize::Default,
        };
        AudioEngine { device, config }
    }

    pub async fn play_audio(&self, frequencies: Vec<f32>, duration_secs: f32) {
        let device = self.device.clone();
        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let sample_rate = config.sample_rate.0 as f32;
            let channels = config.channels as usize;
            let mut phase = 0.0;
            let phase_increment: Vec<f32> = frequencies
                .iter()
                .map(|&freq| (2.0 * PI * freq) / sample_rate)
                .collect();

            let stream = device
                .build_output_stream(
                    &config,
                    move |data: &mut [f32], _: &OutputCallbackInfo| {
                        for sample in data.iter_mut() {
                            let mut value = 0.0;
                            for &inc in &phase_increment {
                                value += (phase + inc).sin();
                            }
                            *sample = value / frequencies.len() as f32;
                            phase += phase_increment[0];
                            if phase > 2.0 * PI {
                                phase -= 2.0 * PI;
                            }
                        }
                    },
                    move |err| {
                        eprintln!("An error occurred on the audio stream: {:?}", err);
                    },
                    None,
                )
                .expect("Failed to build output stream");

            stream.play().expect("Failed to play the stream");

            std::thread::sleep(Duration::from_secs_f32(duration_secs));
        })
        .await
        .expect("Audio task failed");
    }
}

use chrono::{NaiveDateTime, Utc};

struct Attempt<T> {
    correct_answer: T,
    attempt_answer: T,
    correct: bool,
    attempt_number: u64,
    time_since_last_attempt: Option<u64>,
    elapsed_time: u64,
    attempt_time: NaiveDateTime,
    game_number: u64,
}

impl<T: PartialEq> Attempt<T> {
    fn new(
        correct_answer: T,
        attempt_answer: T,
        attempt_number: u64,
        time_since_last_attempt: Option<Duration>,
        elapsed_time: Duration,
        game_number: u64,
    ) -> Self {
        let correct = correct_answer == attempt_answer;
        let attempt_time = Utc::now().naive_utc();
        match time_since_last_attempt {
            Some(time_since) => Attempt {
                correct_answer,
                attempt_answer,
                correct,
                attempt_number,
                time_since_last_attempt: Some(time_since.as_secs()),
                elapsed_time: elapsed_time.as_secs(),
                attempt_time,
                game_number,
            },
            None => Attempt {
                correct_answer,
                attempt_answer,
                correct,
                attempt_number,
                time_since_last_attempt: None,
                elapsed_time: elapsed_time.as_secs(),
                attempt_time,
                game_number,
            },
        }
    }

    fn deserialize_time(&self) -> (Option<Duration>, Duration, NaiveDateTime) {
        match self.time_since_last_attempt {
            Some(time_since) => (
                Some(Duration::from_secs(time_since)),
                Duration::from_secs(self.elapsed_time),
                self.attempt_time,
            ),
            None => (
                None,
                Duration::from_secs(self.elapsed_time),
                self.attempt_time,
            ),
        }
    }
}

struct Game<T> {
    game_number: u64,
    possible_notes: Vec<T>,
    attempted_notes: Vec<T>,
    attempts: Vec<Attempt<T>>,
    start_time: NaiveDateTime,
    end_time: Option<NaiveDateTime>,
}

impl<T> Game<T> {
    fn new(possible_notes: Vec<T>, game_number: u64) -> Self {
        Game {
            game_number,
            possible_notes,
            attempted_notes: Vec::new(),
            attempts: Vec::new(),
            start_time: Utc::now().naive_utc(),
            end_time: None,
        }
    }
}

struct EarTraining<T> {
    audio_engine: AudioEngine,
    game: Game<T>,
}

impl<T> EarTraining<T> {
    fn new_notepitch_from_instrument(instrument: &Instrument) -> EarTraining<NotePitch> {
        let mut possible_notes: Vec<NotePitch> = Vec::new();
        for string in &instrument.fretboard {
            for note in string {
                possible_notes.push(note.note_pitch.clone());
            }
        }
        EarTraining {
            audio_engine: AudioEngine::new(),
            game: Game::new(possible_notes, 1),
        }
    }
}

struct RunTime {
    instrument: Instrument,
    key: NoteName,
    scale_current: Scale,
    chord_current: Chord,
    audio_engine: AudioEngine,
    display_notes: Vec<NoteName>,
}

impl RunTime {
    fn new() -> Self {
        let instrument = Instrument::new(
            &InstrumentType::Guitar,
            &TuningType::Standard,
            &NotePitch::new(&NaturalNote::C, &None, 4),
            6,
            24,
        );
        let key: NoteName = NoteName::new(&NaturalNote::C, &None);
        let scale_current: Scale = Scale::new(&key, &ScaleDef::new_major());
        let chord_current: Chord = Chord::new(
            &NoteName::new(&NaturalNote::C, &None),
            &ChordDef::new_minor_seven(),
        );
        let audio_engine: AudioEngine = AudioEngine::new();
        RunTime {
            instrument,
            key,
            scale_current,
            chord_current,
            audio_engine,
            display_notes: Vec::new(),
        }
    }

    pub async fn start(&mut self) {
        loop {
            println!("{}", self.instrument);
            println!("\nMenu:");
            println!("1 - Choose Key");
            println!("2 - Choose Chord");
            println!("3 - Choose Scale");
            println!("8 - Display Full Instrument");
            println!("9 - Change Instrument Tuning");
            println!("0 - Exit");
            println!("Enter your choice:");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim() {
                "1" => self.choose_key().await,
                "2" => self.choose_chord().await,
                "3" => self.choose_scale().await,
                "8" => self.display_instrument().await,
                "9" => self.change_tuning().await,
                "0" => {
                    println!("Exiting...");
                    break;
                }
                _ => println!("Invalid choice, please try again."),
            }
        }
    }

    async fn choose_key(&mut self) {
        println!("Enter a new key (e.g., C, D#, F#):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let key = NoteName::from_string(input.trim().to_string());
        self.key = key;
        println!("Key changed to {}", self.key);
    }

    async fn choose_chord(&mut self) {
        println!("Enter a chord (e.g., Cmaj7, Dm, G7):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let chord = Chord::from_string(&self.key, input.trim().to_string());
        self.chord_current = chord;
        self.display_notes = self.chord_current.notes.clone();
        Instrument::show_notes(&mut self.instrument, &self.display_notes);
        println!(
            "Chord changed to {} {} definition: {}",
            self.chord_current, self.chord_current.name, self.chord_current.definition
        );
    }

    async fn choose_scale(&mut self) {
        println!("Enter a scale (e.g., Major, Minor, Dorian):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let scale = Scale::from_string(&self.key, input.trim().to_string());
        self.scale_current = scale;
        self.display_notes = self.scale_current.notes.clone();
        Instrument::show_notes(&mut self.instrument, &self.display_notes);
        println!(
            "Scale changed to {} {} definition: {}",
            self.scale_current, self.scale_current.name, self.scale_current.definition
        );
    }

    async fn change_tuning(&mut self) {
        println!("Enter a tuning (e.g., Standard, Drop D, Open G):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let tuning = TuningType::from_string(input.trim().to_string());
        println!("Tuning changed to {:?}", tuning);
    }

    async fn display_instrument(&mut self) {
        println!("\nInstrument Details:");
        Instrument::show_all(&mut self.instrument);
    }
}
