use core::fmt::{Display, Formatter, Result};

fn main() {
    let i: Instrument = Instrument::new(
        InstrumentType::Guitar,
        TuningType::Standard,
        NotePitch::new("E", 4),
        6,
        24,
    );
    print!("{}", i);
    let s: Scale = Scale::new(&NoteName::new("D"), &ScaleDef::new_blues());
    print!("{}", s.pattern);
    print!("{}", s);
    let c: Chord = Chord::new(&NoteName::new("D"), &ChordDef::new_minor());
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
            for j in 0..self.fret_count {
                write!(f, "{} ", self.fretboard[i][j])?;
            }
            write!(f, "\n")?;
        }
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
                musical_string.push(NotePitch::find_note(
                    &self.tuning[i],
                    j.try_into().unwrap(),
                ));
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
                    self.tuning
                        .push(NotePitch::find_note(&self.tuning[0], 12));
                    self.tuning
                        .push(NotePitch::find_note(&self.tuning[0], 16));
                    self.tuning.push(NotePitch::find_note(&self.tuning[3], 3));
                    self.tuning
                        .push(NotePitch::find_note(&self.tuning[0], 24));
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
                    self.tuning
                        .push(NotePitch::find_note(&self.tuning[0], 12));
                    self.tuning
                        .push(NotePitch::find_note(&self.tuning[0], 16));
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
struct NotePitch {
    note: NoteName,
    octave: u8,
}

impl Display for NotePitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.note, self.octave)?;
        Ok(())
    }
}

impl NotePitch {
    fn new(note_name: &str, octave: u8) -> Self {
        match note_name {
            "A" => NotePitch {
                note: NoteName::A,
                octave,
            },
            "Asharp" => NotePitch {
                note: NoteName::Asharp,
                octave,
            },
            "B" => NotePitch {
                note: NoteName::B,
                octave,
            },
            "C" => NotePitch {
                note: NoteName::C,
                octave,
            },
            "Csharp" => NotePitch {
                note: NoteName::Csharp,
                octave,
            },
            "D" => NotePitch {
                note: NoteName::D,
                octave,
            },
            "Dsharp" => NotePitch {
                note: NoteName::Dsharp,
                octave,
            },
            "E" => NotePitch {
                note: NoteName::E,
                octave,
            },
            "F" => NotePitch {
                note: NoteName::F,
                octave,
            },
            "Fsharp" => NotePitch {
                note: NoteName::Fsharp,
                octave,
            },
            "G" => NotePitch {
                note: NoteName::G,
                octave,
            },
            "Gsharp" => NotePitch {
                note: NoteName::Gsharp,
                octave,
            },
            _ => NotePitch {
                note: NoteName::C,
                octave,
            },
        }
    }

    fn number_to_note_pitch(note_number: u8, octave: u8) -> NotePitch {
        match note_number {
            0 => NotePitch {
                note: NoteName::C,
                octave,
            },
            1 => NotePitch {
                note: NoteName::Csharp,
                octave,
            },
            2 => NotePitch {
                note: NoteName::D,
                octave,
            },
            3 => NotePitch {
                note: NoteName::Dsharp,
                octave,
            },
            4 => NotePitch {
                note: NoteName::E,
                octave,
            },
            5 => NotePitch {
                note: NoteName::F,
                octave,
            },
            6 => NotePitch {
                note: NoteName::Fsharp,
                octave,
            },
            7 => NotePitch {
                note: NoteName::G,
                octave,
            },
            8 => NotePitch {
                note: NoteName::Gsharp,
                octave,
            },
            9 => NotePitch {
                note: NoteName::A,
                octave,
            },
            10 => NotePitch {
                note: NoteName::Asharp,
                octave,
            },
            11 => NotePitch {
                note: NoteName::B,
                octave,
            },
            _ => NotePitch {
                note: NoteName::A,
                octave,
            },
        }
    }

    fn note_pitch_to_number(note_pitch: &NotePitch) -> (u8, u8) {
        match note_pitch.note {
            NoteName::C => (0, note_pitch.octave),
            NoteName::Csharp => (1, note_pitch.octave),
            NoteName::D => (2, note_pitch.octave),
            NoteName::Dsharp => (3, note_pitch.octave),
            NoteName::E => (4, note_pitch.octave),
            NoteName::F => (5, note_pitch.octave),
            NoteName::Fsharp => (6, note_pitch.octave),
            NoteName::G => (7, note_pitch.octave),
            NoteName::Gsharp => (8, note_pitch.octave),
            NoteName::A => (9, note_pitch.octave),
            NoteName::Asharp => (10, note_pitch.octave),
            NoteName::B => (11, note_pitch.octave),
        }
    }

    fn find_note(open_note: &NotePitch, fret: u8) -> NotePitch {
        let (x, y) = NotePitch::add(&open_note, fret);
        NotePitch::number_to_note_pitch(x, y)
    }

    fn add(start_note: &NotePitch, to_add: u8) -> (u8, u8) {
        let (note_number, octave) = NotePitch::note_pitch_to_number(&start_note);
        let mut octave = octave;
        let mut number = note_number + to_add;
        while number >= 12 {
            number = number - 12;
            octave = octave + 1;
        }
        (number, octave)
    }

    fn minus(start_note: &NotePitch, to_add: u8) -> (u8, u8) {
        let (note_number, octave) = NotePitch::note_pitch_to_number(&start_note);
        let mut octave = octave;
        let mut number = note_number - to_add;
        while number < 0 {
            number = number + 12;
            octave = octave - 1;
        }
        (number, octave)
    }
}

#[derive(PartialEq, Clone, Debug)]
enum NoteName {
    A,
    Asharp,
    B,
    C,
    Csharp,
    D,
    Dsharp,
    E,
    F,
    Fsharp,
    G,
    Gsharp,
}

impl NoteName {
    fn new(name: &str) -> Self {
        match name {
            "A" => NoteName::A,
            "Asharp" => NoteName::Asharp,
            "B" => NoteName::B,
            "C" => NoteName::C,
            "Csharp" => NoteName::Csharp,
            "D" => NoteName::D,
            "Dsharp" => NoteName::Dsharp,
            "E" => NoteName::E,
            "F" => NoteName::F,
            "Fsharp" => NoteName::Fsharp,
            "G" => NoteName::G,
            "Gsharp" => NoteName::Gsharp,
            _ => NoteName::C,
        }
    }

    fn up_step(start_note: &NoteName, step: &Step) -> NoteName {
        let number = NoteName::add(start_note, step);
        NoteName::number_to_note(number)
    }

    fn down_step(start_note: &NoteName, step: &Step) -> NoteName {
        let number = NoteName::minus(start_note, step);
        NoteName::number_to_note(number)
    }

    fn add(start_note: &NoteName, step: &Step) -> u8 {
        let note_number = NoteName::note_to_number(start_note);
        let to_add = match step {
            Step::Whole(step_value) => step_value.value,
            Step::Half(step_value) => step_value.value,
            Step::OneAndAHalf(step_value) => step_value.value,
        };
        let mut number = note_number + to_add;
        while number >= 12 {
            number = number - 12;
        }
        number
    }

    fn minus(start_note: &NoteName, step: &Step) -> u8 {
        let note_number = NoteName::note_to_number(start_note);
        let to_add = match step {
            Step::Whole(step_value) => step_value.value,
            Step::Half(step_value) => step_value.value,
            Step::OneAndAHalf(step_value) => step_value.value,
        };
        let mut number = note_number - to_add;
        while number < 0 {
            number = number + 12;
        }
        number
    }

    fn note_to_number(note: &NoteName) -> u8 {
        match note {
            NoteName::C => 0,
            NoteName::Csharp => 1,
            NoteName::D => 2,
            NoteName::Dsharp => 3,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::Fsharp => 6,
            NoteName::G => 7,
            NoteName::Gsharp => 8,
            NoteName::A => 9,
            NoteName::Asharp => 10,
            NoteName::B => 11,
        }
    }

    fn number_to_note(number: u8) -> NoteName {
        match number {
            0 => NoteName::C,
            1 => NoteName::Csharp,
            2 => NoteName::D,
            3 => NoteName::Dsharp,
            4 => NoteName::E,
            5 => NoteName::F,
            6 => NoteName::Fsharp,
            7 => NoteName::G,
            8 => NoteName::Gsharp,
            9 => NoteName::A,
            10 => NoteName::Asharp,
            11 => NoteName::B,
            _ => NoteName::C,
        }
    }
    fn get_name(self: &Self) -> &str {
        match self {
            NoteName::A => "A",
            NoteName::Asharp => "A♯",
            NoteName::B => "B",
            NoteName::C => "C",
            NoteName::Csharp => "C♯",
            NoteName::D => "D",
            NoteName::Dsharp => "D♯",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::Fsharp => "F♯",
            NoteName::G => "G",
            NoteName::Gsharp => "G♯",
        }
    }
}

impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = NoteName::get_name(&self);
        write!(f, "{}", s)?;
        Ok(())
    }
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
    value: u8,
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
    pattern: ScaleDef,
    notes: Vec<NoteName>,
}

impl Scale {
    fn new(note_1: &NoteName, pattern: &ScaleDef) -> Self {
        let mut notes: Vec<NoteName> = Vec::new();
        notes.push(note_1.clone());
        let mut count: usize = 0;
        for step in &pattern.steps {
            notes.push(NoteName::up_step(&notes[count], &step));
            count = count + 1;
        }
        Scale {
            pattern: pattern.clone(),
            notes,
        }
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} scale: ", self.notes[0], self.pattern.name)?;
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
    None,
}
impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Accidental::Sharp => write!(f, "♯")?,
            Accidental::Flat => write!(f, "♭")?,
            Accidental::None => write!(f, "")?,
        }
        Ok(())
    }
}
#[derive(PartialEq, Clone, Debug)]
struct Interval {
    accidental: Accidental,
    interval: u8,
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.accidental, self.interval)?;
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::Sharp,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 2,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 4,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 7,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 7,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 4,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::Sharp,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 7,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 9,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::Sharp,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 7,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
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
            accidental: Accidental::None,
            interval: 1,
        });
        intervals.push(Interval {
            accidental: Accidental::None,
            interval: 3,
        });
        intervals.push(Interval {
            accidental: Accidental::Sharp,
            interval: 5,
        });
        intervals.push(Interval {
            accidental: Accidental::Flat,
            interval: 7,
        });
        intervals.push(Interval {
            accidental: Accidental::Sharp,
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
    notes: Vec<NoteName>,
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
    fn new(root_note: &NoteName, definition: &ChordDef) -> Self {
        let scale: Scale = Scale::new(&root_note, &ScaleDef::new_major());
        let mut notes: Vec<NoteName> = Vec::new();
        for interval in &definition.intervals {
            if interval.accidental == Accidental::None {
                notes.push(scale.notes[(interval.interval - 1) as usize].clone());
            }
            if interval.accidental == Accidental::Flat {
                let mut note: NoteName = scale.notes[(interval.interval - 1) as usize].clone();
                note = NoteName::down_step(&note, &Step::new_half());
                notes.push(note);
            }
            if interval.accidental == Accidental::Sharp {
                let mut note: NoteName = scale.notes[(interval.interval - 1) as usize].clone();
                note = NoteName::up_step(&note, &Step::new_half());
                notes.push(note);
            }
        }
        Chord {
            definition: definition.clone(),
            notes,
            name: NoteName::get_name(&root_note).to_owned() + " " + &definition.name,
            short_name: NoteName::get_name(&root_note).to_owned() + &definition.naming_convention,
        }
    }
}
