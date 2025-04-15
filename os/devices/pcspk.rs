/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pcspk                                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Implementation for beep sound using the pc speaker. Works in    ║
   ║         qemu only if started with the correct audio settings.           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author:  Michael Schoettner, HHU, 22.9.2016                             ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![allow(dead_code)]

use crate::kernel::cpu;

// Ports
const PORT_CTRL: u16 = 0x43;
const PORT_DATA0: u16 = 0x40;
const PORT_DATA2: u16 = 0x42;
const PORT_PPI: u16 = 0x61;

// Frequency of musical notes
// (Our OS does not really support floating point. The numbers will be converted to u32 in 'play')
const C0: f32 = 130.81;
const C0X: f32 = 138.59;
const D0: f32 = 146.83;
const D0X: f32 = 155.56;
const E0: f32 = 164.81;
const F0: f32 = 174.61;
const F0X: f32 = 185.00;
const G0: f32 = 196.00;
const G0X: f32 = 207.65;
const A0: f32 = 220.00;
const A0X: f32 = 233.08;
const B0: f32 = 246.94;

const C1: f32 = 261.63;
const C1X: f32 = 277.18;
const D1: f32 = 293.66;
const D1X: f32 = 311.13;
const E1: f32 = 329.63;
const F1: f32 = 349.23;
const F1X: f32 = 369.99;
const G1: f32 = 391.00;
const G1X: f32 = 415.30;
const A1: f32 = 440.00;
const A1X: f32 = 466.16;
const B1: f32 = 493.88;

const C2: f32 = 523.25;
const C2X: f32 = 554.37;
const D2: f32 = 587.33;
const D2X: f32 = 622.25;
const E2: f32 = 659.26;
const F2: f32 = 698.46;
const F2X: f32 = 739.99;
const G2: f32 = 783.99;
const G2X: f32 = 830.61;
const A2: f32 = 880.00;
const A2X: f32 = 923.33;
const B2: f32 = 987.77;
const C3: f32 = 1046.50;

/**
 Description: Play musical note with given frequency for given time. \
              Then the pc speaker ist turned off.

 Parameters: \
            `f` frequency of musical note \
            `d` duration in ms
*/
pub fn play(f: f32, d: u32) {

    /* Hier muss Code eingefuegt werden */

}

/**
 Description: turns the speaker on
*/
pub fn speaker_on() {

    /* Hier muss Code eingefuegt werden */

}

/**
 Description: turns the speaker off
*/
pub fn speaker_off() {

    /* Hier muss Code eingefuegt werden */

}

/**
 Description: Helper function of `delay`. Returns the 16 bit counter value\
              of the counter 0 of the PIT.
*/
fn read_counter() -> u32 {

   /* Hier muss Code eingefuegt werden */

}

/**
 Description: Delay execution for given time in ms. \
              Minimum delay is 10ms. \
              For the implementation we use counter 0 of the PIT. \
              We might have to wait for several count downs to 0\
              because the counter has only 16 bit.

 Parameters: \
            `d` duration in ms
*/
fn delay(mut d: u32) {

    /* Hier muss Code eingefuegt werden */

}

/**
 Description: Tetris Sound, Kévin Rapaille, August 2013\
              https://gist.github.com/XeeX/6220067
*/
pub fn tetris() {
    play(658.0, 125);
    play(1320.0, 500);
    play(990.0, 250);
    play(1056.0, 250);
    play(1188.0, 250);
    play(1320.0, 125);
    play(1188.0, 125);
    play(1056.0, 250);
    play(990.0, 250);
    play(880.0, 500);
    play(880.0, 250);
    play(1056.0, 250);
    play(1320.0, 500);
    play(1188.0, 250);
    play(1056.0, 250);
    play(990.0, 750);
    play(1056.0, 250);
    play(1188.0, 500);
    play(1320.0, 500);
    play(1056.0, 500);
    play(880.0, 500);
    play(880.0, 500);
    delay(250);
    play(1188.0, 500);
    play(1408.0, 250);
    play(1760.0, 500);
    play(1584.0, 250);
    play(1408.0, 250);
    play(1320.0, 750);
    play(1056.0, 250);
    play(1320.0, 500);
    play(1188.0, 250);
    play(1056.0, 250);
    play(990.0, 500);
    play(990.0, 250);
    play(1056.0, 250);
    play(1188.0, 500);
    play(1320.0, 500);
    play(1056.0, 500);
    play(880.0, 500);
    play(880.0, 500);
    delay(500);
    play(1320.0, 500);
    play(990.0, 250);
    play(1056.0, 250);
    play(1188.0, 250);
    play(1320.0, 125);
    play(1188.0, 125);
    play(1056.0, 250);
    play(990.0, 250);
    play(880.0, 500);
    play(880.0, 250);
    play(1056.0, 250);
    play(1320.0, 500);
    play(1188.0, 250);
    play(1056.0, 250);
    play(990.0, 750);
    play(1056.0, 250);
    play(1188.0, 500);
    play(1320.0, 500);
    play(1056.0, 500);
    play(880.0, 500);
    play(880.0, 500);
    delay(250);
    play(1188.0, 500);
    play(1408.0, 250);
    play(1760.0, 500);
    play(1584.0, 250);
    play(1408.0, 250);
    play(1320.0, 750);
    play(1056.0, 250);
    play(1320.0, 500);
    play(1188.0, 250);
    play(1056.0, 250);
    play(990.0, 500);
    play(990.0, 250);
    play(1056.0, 250);
    play(1188.0, 500);
    play(1320.0, 500);
    play(1056.0, 500);
    play(880.0, 500);
    play(880.0, 500);
    delay(500);
    play(660.0, 1000);
    play(528.0, 1000);
    play(594.0, 1000);
    play(495.0, 1000);
    play(528.0, 1000);
    play(440.0, 1000);
    play(419.0, 1000);
    play(495.0, 1000);
    play(660.0, 1000);
    play(528.0, 1000);
    play(594.0, 1000);
    play(495.0, 1000);
    play(528.0, 500);
    play(660.0, 500);
    play(880.0, 1000);
    play(838.0, 2000);
    play(660.0, 1000);
    play(528.0, 1000);
    play(594.0, 1000);
    play(495.0, 1000);
    play(528.0, 1000);
    play(440.0, 1000);
    play(419.0, 1000);
    play(495.0, 1000);
    play(660.0, 1000);
    play(528.0, 1000);
    play(594.0, 1000);
    play(495.0, 1000);
    play(528.0, 500);
    play(660.0, 500);
    play(880.0, 1000);
    play(838.0, 2000);
    speaker_off();
}

/**
 Description: Clint, Part of Daft Punk’s Aerodynamic\
               https://www.kirrus.co.uk/2010/09/linux-beep-music
*/
pub fn aerodynamic() {
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(370.0, 122);
    play(493.9, 122);
    play(370.0, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(587.3, 122);
    play(415.3, 122);
    play(493.9, 122);
    play(415.3, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(784.0, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(493.9, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(440.0, 122);
    play(659.3, 122);
    play(440.0, 122);
    play(554.4, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(740.0, 122);
    play(987.8, 122);
    play(740.0, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1174.7, 122);
    play(830.6, 122);
    play(987.8, 122);
    play(830.6, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1568.0, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(987.8, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    play(1318.5, 122);
    play(880.0, 122);
    play(1108.7, 122);
    play(880.0, 122);
    speaker_off();
}
