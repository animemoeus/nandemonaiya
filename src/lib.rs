#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

// Nada dasar (middle octave) - C4 to B4
const DO: u32 = 262;
const DO_SHARP: u32 = 277;
const RE: u32 = 294;
const RE_SHARP: u32 = 311;
const MI: u32 = 330;
const FA: u32 = 349;
const FA_SHARP: u32 = 370;
const SOL: u32 = 392;
const SOL_SHARP: u32 = 415;
const LA: u32 = 440;
const LA_SHARP: u32 = 466;
const SI: u32 = 494;

// Nada rendah (low octave) - C3 to B3
const DO_LOW: u32 = 131;
const DO_SHARP_LOW: u32 = 139;
const RE_LOW: u32 = 147;
const RE_SHARP_LOW: u32 = 156;
const MI_LOW: u32 = 165;
const FA_LOW: u32 = 175;
const FA_SHARP_LOW: u32 = 185;
const SOL_LOW: u32 = 196;
const SOL_SHARP_LOW: u32 = 208;
const LA_LOW: u32 = 220;
const LA_SHARP_LOW: u32 = 233;
const SI_LOW: u32 = 247;

// Nada tinggi (high octave) - C5 to B5
const DO_HIGH: u32 = 523;
const DO_SHARP_HIGH: u32 = 554;
const RE_HIGH: u32 = 587;
const RE_SHARP_HIGH: u32 = 622;
const MI_HIGH: u32 = 659;
const FA_HIGH: u32 = 698;
const FA_SHARP_HIGH: u32 = 740;
const SOL_HIGH: u32 = 784;
const SOL_SHARP_HIGH: u32 = 831;
const LA_HIGH: u32 = 880;
const LA_SHARP_HIGH: u32 = 932;
const SI_HIGH: u32 = 988;

// Nada sangat tinggi (higher octave) - C6 to B6
const DO_VERY_HIGH: u32 = 1047;
const RE_VERY_HIGH: u32 = 1175;
const MI_VERY_HIGH: u32 = 1319;
const FA_VERY_HIGH: u32 = 1397;
const SOL_VERY_HIGH: u32 = 1568;
const LA_VERY_HIGH: u32 = 1760;
const SI_VERY_HIGH: u32 = 1976;

const FPS: i32 = 60;
const TEMPO: i32 = 6;      // Base tempo unit
const TEMPO_SHORT: i32 = 4;  // Nada cepat
const TEMPO_LONG: i32 = 12;  // Nada panjang
const REST: u32 = 0;       // Jeda/istirahat

static mut TIME_FRAME: i32 = 0;
static mut CURRENT_NOTE: usize = 0;
static mut NOTE_TIMER: i32 = 0;

// Nandemonaiya - RADWIMPS
// Dengan timing yang lebih akurat dan jeda antar frase
const MELODY: [(u32, i32); 290] = [
    // Opening phrase 1: E°D°C°C° C°G°G°A°C°°B°A° C°D°E°
    (MI_HIGH, 10), (RE_HIGH, 10), (DO_HIGH, 10), (DO_HIGH, 30),
    (REST, 30),
    (DO_HIGH, 30), (SOL, TEMPO), (SOL, TEMPO), (LA, TEMPO),
    (DO_VERY_HIGH, TEMPO), (SI_HIGH, TEMPO), (LA_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (RE_HIGH, TEMPO), (MI_HIGH, TEMPO),
    (REST, TEMPO),

    // Phrase 2: E°D°C°C° C°G°E°D° C°C° C°D° C°C°
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO), (DO_HIGH, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (SOL, TEMPO), (MI_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO),

    // Phrase 3: E°D°C°C° C°G°G°A°C°°B°A° C°D°E°
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO), (DO_HIGH, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (SOL, TEMPO), (SOL, TEMPO), (LA, TEMPO),
    (DO_VERY_HIGH, TEMPO), (SI_HIGH, TEMPO), (LA_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (RE_HIGH, TEMPO), (MI_HIGH, TEMPO),
    (REST, TEMPO),

    // Phrase 4: E°D°C° C°G° E°D° C°C° C°D° C°C°
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (SOL, TEMPO),
    (REST, TEMPO_SHORT),
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO_LONG),

    // Phrase 5: E°D°C°C° C°G°G°A°C°°B°A° C°D°E°
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO), (DO_HIGH, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (SOL, TEMPO), (SOL, TEMPO), (LA, TEMPO),
    (DO_VERY_HIGH, TEMPO), (SI_HIGH, TEMPO), (LA_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (RE_HIGH, TEMPO), (MI_HIGH, TEMPO),
    (REST, TEMPO),

    // Phrase 6: E°D°C°C° C°G°E°D° C°C° C°D° C°C°
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO), (DO_HIGH, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (SOL, TEMPO), (MI_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO),

    // Phrase 7: E°D°C°C° C°G°G°A°C°°B°A° A°G°G°
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO), (DO_HIGH, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (SOL, TEMPO), (SOL, TEMPO), (LA, TEMPO),
    (DO_VERY_HIGH, TEMPO), (SI_HIGH, TEMPO), (LA_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (LA_HIGH, TEMPO_LONG), (SOL_HIGH, TEMPO), (SOL_HIGH, TEMPO),
    (REST, TEMPO),

    // Phrase 8: C°D°E° E°G°E°D° C°C° C°DE°C°
    (DO_HIGH, TEMPO), (RE_HIGH, TEMPO), (MI_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (MI_HIGH, TEMPO), (SOL_HIGH, TEMPO), (MI_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (RE_HIGH, TEMPO), (MI_HIGH, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO),

    // Phrase 9: C°C°G° E°D° C°C° AC° G°
    (DO_HIGH, TEMPO), (DO_HIGH, TEMPO), (SOL, TEMPO),
    (REST, TEMPO_SHORT),
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (LA_LOW, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (SOL, TEMPO_LONG),
    (REST, TEMPO),

    // Phrase 10: E°D° C°E° C°C°G° E°D° C°C° AC°
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (MI_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (DO_HIGH, TEMPO), (SOL, TEMPO),
    (REST, TEMPO_SHORT),
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (LA_LOW, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO),

    // Phrase 11: C°C°G° E°D° C°C° AC°
    (DO_HIGH, TEMPO), (DO_HIGH, TEMPO), (SOL, TEMPO),
    (REST, TEMPO_SHORT),
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (LA_LOW, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO),

    // Phrase 12: C°°B°A° G°G° C°C°G° E°D° C°C°
    (DO_VERY_HIGH, TEMPO), (SI_HIGH, TEMPO), (LA_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (SOL_HIGH, TEMPO_LONG), (SOL_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO), (DO_HIGH, TEMPO), (SOL, TEMPO),
    (REST, TEMPO_SHORT),
    (MI_HIGH, TEMPO), (RE_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_HIGH, TEMPO_LONG), (DO_HIGH, TEMPO),
    (REST, TEMPO_LONG),

    // Chorus part: C°E°D°C° G°A°C°°C°°
    (DO_HIGH, TEMPO), (MI_HIGH, TEMPO), (RE_HIGH, TEMPO), (DO_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (SOL, TEMPO), (LA, TEMPO), (DO_VERY_HIGH, TEMPO_LONG), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO_SHORT),

    // C°°D°°C°°C°° G°A°C°° G°A°C°°
    (DO_VERY_HIGH, TEMPO), (RE_VERY_HIGH, TEMPO), (DO_VERY_HIGH, TEMPO), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (SOL, TEMPO), (LA, TEMPO), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (SOL, TEMPO), (LA, TEMPO), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO_SHORT),

    // C°°C°°D°°C°° G°A°C°° G°A°C°°
    (DO_VERY_HIGH, TEMPO), (DO_VERY_HIGH, TEMPO), (RE_VERY_HIGH, TEMPO), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (SOL, TEMPO), (LA, TEMPO), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (SOL, TEMPO), (LA, TEMPO), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO_SHORT),

    // Bridge pattern
    (SOL, TEMPO), (SOL, TEMPO), (LA, TEMPO), (DO_VERY_HIGH, TEMPO_LONG),
    (REST, TEMPO_SHORT),
    (SOL, TEMPO_LONG),
    (REST, TEMPO_SHORT),
    (DO_VERY_HIGH, TEMPO_LONG),
    (REST, TEMPO),

    // Pattern continues
    (SOL, TEMPO), (LA, TEMPO), (MI_VERY_HIGH, TEMPO), (RE_VERY_HIGH, TEMPO), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (SOL, TEMPO), (LA, TEMPO), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_VERY_HIGH, TEMPO), (RE_VERY_HIGH, TEMPO), (DO_VERY_HIGH, TEMPO), (DO_VERY_HIGH, TEMPO),
    (REST, TEMPO),

    // Ending pattern with C°°A°A°G°G°
    (DO_VERY_HIGH, TEMPO), (LA_HIGH, TEMPO), (LA_HIGH, TEMPO), (SOL_HIGH, TEMPO), (SOL_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_VERY_HIGH, TEMPO), (LA_HIGH, TEMPO), (LA_HIGH, TEMPO), (SOL_HIGH, TEMPO), (SOL_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (DO_VERY_HIGH, TEMPO), (LA_HIGH, TEMPO), (LA_HIGH, TEMPO), (SOL_HIGH, TEMPO), (SOL_HIGH, TEMPO),
    (REST, TEMPO_SHORT),
    (MI_HIGH, TEMPO), (MI_HIGH, TEMPO), (SOL_HIGH, TEMPO), (LA_HIGH, TEMPO), (SOL_HIGH, TEMPO_LONG),
    (REST, TEMPO_LONG),
];

#[no_mangle]
unsafe fn update() {
    unsafe { *DRAW_COLORS = 2 }
    text("Nandemonaiya", 10, 10);
    text("RADWIMPS", 10, 20);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
        TIME_FRAME = 0;
        CURRENT_NOTE = 0;
        NOTE_TIMER = 0;
    }

    blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);

    let progress_text = format!("Note: {}/{}", CURRENT_NOTE + 1, MELODY.len());
    text(&progress_text, 10, 100);

    text("Press X to restart", 10, 120);

    play_music_sequence();

    TIME_FRAME += 1;
}

unsafe fn play_music_sequence() {
    if CURRENT_NOTE < MELODY.len() {
        let (freq, duration) = MELODY[CURRENT_NOTE];

        if NOTE_TIMER == 0 {
            // Hanya mainkan tone kalau bukan REST
            if freq > 0 {
                tone(
                    freq,
                    (duration * 1000 / FPS) as u32,
                    100,  // Volume penuh
                    TONE_PULSE1 | TONE_MODE1
                );
            }
        }

        NOTE_TIMER += 1;

        if NOTE_TIMER >= duration {
            CURRENT_NOTE += 1;
            NOTE_TIMER = 0;
        }
    } else {
        // Loop dari awal
        CURRENT_NOTE = 0;
        NOTE_TIMER = 0;
    }
}