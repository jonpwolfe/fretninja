
use strum_macros::EnumString;
fn main() {
    println!("Hello, world!");
}
struct Instrument{
    type_of_instrument : TypeOfInstrument,
    tuning_style : TuningStyle,
    root_note : MusicalNote,
    string_count : u8,
    fret_count : u8,
    tuning : Vec<MusicalNote>,
    notes : Vec<Vec<MusicalNote>>
}
impl Instrument {
    fn new(type_of_instrument: TypeOfInstrument, tuning_style :TuningStyle, root_note :MusicalNote, string_count:u8, fret_count : u8) ->Self{
        
        
    let mut instrument = Instrument{
        type_of_instrument ,
        tuning_style : tuning_style.clone(),
        root_note : root_note.clone(),
        string_count,
        fret_count,
        tuning: Vec::new(),      
        notes: Vec::new(),
        };
    Instrument::calculate_tuning(&mut instrument);
    Instrument::get_notes(&mut instrument);
    instrument
    }
   fn get_notes(instrument : &mut Instrument) {
        let mut musical_string: Vec<MusicalNote> = Vec::new();
        let mut notes :Vec<Vec<MusicalNote>> = Vec::new();
        for i in 0..instrument.string_count as usize{
            for j in 0..instrument.fret_count{
                musical_string.push(MusicalNote::number_to_note(MusicalNote::add(&instrument.tuning[i],j)));
           }
         notes.push(musical_string.clone());
         
        }
        instrument.notes = notes;
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
#[derive(Clone,Debug)]
struct MusicalNote{
    note_name : NoteName,
}
impl MusicalNote{
    fn generate_note() -> Self{
        MusicalNote{note_name : NoteName::G}
    }
    
fn number_to_note(number: u8) -> MusicalNote{
    match number {
       
        1=> MusicalNote{note_name : NoteName::A},
        2=> MusicalNote{note_name : NoteName::ASharp},
        3=> MusicalNote{note_name : NoteName::B},
        4=> MusicalNote{note_name : NoteName::C},
        5=> MusicalNote{note_name : NoteName::CSharp},
        6=> MusicalNote{note_name : NoteName::D},
        7=> MusicalNote{note_name : NoteName::DSharp},
        8=> MusicalNote{note_name : NoteName::E},
        9=> MusicalNote{note_name : NoteName::F},
        10=> MusicalNote{note_name : NoteName::FSharp},
        11=> MusicalNote{note_name : NoteName::G},
        12=> MusicalNote{note_name : NoteName::GSharp},
        _ => MusicalNote{note_name : NoteName::A},
    }
    }
 fn note_to_number(note:&MusicalNote) -> u8{
    match note.note_name {
               NoteName::A=>1,
        NoteName::ASharp=>2,
        NoteName::B=>3,
         NoteName::C=>4,
        NoteName::CSharp=>5,
        NoteName::D=>6,
        NoteName::DSharp=>7,
         NoteName::E=>8,
         NoteName::F=>9,
         NoteName::FSharp=>10,
          NoteName::G => 11,
        NoteName::GSharp=>12,
    }
}
  fn find_note(open_note : &MusicalNote, fret: u8) -> MusicalNote{
   let a = MusicalNote::number_to_note(MusicalNote::add(&open_note, fret));
    a
 }
fn add(note: &MusicalNote, fret: u8) -> u8 {
    if MusicalNote::note_to_number(&note) + fret == 36 {
        return MusicalNote::note_to_number(&note);
    } else if fret == 0 {
        return MusicalNote::note_to_number(&note);
    } else if MusicalNote::note_to_number(&note) + fret <= 12 {
        return MusicalNote::note_to_number(&note) + fret;
    } else if (MusicalNote::note_to_number(&note) + fret) % 12 != 0 {
        return (MusicalNote::note_to_number(&note) + fret) % 12;
    } else if MusicalNote::note_to_number(&note) + fret == 24 {
        return MusicalNote::note_to_number(&note) - 12;
    }

    // You might want to handle other cases or add a default return value here
    panic!("Unhandled case");
}
 }
  #[derive(EnumString,PartialEq, Clone, Debug)]
    pub enum NoteName {
        #[strum(serialize = "A")]
        A,
        #[strum(serialize = "A#")]
        ASharp,
        #[strum(serialize = "B")]
        B,
        #[strum(serialize = "C")]
        C,
        #[strum(serialize = "C#")]
        CSharp,
        #[strum(serialize = "D")]
        D,
        #[strum(serialize = "D#")]
        DSharp,
        #[strum(serialize = "E")]
        E,
        #[strum(serialize = "F")]
        F,
        #[strum(serialize = "F#")]
        FSharp,
        #[strum(serialize = "G")]
        G,
        #[strum(serialize = "G#")]
        GSharp,
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

