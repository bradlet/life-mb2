# life-mb2

## Conway's Game of Life player for BBC MicroBit V2

Bradley Thompson
CS 510 Rust Embedded - taught by Bart Massey
Winter 2024
Homework 2

## Write Up

This was a super fun assignment! I really appreciate that your classes are always
so entertaining; this was a super neat introduction (kinda reintroduction) into
embedded programming. I'd implemented a rudimentary x86 OS following Phil Opperrman's
OS blog ([my impl here](https://github.com/bradlet/thompson_rust_os)) but given
my lack of embedded experience, I think a lot of the... learning, was lost on me.
This is a much more pallatable introduction to the world of embedded.

I started out focusing on simply getting the display to work given the BSC.
After that I started working on the randomize functionality, and wanted to get into
building out a test module. I had forgotten that we are in a no_std environment, and
that makes things like mocking difficult... I wanted / maybe needed mocks to make a
testable wrapper for the HAL's RNG crate. With that realization, I decided
to just skip on unit testing for the sake of keeping it simple...

AND THEN I wanted to play around w/ Rust closures a bit more, so I over-engineered
the button-handling logic accordingly! I was pleased to see that the BSC provides
simple interfaces to interact w/ the button pins, so I didn't have to re-learn
interrupt handling from my OS impl.

Finishing up adding in the complement behavior -- the complement functionality was
easy enough; however, I was temporarily perplexed because the B button was behaving
differently from the A button. For whatever reason, after being pressed once,
`is_low()` was always returning `true`. I struggled with it for a bit before turning
to Google and finding the SO post that I link alongside my type alias for the B pin
(`BtnBPd` for "Button B Pressed Down"). I still find it strange that the board
configuration would matter as it seems like the BSC provides methods that can
coerce the Input into whatever type (Floating, PullUp, PullDown)... But I just
assumed that the issue lay in me incorrectly treating btn_b as a PullDown, regardless.
Changing it to `PullUp` fixed it, so I guess the actual hardware layout for that pin
is different from the A button. After getting that figured out, my impl for the 
assignment was finished.
