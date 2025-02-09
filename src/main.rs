use core::fmt::{Result, Display, Formatter};

fn main() {
    let i : Instrument = Instrument::new(InstrumentType::Guitar, TuningType::Standard, PitchedNote::new("E", 4), 6, 24);
    print!("{}", i);
    let s : Scale = Scale::new(&Note::new("D"), &ScaleDef::new_blues());
    print!("{}", s.pattern);
    print!("{}", s);
    let c : Chord = Chord::new(&Note::new("D"), &ChordDef::new_major());
    print!("{}", c.definition);
    print!("{}", c);
}

struct Instrument {
    instrument_type : InstrumentType,
    tuning_type : TuningType,
    root_note : PitchedNote,
    string_count : usize,
    fret_count : usize,
    tuning : Vec<PitchedNote>,
    fretboard : Vec<Vec<PitchedNote>>
}

impl Display for Instrument {

    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
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

    fn new(instrument_type : InstrumentType, tuning_type : TuningType,
     root_note : PitchedNote, string_count : usize, fret_count : usize) -> Self {
        let mut instrument = Instrument {
            instrument_type,
            tuning_type : tuning_type.clone(),
            root_note : root_note.clone(),
            string_count,
            fret_count : fret_count + 1,
            tuning : Vec::new(),      
            fretboard : Vec::new()
            };
        Instrument::calculate_tuning(&mut instrument);
        Instrument::calculate_notes(&mut instrument);
        instrument
    }

   fn calculate_notes(self : &mut Self) {
        let mut notes :Vec<Vec<PitchedNote>> = Vec::new();
        for i in 0..self.string_count {
            let mut musical_string: Vec<PitchedNote> = Vec::new();
            for j in 0..self.fret_count {
                musical_string.push(PitchedNote::find_note(&self.tuning[i], j.try_into().unwrap()));
            }
            notes.push(musical_string.clone());
        }
        self.fretboard = notes;
    }

    fn calculate_tuning(self : &mut Self) {
        self.tuning.push(self.root_note.clone());   
        match self.instrument_type {
            InstrumentType::Guitar => {
                match self.tuning_type {
                    TuningType::Standard => {
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 5));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[1], 5));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[2], 5));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[3], 4));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[4], 5));
                    },
                    TuningType::Drop => {
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 7));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[1], 5));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[2], 5));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[3], 4));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[4], 5));
                    },
                    TuningType::Open  => {
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 7));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 12));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 16));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[3], 3));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 24));
                    },
                    _ => todo!()
                }

            },
            InstrumentType::Bass => {
                match self.tuning_type {
                    TuningType::Standard => {
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 5));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[1], 5));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[2], 5));
                    },
                    TuningType::Drop => {
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 7));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[1], 5));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[2], 5));
                    },
                    TuningType::Open => {
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 7));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 12));
                        self.tuning.push(PitchedNote::find_note(&self.tuning[0], 16));
                    },
                    _ => todo!()
                }
            },
            _ => todo!()
        }
    }
    
}   

#[derive(Clone, Debug)]
struct PitchedNote {
    note : Note,
    octave : u8
}

impl Display for PitchedNote {

    fn fmt(&self, f : &mut Formatter<'_>) -> Result{
    write!(f, "{}{}", self.note, self.octave)?;
    Ok(())
    }
}

impl PitchedNote {

    fn new(note_name : &str, octave : u8) -> Self {
        match note_name {
            "A" => PitchedNote {
                note : Note::A,
                octave
            },
            "Asharp" => PitchedNote {
                note : Note::Asharp,
                octave
            },
            "B" => PitchedNote {
                note : Note::B, 
                octave
            },
            "C" => PitchedNote {
                note : Note::C, 
                octave
            },
            "Csharp" => PitchedNote {
                note : Note::Csharp, 
                octave
            },
            "D" => PitchedNote {
                note : Note::D, 
                octave
            },
            "Dsharp" => PitchedNote {
                note : Note::Dsharp, 
                octave
            },
            "E" => PitchedNote {
                note : Note::E, 
                octave
            },
            "F" => PitchedNote {
                note : Note::F, 
                octave
            },
            "Fsharp" => PitchedNote {
                note : Note::Fsharp, 
                octave
            },
            "G" => PitchedNote {
                note : Note::G, 
                octave
            },
            "Gsharp" => PitchedNote {
                note : Note::Gsharp, 
                octave
            },
            _ => PitchedNote {
                note : Note::C, 
                octave
            }
        }  
    }

    fn number_to_pitched_note(note_number : u8, octave : u8) -> PitchedNote {
        match note_number {
            0 => PitchedNote {
                note : Note::C,
                octave
            },
            1 => PitchedNote {
                note : Note::Csharp,
                octave
            },
            2 => PitchedNote {
                note : Note::D,
                octave
            },
            3 => PitchedNote {
                note : Note::Dsharp,
                octave
            },
            4 => PitchedNote {
                note : Note::E,
                octave
            },
            5 => PitchedNote {
                note : Note::F,
                octave
            },
            6 => PitchedNote {
                note : Note::Fsharp,
                octave
            },
            7 => PitchedNote {
                note : Note::G,
                octave
            },
            8 => PitchedNote {
                note : Note::Gsharp,
                octave
            },
            9 => PitchedNote {
                note : Note::A,
                octave
            },
            10 => PitchedNote {
                note : Note::Asharp,
                octave
            },
            11 => PitchedNote {
                note : Note::B,
                octave
            },
            _ => PitchedNote {
                note : Note::A,
                octave
            }
        }
    }

    fn pitched_note_to_number(pitched_note : &PitchedNote) -> (u8, u8) {
        match pitched_note.note {
            Note::C => (0, pitched_note.octave),
            Note::Csharp => (1, pitched_note.octave),
            Note::D => (2, pitched_note.octave),
            Note::Dsharp => (3, pitched_note.octave),
            Note::E => (4, pitched_note.octave),
            Note::F => (5, pitched_note.octave),
            Note::Fsharp => (6, pitched_note.octave),
            Note::G => (7, pitched_note.octave),
            Note::Gsharp => (8, pitched_note.octave),
            Note::A => (9, pitched_note.octave),
            Note::Asharp => (10, pitched_note.octave),
            Note::B => (11, pitched_note.octave)
        }
    }

    fn find_note(open_note : &PitchedNote, fret : u8) -> PitchedNote {
        let (x, y) = PitchedNote::add(&open_note, fret);
        PitchedNote::number_to_pitched_note(x, y)
    }

    fn add(start_note : &PitchedNote, to_add : u8) -> (u8, u8) {
        let (note_number, octave) = PitchedNote::pitched_note_to_number(&start_note);
        let mut octave = octave;
        let mut number = note_number + to_add;
        while number >= 12 {
            number = number - 12;
            octave = octave + 1;
        }
        (number, octave)
    }

    fn minus(start_note : &PitchedNote, to_add : u8) -> (u8, u8) {
        let (note_number, octave) = PitchedNote::pitched_note_to_number(&start_note);
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
enum Note {
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
    Gsharp
}

impl Note {

    fn new(name : &str) -> Self {
        match name {
            "A" => Note::A, 
            "Asharp" =>  Note::Asharp,
            "B" => Note::B, 
            "C" => Note::C,
            "Csharp" =>  Note::Csharp,
            "D" => Note::D,
            "Dsharp" => Note::Dsharp,
            "E" => Note::E,
            "F" => Note::F,
            "Fsharp" => Note::Fsharp, 
            "G" =>Note::G,
            "Gsharp" => Note::Gsharp,
            _ => Note::C
        }  
    }

    fn up_step(start_note : &Note, step : &Step) -> Note {
        let number = Note::add(start_note, step);
        Note::number_to_note(number)
    }

    fn down_step(start_note : &Note, step : &Step) -> Note {
        let number = Note::minus(start_note, step);
        Note::number_to_note(number)
    }

    fn add(start_note : &Note, step : &Step) -> u8 {
        let note_number = Note::note_to_number(start_note);
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

    fn minus(start_note : &Note, step : &Step) -> u8 {
        let note_number = Note::note_to_number(start_note);
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

    fn note_to_number(note : &Note) -> u8 {
        match note {
            Note::C => 0,
            Note::Csharp => 1,
            Note::D => 2,
            Note::Dsharp => 3,
            Note::E => 4,
            Note::F => 5,
            Note::Fsharp => 6,
            Note::G => 7,
            Note::Gsharp => 8,
            Note::A => 9,
            Note::Asharp => 10,
            Note::B => 11
        }
    }

    fn number_to_note(number : u8) -> Note {
        match number {
            0 => Note::C,
            1 => Note::Csharp,
            2 => Note::D,
            3 => Note::Dsharp,
            4 => Note::E,
            5 => Note::F,
            6 => Note::Fsharp,
            7 => Note::G,
            8 => Note::Gsharp,
            9 => Note::A,
            10 => Note::Asharp,
            11 => Note::B,
            _ => Note::C
        }
    }
    fn get_name(self : &Self) -> &str {
        match self {
            Note::A => "A",
            Note::Asharp => "A♯",
            Note::B => "B",
            Note::C => "C",
            Note::Csharp => "C♯",
            Note::D => "D",
            Note::Dsharp => "D♯",
            Note::E => "E",
            Note::F => "F",
            Note::Fsharp => "F♯",
            Note::G => "G",
            Note::Gsharp => "G♯"
        }
    }
}

impl Display for Note {

    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        let s = Note::get_name(&self);
        write!(f, "{}", s)?;
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
enum TuningType {
    Open,
    Drop,
    Standard,
    Custom
}

#[derive(PartialEq, Clone, Debug)]
enum InstrumentType {
    Guitar,
    Bass,
    Mandolin,
    Banjo,
    Ukelelle
}

#[derive(PartialEq, Clone,  Debug)]
struct StepValue {
    value : u8
}

#[derive(PartialEq, Clone, Debug)]
enum Step{
    Whole(StepValue),
    Half(StepValue),
    OneAndAHalf(StepValue)
}

impl Step{

    fn new_whole() -> Self {
        Step::Whole(StepValue { 
            value : 2
        })
    }
    fn new_half() -> Self {
        Step::Half(StepValue {
            value : 1
        })
    }
    fn new_one_and_a_half() -> Self {
        Step::OneAndAHalf(StepValue { 
            value : 3
        })
    }
}

impl Display for Step {

    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
       let s = match self {
            Step::Whole(_step_value) => "W".to_string(), 
            Step::Half(_step_value) =>  "H".to_string(),
            Step::OneAndAHalf(_step_value) => "3/2".to_string(),
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
struct ScaleDef {
    name : String,
    steps : Vec<Step>
}

impl ScaleDef {

    fn new_major() -> Self {
        let mut  steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        ScaleDef{
            name : "Major".to_string(),
            steps
        }
    }

    fn new_ionian() -> Self {
        let mut  steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        ScaleDef{
            name : "Ionian".to_string(),
            steps
        }
    }

    fn new_dorian() -> Self {
        let mut  steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Dorian".to_string(),
            steps
        }
    }

    fn new_phrygian() -> Self {
        let mut  steps : Vec<Step> = Vec::new();
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Phrygian".to_string(),
            steps
        }
    }

    fn new_lydian() -> Self {
        let mut  steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        ScaleDef{
            name : "Lydian".to_string(),
            steps
        }
    }

    fn new_mixolydian() -> Self {
        let mut  steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Mixolydian".to_string(),
            steps
        }
    }

    fn new_aeolian() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Aeolian".to_string(),
            steps
        }
    }

    fn new_locrian() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Locrian".to_string(),
            steps
        }
    }

    fn new_natural_minor() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Natural Minor".to_string(),
            steps
        }
    }

    fn new_harmonic_minor() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_half());
        ScaleDef{
            name : "Harmonic Minor".to_string(),
            steps
        }
    }

    fn new_melodic_minor_ascending() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        ScaleDef{
            name : "Melodic Minor Ascending".to_string(),
            steps
        } 
    }

    fn new_melodic_minor_descending() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Melodic Minor Descending".to_string(),
            steps
        } 
    }

    fn new_chromatic() -> Self {
        let mut steps : Vec<Step> = Vec::new();
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
        ScaleDef{
            name : "Chromatic".to_string(),
            steps
        } 
    }

    fn new_whole_tone() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Whole Tone".to_string(),
            steps
        } 
    }

    fn new_major_pentatonic() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_one_and_a_half());
        ScaleDef{
            name : "Major Pentatonic".to_string(),
            steps
        } 
    }

    fn new_minor_pentatonic() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_whole());
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Minor Pentatonic".to_string(),
            steps
        } 
    }

    fn new_blues() -> Self {
        let mut steps : Vec<Step> = Vec::new();
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        steps.push(Step::new_half());
        steps.push(Step::new_half());
        steps.push(Step::new_one_and_a_half());
        steps.push(Step::new_whole());
        ScaleDef{
            name : "Blues".to_string(),
            steps
        } 
    }

}

impl Display for ScaleDef {

    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        write!(f, "{} scale: ", self.name)?;
        for step in &self.steps { 
        write!(f, "{} ", step)?;
        };
        write!(f, "\n")?;
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Scale {
    pattern : ScaleDef,
    notes : Vec<Note>
}

impl Scale {

    fn new(note_1 : &Note, pattern : &ScaleDef) -> Self {
        let mut notes : Vec<Note> = Vec::new();
        notes.push(note_1.clone());
        let mut count : usize = 0;
        for step in &pattern.steps {
            notes.push(Note::up_step(&notes[count], &step));
            count = count + 1;
        }
        Scale{pattern : pattern.clone(), notes}
    }
}

impl Display for Scale {
    
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        write!(f, "{} {} scale: ", self.notes[0], self.pattern.name)?;
        for note in &self.notes {
            write!(f, "{} ", note)?;
        };
        write!(f, "\n")?;
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Accidental {
    Sharp,
    Flat,
    None
}
impl Display for Accidental {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
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
    accidental : Accidental,
    interval : u8
}

impl Display for Interval {

    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
            write!(f, "{}{}", self.accidental, self.interval)?;
            Ok(())
    }

}
#[derive(PartialEq, Clone, Debug)]
struct ChordDef {
    name : String,
    naming_convention : String,
    intervals : Vec<Interval>,
}

impl Display for ChordDef {
    
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        write!(f, "{} chord: ", self.name)?;
        for interval in &self.intervals {
            write!(f, "{} ", interval)?;
        };
        write!(f, "\n")?;
        Ok(())
    }
}

impl ChordDef {

    fn new_major() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        ChordDef{
            name : "Major".to_string(), 
            naming_convention : "".to_string(), 
            intervals 
        }
    }

    fn new_minor() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        ChordDef {
            name : "Minor".to_string(),       
            naming_convention : "m".to_string(), 
            intervals 
        }
    }

    fn new_diminished() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental : Accidental::None,
            interval : 1
        });
        intervals.push(Interval {
            accidental : Accidental::Flat,
            interval : 3
        });
        intervals.push(Interval {
            accidental : Accidental::Flat,
            interval : 5
        });
        ChordDef {
            name : "Diminished".to_string(),
            naming_convention : "dim".to_string(), 
            intervals 
        }
    }

    fn new_augmented() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval {
            accidental : Accidental::None,
            interval : 1
        });
        intervals.push(Interval {
            accidental : Accidental::None,
            interval : 3
        });
        intervals.push(Interval {
            accidental : Accidental::Sharp,
            interval : 5
        });
        ChordDef {
            name : "Augmented".to_string(),
            naming_convention : "aug".to_string(), 
            intervals 
        }
    }

    fn new_suspended_second() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 2 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        ChordDef { 
            name : "Suspended 2nd".to_string(),
            naming_convention : "sus2".to_string(), 
            intervals 
        }
    }

    fn new_suspended_fourth() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 4 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        ChordDef { 
            name : "Suspended 4th".to_string(),
            naming_convention : "sus4".to_string(), 
            intervals 
        }
    }

    fn new_power() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        ChordDef {
            name : "Power".to_string(), 
            naming_convention : "5".to_string(), 
            intervals 
        }
    }

    fn new_major_seventh() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 7 
        });
        ChordDef { 
            name : "Major 7th".to_string(),
            naming_convention : "maj7".to_string(), 
            intervals 
        }
    }

    fn new_minor_seventh() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        ChordDef { 
            name : "Minor 7th".to_string(),
            naming_convention : "m7".to_string(), 
            intervals 
        }
    }

    fn new_dominant_seventh() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        ChordDef {
            name : "Dominant 7th".to_string(), 
            naming_convention : "7".to_string(), 
            intervals 
        }
    }

    fn new_minor_major_7() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 7 
        });
        ChordDef { 
            name : "Minor Major 7th".to_string(),
            naming_convention : "m(Maj7)".to_string(), 
            intervals 
        }
    }

    fn new_sixth() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 6 
        });
        ChordDef { 
            name : "6th".to_string(),
            naming_convention : "6".to_string(), 
            intervals 
        }
    }

    fn new_minor_sixth() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 6 
        });
        ChordDef { 
            name : "Minor 6th".to_string(),
            naming_convention : "m6".to_string(), 
            intervals 
        }
    }
    fn new_ninth() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 9 
        });
        ChordDef {
            name : "9th".to_string(), 
            naming_convention : "9".to_string(), 
            intervals 
        }
    }

    fn new_minor_ninth() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 9 
        });
        ChordDef {
            name : "Minor 9th".to_string(), 
            naming_convention : "m9".to_string(), 
            intervals 
        }
    }

    fn new_add_ninth() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 9 
        });
        ChordDef { 
            name : "Add 9th".to_string(),
            naming_convention : "add9".to_string(), 
            intervals 
        }
    }

    fn new_seventh_suspended_fourth() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 4 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        ChordDef { 
            name : "7th Suspended 4th".to_string(),
            naming_convention : "7sus4".to_string(), 
            intervals 
        }
    }

    fn new_dimished_seventh() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 6 
        });
        ChordDef {
            name : "Diminished 7th".to_string(), 
            naming_convention : "dim7".to_string(), 
            intervals 
        }
    }

    fn new_half_diminished() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        ChordDef {
            name : "Half Diminished".to_string(), 
            naming_convention : "7♭5".to_string(), 
            intervals 
        }
    }

    fn new_plus_seventh() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::Sharp, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        ChordDef {
            name : "Plus 7th".to_string(), 
            naming_convention : "+7".to_string(), 
            intervals 
        }
    }

    fn new_minor_eleventh() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 9 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 11 
        });
        ChordDef { 
            name : "Minor 11th".to_string(),
            naming_convention : "m11".to_string(), 
            intervals 
        }
    }

    fn new_augmented_major_seventh() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::Sharp, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 7 
        });
        ChordDef {
            name : "Augmented Major 7th".to_string(), 
            naming_convention : "Maj7♯5".to_string(), 
            intervals 
        }
    }

    fn new_dominant_seventh_flat_ninth() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 9 
        });
        ChordDef {
            name : "Dominant 7th Flat 9th".to_string(), 
            naming_convention : "7♭9".to_string(), 
            intervals 
        }
    }

    fn new_altered_dominant_seventh() -> Self {
        let mut intervals : Vec<Interval> = Vec::new();
        intervals.push(Interval { 
            accidental : Accidental::None,
            interval : 1 
        });
        intervals.push(Interval { 
            accidental : Accidental::None, 
            interval : 3 
        });
        intervals.push(Interval { 
            accidental : Accidental::Sharp, 
            interval : 5 
        });
        intervals.push(Interval { 
            accidental : Accidental::Flat, 
            interval : 7 
        });
        intervals.push(Interval { 
            accidental : Accidental::Sharp, 
            interval : 9 
        });
        ChordDef {
            name : "Altered Dominant 7th".to_string(), 
            naming_convention : "7♯5♯9".to_string(), 
            intervals 
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Chord {
    definition : ChordDef,
    notes : Vec<Note>,
    name : String,
    short_name : String,
}

impl Display for Chord {
    
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        write!(f, "{} chord: ", self.name)?;
        for note in &self.notes {
            write!(f, "{} ", note)?;
        };
        write!(f, "\n")?;
        Ok(())
    }
}

impl Chord {

    fn new(root_note : &Note, definition : &ChordDef) -> Self {
        let scale : Scale = Scale::new(&root_note, &ScaleDef::new_major());
        let mut notes : Vec<Note> = Vec::new();
        for interval in &definition.intervals {
            if interval.accidental == Accidental::None {
                notes.push(scale.notes[(interval.interval - 1) as usize].clone());
            }
            if interval.accidental == Accidental::Flat {
                let mut note : Note = scale.notes[(interval.interval - 1) as usize].clone();
                note = Note::down_step(&note, &Step::new_half());
                notes.push(note);
            }
             if interval.accidental == Accidental::Sharp {
                let mut note : Note = scale.notes[(interval.interval - 1) as usize].clone();
                note = Note::up_step(&note, &Step::new_half());
                notes.push(note);
            }
        }
        Chord {
            definition : definition.clone(),
            notes,
            name : Note::get_name(&root_note).to_owned() + " " + &definition.name,
            short_name : Note::get_name(&root_note).to_owned() + &definition.naming_convention
        }
    }
}