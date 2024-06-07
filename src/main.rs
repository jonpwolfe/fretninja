
use strum_macros::EnumString;
use core::fmt::{Result, Display, Formatter};

fn main() {
    let i : Instrument = Instrument::new(TypeOfInstrument::GUITAR, TuningStyle::STANDARD, MusicalNote::new("E",4), 6 ,25);
    print!("{}", i);
}

struct Instrument{
    type_of_instrument : TypeOfInstrument,
    tuning_style : TuningStyle,
    root_note : MusicalNote,
    string_count : u8,
    fret_count : u8,
    tuning : Vec<MusicalNote>,
    fretboard : Vec<Vec<MusicalNote>>
}

impl Display for Instrument {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
       for i in (0..self.string_count as usize).rev(){
            for j in 0..self.fret_count as usize{
        write!(f,"{} ", self.fretboard[i][j])?;
    }
    write!(f,"\n")?;
}
Ok(())
}
}

impl Instrument {
    fn new(type_of_instrument : TypeOfInstrument, tuning_style : TuningStyle, root_note : MusicalNote, string_count : u8, fret_count : u8) -> Self {
        let mut instrument = Instrument{
            type_of_instrument ,
            tuning_style : tuning_style.clone(),
            root_note : root_note.clone(),
            string_count,
            fret_count,
            tuning: Vec::new(),      
            fretboard: Vec::new(),
            };
        Instrument::calculate_tuning(&mut instrument);
        Instrument::calculate_notes(&mut instrument);
        instrument
    }

   fn calculate_notes(self : &mut Self) {
        
        let mut notes :Vec<Vec<MusicalNote>> = Vec::new();
        for i in 0..self.string_count as usize{
            let mut musical_string: Vec<MusicalNote> = Vec::new();
            for j in 0..self.fret_count as usize{
                musical_string.push(MusicalNote::find_note(&self.tuning[i],j.try_into().unwrap()));
           }
         notes.push(musical_string.clone());
        }
        self.fretboard = notes;
    }
    fn calculate_tuning(self: &mut Self){
        self.tuning.push(self.root_note.clone());   
        match self.type_of_instrument {
        TypeOfInstrument::GUITAR=>{
            match self.tuning_style{
             TuningStyle::STANDARD =>{
                self.tuning.push(MusicalNote::find_note(&self.tuning[0], 5));
                self.tuning.push(MusicalNote::find_note(&self.tuning[1], 5));
                self.tuning.push(MusicalNote::find_note(&self.tuning[2], 5));
                self.tuning.push(MusicalNote::find_note(&self.tuning[3], 4));
                self.tuning.push(MusicalNote::find_note(&self.tuning[4], 5));
                }
            TuningStyle::DROP => {
                self.tuning.push(MusicalNote::find_note(&self.tuning[0], 7));
                self.tuning.push(MusicalNote::find_note(&self.tuning[1], 5));
                self.tuning.push(MusicalNote::find_note(&self.tuning[2], 5));
                self.tuning.push(MusicalNote::find_note(&self.tuning[3], 4));
                self.tuning.push(MusicalNote::find_note(&self.tuning[4], 5));
                }
            TuningStyle::OPEN  => {
                self.tuning.push(MusicalNote::find_note(&self.tuning[0], 7));
                self.tuning.push(MusicalNote::find_note(&self.tuning[0],12));
                self.tuning.push(MusicalNote::find_note(&self.tuning[0], 16));
                self.tuning.push(MusicalNote::find_note(&self.tuning[3], 3));
                self.tuning.push(MusicalNote::find_note(&self.tuning[0],24));
                }
                _ => todo!(),
            }

        }
        TypeOfInstrument::BASS=>{
            match self.tuning_style {
                TuningStyle::STANDARD=>{
                    self.tuning.push(MusicalNote::find_note(&self.tuning[0], 5));
                    self.tuning.push(MusicalNote::find_note(&self.tuning[1], 5));
                    self.tuning.push(MusicalNote::find_note(&self.tuning[2], 5));
                    }
                TuningStyle::DROP => {
                    self.tuning.push(MusicalNote::find_note(&self.tuning[0], 7));
                    self.tuning.push(MusicalNote::find_note(&self.tuning[1], 5));
                    self.tuning.push(MusicalNote::find_note(&self.tuning[2], 5));
                    }
                TuningStyle::OPEN => {
                    self.tuning.push(MusicalNote::find_note(&self.tuning[0], 7));
                    self.tuning.push(MusicalNote::find_note(&self.tuning[0], 12));
                    self.tuning.push(MusicalNote::find_note(&self.tuning[0], 16));
                     }
                     _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
    
}   
#[derive(Clone, Debug)]
struct MusicalNote{
    name : NoteName,
    octave : u8,
}

impl Display for MusicalNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
    write!(f,"{}{}", self.name,self.octave)?;
    Ok(())
    }
}

impl MusicalNote{
    fn new(name:&str, octave:u8) -> Self{
        match name{
        "A"=> MusicalNote{name : NoteName::A, octave},
        "Asharp" => MusicalNote{name : NoteName::Asharp, octave},
        "B" => MusicalNote{name : NoteName::B, octave},
        "C" => MusicalNote{name : NoteName::C, octave},
        "Csharp" => MusicalNote{name : NoteName::Csharp, octave},
        "D" => MusicalNote{name : NoteName::D, octave},
        "Dsharp" => MusicalNote{name : NoteName::Dsharp, octave},
        "E" => MusicalNote{name : NoteName::E, octave},
        "F" => MusicalNote{name : NoteName::F, octave},
        "Fsharp" => MusicalNote{name : NoteName::Fsharp, octave},
        "G" => MusicalNote{name : NoteName::G, octave},
        "Gsharp" => MusicalNote{name : NoteName::Gsharp, octave},
        _ => MusicalNote{name : NoteName::C, octave},
      }  
    }
    
fn number_to_note(number: u8, octave :u8) -> MusicalNote{
     match number{
        1 => MusicalNote{name : NoteName::C,octave},
        2 => MusicalNote{name : NoteName::Csharp,octave},
        3 => MusicalNote{name : NoteName::D,octave},
        4 => MusicalNote{name : NoteName::Dsharp,octave},
        5 => MusicalNote{name : NoteName::E,octave},
        6 => MusicalNote{name : NoteName::F,octave},
        7 => MusicalNote{name : NoteName::Fsharp,octave},
        8 => MusicalNote{name : NoteName::G,octave},
        9 => MusicalNote{name : NoteName::Gsharp,octave},
        10 => MusicalNote{name : NoteName::A,octave},
        11 => MusicalNote{name : NoteName::Asharp,octave},
        12 => MusicalNote{name : NoteName::B,octave},
        _ => MusicalNote{name : NoteName::A,octave},
        }
    }
 fn note_to_number(note:&MusicalNote) -> (u8, u8){
    match note.name {
        NoteName::C => (1,note.octave),
        NoteName::Csharp => (2,note.octave),
        NoteName::D => (3,note.octave),
        NoteName::Dsharp => (4,note.octave),
        NoteName::E => (5,note.octave),
        NoteName::F => (6,note.octave),
        NoteName::Fsharp => (7,note.octave),
        NoteName::G => (8,note.octave),
        NoteName::Gsharp => (9,note.octave),
        NoteName::A => (10,note.octave),
        NoteName::Asharp => (11,note.octave),
        NoteName::B => (12,note.octave),
    }
}
  fn find_note(open_note : &MusicalNote, fret: u8) -> MusicalNote{
   let (x,y) = MusicalNote::add(&open_note, fret);
   MusicalNote::number_to_note(x,y)
 }
fn add(start_note: &MusicalNote, to_add: u8) -> (u8, u8) {
    let (note, octave) = MusicalNote::note_to_number(&start_note);
    let mut octave = octave;
    let mut num = note+to_add;
      while num > 12 {
            num=num-12;
            octave=octave+1;
        }
            return (num, octave)
}


}
  #[derive(EnumString,PartialEq, Clone, Debug)]
    pub enum NoteName {
        #[strum(serialize = "A")]
        A,
        #[strum(serialize = "A#")]
        Asharp,
        #[strum(serialize = "B")]
        B,
        #[strum(serialize = "C")]
        C,
        #[strum(serialize = "C#")]
        Csharp,
        #[strum(serialize = "D")]
        D,
        #[strum(serialize = "D#")]
        Dsharp,
        #[strum(serialize = "E")]
        E,
        #[strum(serialize = "F")]
        F,
        #[strum(serialize = "F#")]
        Fsharp,
        #[strum(serialize = "G")]
        G,
        #[strum(serialize = "G#")]
        Gsharp,
    }
    impl Display for NoteName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        let s = match self {
            NoteName::A => "A",
            NoteName::Asharp => "A#",
            NoteName::B => "B",
            NoteName::C => "C",
            NoteName::Csharp => "C#",
            NoteName::D => "D",
            NoteName::Dsharp => "D#",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::Fsharp => "F#",
            NoteName::G => "G",
            NoteName::Gsharp => "G#",
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}
    #[derive(EnumString,PartialEq, Clone, Debug)]
    enum TuningStyle{
    #[strum(serialize = "Open")]
    OPEN,
    #[strum(serialize = "Drop")]
    DROP,
    #[strum(serialize = "Standard")]
    STANDARD,
    #[strum(serialize = "Custom")]
    CUSTOM,
}
 #[derive(EnumString,PartialEq, Clone, Debug)]
enum TypeOfInstrument {
    #[strum(serialize = "Guitar")]
    GUITAR,
    #[strum(serialize = "Bass")]
    BASS,
    #[strum(serialize = "Mandolin")]
    MANDOLIN,
    #[strum(serialize = "Banjo")]
    BANJO,
    #[strum(serialize = "Ukelelle")]
    UKELELLE,
    #[strum(serialize = "Custom")]
    CUSTOM,
}

