
use strum_macros::EnumString;
use core::fmt::{Result, Display, Formatter};

fn main() {
    let i : Instrument = Instrument::new(TypeOfInstrument::GUITAR, TuningStyle::STANDARD, MusicalNote::new("E"), 6 ,25);
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
                musical_string.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[i],j.try_into().unwrap())));
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
                self.tuning.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[1], 5)));
                self.tuning.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[2], 5)));
                self.tuning.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[3], 4)));
                self.tuning.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[4], 5)));
                }
            TuningStyle::DROP => {
                self.tuning.push(MusicalNote::find_note(&self.tuning[0], 7));
                self.tuning.push(MusicalNote::find_note(&self.tuning[1], 5));
                self.tuning.push(MusicalNote::find_note(&self.tuning[2], 5));
                self.tuning.push(MusicalNote::find_note(&self.tuning[3], 4));
                self.tuning.push(MusicalNote::find_note(&self.tuning[4], 5));
                }
            TuningStyle::OPEN  => {
                self.tuning.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[0], 7)));
                self.tuning.push(self.tuning[0].clone());
                self.tuning.push(MusicalNote::number_to_note(
                    MusicalNote::add(&self.tuning[0], 16)));
                self.tuning.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[3], 3)));
                self.tuning.push(self.tuning[0].clone());
                }
                _ => todo!(),
            }

        }
        TypeOfInstrument::BASS=>{
            match self.tuning_style {
                TuningStyle::STANDARD=>{
                    self.tuning.push(MusicalNote::find_note(&self.tuning[0], 5));
                    self.tuning.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[1], 5)));
                    self.tuning.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[2], 5)));
                    }
                TuningStyle::DROP => {
                    self.tuning.push(MusicalNote::find_note(&self.tuning[0], 7));
                    self.tuning.push(MusicalNote::find_note(&self.tuning[1], 5));
                    self.tuning.push(MusicalNote::find_note(&self.tuning[2], 5));
                    }
                TuningStyle::OPEN => {
                    self.tuning.push(MusicalNote::number_to_note(MusicalNote::add(&self.tuning[0], 7)));
                    self.tuning.push(self.tuning[0].clone());
                    self.tuning.push(MusicalNote::number_to_note(
                        MusicalNote::add(&self.tuning[0], 16)));
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
}

impl Display for MusicalNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
    write!(f,"{}", self.name)?;
    Ok(())
    }
}

impl MusicalNote{
    fn new(name:&str) -> Self{
        match name{
        "A"=> MusicalNote{name : NoteName::A},
        "Asharp" => MusicalNote{name : NoteName::Asharp},
        "B" => MusicalNote{name : NoteName::B},
        "C" => MusicalNote{name : NoteName::C},
        "Csharp" => MusicalNote{name : NoteName::Csharp},
        "D" => MusicalNote{name : NoteName::D},
        "Dsharp" => MusicalNote{name : NoteName::Dsharp},
        "E" => MusicalNote{name : NoteName::E},
        "F" => MusicalNote{name : NoteName::F},
        "Fsharp" => MusicalNote{name : NoteName::Fsharp},
        "G" => MusicalNote{name : NoteName::G},
        "Gsharp" => MusicalNote{name : NoteName::Gsharp},
        _ => MusicalNote{name : NoteName::C},
      }  
    }
    
fn number_to_note(number: u8) -> MusicalNote{
    match number %12 {
       
        0 => MusicalNote{name : NoteName::A},
        1 => MusicalNote{name : NoteName::Asharp},
        2 => MusicalNote{name : NoteName::B},
        3 => MusicalNote{name : NoteName::C},
        4 => MusicalNote{name : NoteName::Csharp},
        5 => MusicalNote{name : NoteName::D},
        6 => MusicalNote{name : NoteName::Dsharp},
        7 => MusicalNote{name : NoteName::E},
        8 => MusicalNote{name : NoteName::F},
        9 => MusicalNote{name : NoteName::Fsharp},
        10 => MusicalNote{name : NoteName::G},
        11 => MusicalNote{name : NoteName::Gsharp},
        _ => MusicalNote{name : NoteName::A},
        }
    }
 fn note_to_number(note:&MusicalNote) -> u8{
    match note.name {
        NoteName::A => 0,
        NoteName::Asharp => 1,
        NoteName::B => 2,
        NoteName::C => 3,
        NoteName::Csharp => 4,
        NoteName::D => 5,
        NoteName::Dsharp => 6,
        NoteName::E => 7,
        NoteName::F => 8,
        NoteName::Fsharp => 9,
        NoteName::G => 10,
        NoteName::Gsharp => 11,
    }
}
  fn find_note(open_note : &MusicalNote, fret: u8) -> MusicalNote{
   MusicalNote::number_to_note(MusicalNote::add(&open_note, fret))
 }
fn add(note: &MusicalNote, fret: u8) -> u8 {
    let n = MusicalNote::note_to_number(&note);
      if n + fret % 12 != 0 {
            return n + fret;
        } 
        else {
            return n;
        }
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

