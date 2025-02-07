#![allow(unused)]

// Note frequencies in Hertz as u32
pub const NOTE_B0: u32 = 31;
pub const NOTE_C1: u32 = 33;
pub const NOTE_CS1: u32 = 35;
pub const NOTE_D1: u32 = 37;
pub const NOTE_DS1: u32 = 39;
pub const NOTE_E1: u32 = 41;
pub const NOTE_F1: u32 = 44;
pub const NOTE_FS1: u32 = 46;
pub const NOTE_G1: u32 = 49;
pub const NOTE_GS1: u32 = 52;
pub const NOTE_A1: u32 = 55;
pub const NOTE_AS1: u32 = 58;
pub const NOTE_B1: u32 = 62;
pub const NOTE_C2: u32 = 65;
pub const NOTE_CS2: u32 = 69;
pub const NOTE_D2: u32 = 73;
pub const NOTE_DS2: u32 = 78;
pub const NOTE_E2: u32 = 82;
pub const NOTE_F2: u32 = 87;
pub const NOTE_FS2: u32 = 93;
pub const NOTE_G2: u32 = 98;
pub const NOTE_GS2: u32 = 104;
pub const NOTE_A2: u32 = 110;
pub const NOTE_AS2: u32 = 117;
pub const NOTE_B2: u32 = 123;
pub const NOTE_C3: u32 = 131;
pub const NOTE_CS3: u32 = 139;
pub const NOTE_D3: u32 = 147;
pub const NOTE_DS3: u32 = 156;
pub const NOTE_E3: u32 = 165;
pub const NOTE_F3: u32 = 175;
pub const NOTE_FS3: u32 = 185;
pub const NOTE_G3: u32 = 196;
pub const NOTE_GS3: u32 = 208;
pub const NOTE_A3: u32 = 220;
pub const NOTE_AS3: u32 = 233;
pub const NOTE_B3: u32 = 247;
pub const NOTE_C4: u32 = 262;
pub const NOTE_CS4: u32 = 277;
pub const NOTE_D4: u32 = 294;
pub const NOTE_DS4: u32 = 311;
pub const NOTE_E4: u32 = 330;
pub const NOTE_F4: u32 = 349;
pub const NOTE_FS4: u32 = 370;
pub const NOTE_G4: u32 = 392;
pub const NOTE_GS4: u32 = 415;
pub const NOTE_A4: u32 = 440;
pub const NOTE_AS4: u32 = 466;
pub const NOTE_B4: u32 = 494;
pub const NOTE_C5: u32 = 523;
pub const NOTE_CS5: u32 = 554;
pub const NOTE_D5: u32 = 587;
pub const NOTE_DS5: u32 = 622;
pub const NOTE_E5: u32 = 659;
pub const NOTE_F5: u32 = 698;
pub const NOTE_FS5: u32 = 740;
pub const NOTE_G5: u32 = 784;
pub const NOTE_GS5: u32 = 831;
pub const NOTE_A5: u32 = 880;
pub const NOTE_AS5: u32 = 932;
pub const NOTE_B5: u32 = 988;
pub const NOTE_C6: u32 = 1047;
pub const NOTE_CS6: u32 = 1109;
pub const NOTE_D6: u32 = 1175;
pub const NOTE_DS6: u32 = 1245;
pub const NOTE_E6: u32 = 1319;
pub const NOTE_F6: u32 = 1397;
pub const NOTE_FS6: u32 = 1480;
pub const NOTE_G6: u32 = 1568;
pub const NOTE_GS6: u32 = 1661;
pub const NOTE_A6: u32 = 1760;
pub const NOTE_AS6: u32 = 1865;
pub const NOTE_B6: u32 = 1976;
pub const NOTE_C7: u32 = 2093;
pub const NOTE_CS7: u32 = 2217;
pub const NOTE_D7: u32 = 2349;
pub const NOTE_DS7: u32 = 2489;
pub const NOTE_E7: u32 = 2637;
pub const NOTE_F7: u32 = 2794;
pub const NOTE_FS7: u32 = 2960;
pub const NOTE_G7: u32 = 3136;
pub const NOTE_GS7: u32 = 3322;
pub const NOTE_A7: u32 = 3520;
pub const NOTE_AS7: u32 = 3729;
pub const NOTE_B7: u32 = 3951;
pub const NOTE_C8: u32 = 4186;
pub const NOTE_CS8: u32 = 4435;
pub const NOTE_D8: u32 = 4699;
pub const NOTE_DS8: u32 = 4978;
pub const REST: u32 = 0; // No sound, for pauses

pub struct Song {
    whole_note: u32,
}

impl Song {
    pub fn new(tempo: u16) -> Self {
        let whole_note = (60_000 * 4) / tempo as u32;
        Self { whole_note }
    }

    pub fn calc_note_duration(&self, divider: i16) -> u32 {
        if divider > 0 {
            self.whole_note / divider as u32
        } else {
            let duration = self.whole_note / divider.unsigned_abs() as u32;
            duration * 15 / 10
        }
    }
}
