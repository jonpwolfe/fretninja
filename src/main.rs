use core::fmt::{Display, Formatter, Result};

fn main() {
    let i: Instrument = Instrument::new(
        InstrumentType::Guitar,
        TuningType::Standard,
        NotePitch::new(&NaturalNote::C, &None, 4),
        6,
        24,
    );
    print!("{}", i);
    let s: Scale = Scale::new(
        &NotePitch::new(&NaturalNote::D, &None, 5),
        &ScaleDef::new_major(),
    );
    print!("{}", s.definition);
    print!("{}", s);
    let c: Chord = Chord::new(
        &NotePitch::new(&NaturalNote::D, &None, 5),
        &ChordDef::new_minor_eleventh(),
    );
    print!("{}", c.definition);
    print!("{}", c);
}

struct Instrument {
    instrument_type: InstrumentType,
    tuning_type: TuningType,
    root_note: NotePitch,
    string_count: usize,
    fret_count: usize,
    tuning: Vec<NotePitch>,
    fretboard: Vec<Vec<NotePitch>>,
}

impl Display for Instrument {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for i in (0..self.string_count).rev() {
            write!(f, "{} ", self.string_count - i);
            for j in 0..self.fret_count {
                match &self.fretboard[i][j].accidental {
                    None => write!(f, "{}  ", self.fretboard[i][j])?,
                    Some(_Accidental) => write!(f, "{} ", self.fretboard[i][j])?,
                };
            }
            write!(f, "\n")?;
        }
        write!(f, "  ")?;
        for i in 0..self.fret_count {
            match i {
                0..=9 => write!(f, "{}   ", i)?,
                10.. => write!(f, "{}  ", i)?,
                _ => panic!("unexpected fret_count"),
            };
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl Instrument {
    fn new(
        instrument_type: InstrumentType,
        tuning_type: TuningType,
        root_note: NotePitch,
        string_count: usize,
        fret_count: usize,
    ) -> Self {
        let mut instrument = Instrument {
            instrument_type,
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
        let mut notes: Vec<Vec<NotePitch>> = Vec::new();
        for i in 0..self.string_count {
            let mut musical_string: Vec<NotePitch> = Vec::new();
            for j in 0..self.fret_count {
                musical_string.push(NotePitch::find_note(&self.tuning[i], j.try_into().unwrap()));
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
}

#[derive(PartialEq, Clone, Debug)]
struct NotePitch {
    natural_note: NaturalNote,
    accidental: Option<Accidental>,
    octave: i8,
}

impl Display for NotePitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", NotePitch::get_name(&self), self.octave)?;
        Ok(())
    }
}

impl NotePitch {
    fn new(natural_note: &NaturalNote, accidental: &Option<Accidental>, octave: i8) -> Self {
        NotePitch {
            natural_note: natural_note.clone(),
            accidental: accidental.clone(),
            octave,
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
    fn from_number(note_number: i8, octave: i8) -> NotePitch {
        match note_number {
            0 => NotePitch {
                natural_note: NaturalNote::C,
                accidental: None,
                octave,
            },
            1 => NotePitch {
                natural_note: NaturalNote::C,
                accidental: Some(Accidental::Sharp),
                octave,
            },
            2 => NotePitch {
                natural_note: NaturalNote::D,
                accidental: None,
                octave,
            },
            3 => NotePitch {
                natural_note: NaturalNote::D,
                accidental: Some(Accidental::Sharp),
                octave,
            },
            4 => NotePitch {
                natural_note: NaturalNote::E,
                accidental: None,
                octave,
            },
            5 => NotePitch {
                natural_note: NaturalNote::F,
                accidental: None,
                octave,
            },
            6 => NotePitch {
                natural_note: NaturalNote::F,
                accidental: Some(Accidental::Sharp),
                octave,
            },
            7 => NotePitch {
                natural_note: NaturalNote::G,
                accidental: None,
                octave,
            },
            8 => NotePitch {
                natural_note: NaturalNote::G,
                accidental: Some(Accidental::Sharp),
                octave,
            },
            9 => NotePitch {
                natural_note: NaturalNote::A,
                accidental: None,
                octave,
            },
            10 => NotePitch {
                natural_note: NaturalNote::A,
                accidental: Some(Accidental::Sharp),
                octave,
            },
            11 => NotePitch {
                natural_note: NaturalNote::B,
                accidental: None,
                octave,
            },
            _ => panic!("unexpected note number"),
        }
    }

    fn to_number(note_pitch: &NotePitch) -> (i8, i8) {
        let number: i8 = match note_pitch.natural_note {
            NaturalNote::C => {
                0 + match note_pitch.accidental {
                    Some(Accidental::Flat) => panic!("unexpected accidental"),
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::D => {
                2 + match note_pitch.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::E => {
                4 + match note_pitch.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => panic!("unexpected accidental"),
                    None => 0,
                }
            }
            NaturalNote::F => {
                5 + match note_pitch.accidental {
                    Some(Accidental::Flat) => panic!("unexpected accidental"),
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::G => {
                7 + match note_pitch.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::A => {
                9 + match note_pitch.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => 1,
                    None => 0,
                }
            }
            NaturalNote::B => {
                11 + match note_pitch.accidental {
                    Some(Accidental::Flat) => -1,
                    Some(Accidental::Sharp) => panic!("unexpected accidental"),
                    None => 0,
                }
            }
        };
        match number {
            1 => return (11, note_pitch.octave - 1),
            _ => return (number, note_pitch.octave),
        };
    }

    fn find_note(open_note: &NotePitch, fret: i8) -> NotePitch {
        let (x, y) = NotePitch::add(&open_note, fret);
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
        let s = match self {
            Step::Whole(_step_value) => "W".to_string(),
            Step::Half(_step_value) => "H".to_string(),
            Step::OneAndAHalf(_step_value) => "3/2".to_string(),
        };
        write!(f, "{}", s)?;
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
        write!(f, "{} scale: ", self.name)?;
        for step in &self.steps {
            write!(f, "{} ", step)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Scale {
    definition: ScaleDef,
    notes: Vec<NotePitch>,
}

impl Scale {
    fn new(note_1: &NotePitch, definition: &ScaleDef) -> Self {
        let mut notes: Vec<NotePitch> = Vec::new();
        notes.push(note_1.clone());
        let mut count: usize = 0;
        for step in &definition.steps {
            notes.push(NotePitch::up_step(&notes[count], &step));
            count = count + 1;
        }
        Scale {
            definition: definition.clone(),
            notes,
        }
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} scale: ", self.notes[0], self.definition.name)?;
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
        write!(f, "{} chord: ", self.name)?;
        for interval in &self.intervals {
            write!(f, "{} ", interval)?;
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
            name: "Suspended 2nd".to_string(),
            naming_convention: "sus2".to_string(),
            intervals,
        }
    }

    fn new_suspended_fourth() -> Self {
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
            name: "Suspended 4th".to_string(),
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

    fn new_major_seventh() -> Self {
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
            name: "Major 7th".to_string(),
            naming_convention: "maj7".to_string(),
            intervals,
        }
    }

    fn new_minor_seventh() -> Self {
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
            name: "Minor 7th".to_string(),
            naming_convention: "m7".to_string(),
            intervals,
        }
    }

    fn new_dominant_seventh() -> Self {
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
            name: "Dominant 7th".to_string(),
            naming_convention: "7".to_string(),
            intervals,
        }
    }

    fn new_minor_major_7() -> Self {
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
            name: "Minor Major 7th".to_string(),
            naming_convention: "m(Maj7)".to_string(),
            intervals,
        }
    }

    fn new_sixth() -> Self {
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
            name: "6th".to_string(),
            naming_convention: "6".to_string(),
            intervals,
        }
    }

    fn new_minor_sixth() -> Self {
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
            name: "Minor 6th".to_string(),
            naming_convention: "m6".to_string(),
            intervals,
        }
    }
    fn new_ninth() -> Self {
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
            name: "9th".to_string(),
            naming_convention: "9".to_string(),
            intervals,
        }
    }

    fn new_minor_ninth() -> Self {
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
            name: "Minor 9th".to_string(),
            naming_convention: "m9".to_string(),
            intervals,
        }
    }

    fn new_add_ninth() -> Self {
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
            name: "Add 9th".to_string(),
            naming_convention: "add9".to_string(),
            intervals,
        }
    }

    fn new_seventh_suspended_fourth() -> Self {
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
            name: "7th Suspended 4th".to_string(),
            naming_convention: "7sus4".to_string(),
            intervals,
        }
    }

    fn new_dimished_seventh() -> Self {
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
            name: "Diminished 7th".to_string(),
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

    fn new_plus_seventh() -> Self {
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
            name: "Plus 7th".to_string(),
            naming_convention: "+7".to_string(),
            intervals,
        }
    }

    fn new_minor_eleventh() -> Self {
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
            name: "Minor 11th".to_string(),
            naming_convention: "m11".to_string(),
            intervals,
        }
    }

    fn new_augmented_major_seventh() -> Self {
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
            name: "Augmented Major 7th".to_string(),
            naming_convention: "Maj7♯5".to_string(),
            intervals,
        }
    }

    fn new_dominant_seventh_flat_ninth() -> Self {
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
            name: "Dominant 7th Flat 9th".to_string(),
            naming_convention: "7♭9".to_string(),
            intervals,
        }
    }

    fn new_altered_dominant_seventh() -> Self {
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
            name: "Altered Dominant 7th".to_string(),
            naming_convention: "7♯5♯9".to_string(),
            intervals,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Chord {
    definition: ChordDef,
    notes: Vec<NotePitch>,
    name: String,
    short_name: String,
}

impl Display for Chord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} or {} chord: ", self.name, self.short_name)?;
        for note in &self.notes {
            write!(f, "{} ", note)?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}
impl Chord {
    fn new(root_note: &NotePitch, definition: &ChordDef) -> Self {
        let scale = Scale::new(root_note, &ScaleDef::new_major());
        let mut notes = Vec::new();
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
                Some(Accidental::Flat) => NotePitch::down_step(base_note, &Step::new_half()),
                Some(Accidental::Sharp) => NotePitch::up_step(base_note, &Step::new_half()),
            };

            notes.push(note);
        }

        Chord {
            definition: definition.clone(),
            notes,
            name: format!("{} {}", NotePitch::get_name(root_note), definition.name),
            short_name: format!(
                "{}{}",
                NotePitch::get_name(root_note),
                definition.naming_convention
            ),
        }
    }
}
