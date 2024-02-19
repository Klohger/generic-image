#![feature(new_uninit)]

use generic_image::Image;
use std::{
    fs::File,
    io::{BufWriter, Read},
    mem::size_of,
};
/*
#[repr(u16)]
#[derive(Clone, Copy)]
enum CubeEdge {
    BottomBack,
    BottomRight,
    BottomFront,
    BottomLeft,
    MiddleBackLeft,
    MiddleBackRight,
    MiddleFrontLeft,
    MiddleFrontRight,
    TopBack,
    TopRight,
    TopFront,
    TopLeft,
    Cull,
}

const CULL: [CubeEdge; 3] = [CubeEdge::Cull; 3];
///
///         P6______E10_____P7
///        /|               /|
///     E11 |            E09 |
///      /  |             /  |
///     P4__|_E08_______P5   |
///     |   |            |   |
///     | E06            | E07
///    E05  |          E05   |
///     |   P2____E02____|__P3
///     |   /            |  /
///     |E03             |E01
///     | /              |/
///    P0______E00_____P1
///

const ARR: [[[CubeEdge; 3]; 4]; 256] = [
    [
        // 0b00000000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00000001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00000010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00000011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00000100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00000101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00000110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00000111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00001000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00001001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00001010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00001011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00001100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00001101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00001110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00001111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00010000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00010001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00010010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00010011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00010100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00010101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00010110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00010111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00011000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00011001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00011010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00011011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00011100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00011101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00011110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00011111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00100000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00100001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00100010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00100011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00100100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00100101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00100110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00100111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00101000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00101001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00101010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00101011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00101100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00101101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00101110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00101111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00110000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00110001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00110010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00110011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00110100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00110101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00110110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00110111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00111000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00111001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00111010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00111011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00111100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00111101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00111110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b00111111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01000000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01000001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01000010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01000011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01000100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01000101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01000110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01000111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01001000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01001001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01001010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01001011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01001100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01001101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01001110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01001111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01010000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01010001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01010010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01010011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01010100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01010101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01010110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01010111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01011000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01011001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01011010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01011011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01011100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01011101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01011110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01011111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01100000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01100001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01100010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01100011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01100100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01100101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01100110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01100111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01101000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01101001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01101010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01101011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01101100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01101101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01101110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01101111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01110000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01110001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01110010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01110011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01110100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01110101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01110110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01110111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01111000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01111001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01111010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01111011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01111100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01111101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01111110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b01111111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10000000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10000001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10000010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10000011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10000100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10000101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10000110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10000111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10001000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10001001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10001010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10001011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10001100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10001101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10001110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10001111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10010000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10010001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10010010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10010011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10010100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10010101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10010110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10010111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10011000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10011001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10011010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10011011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10011100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10011101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10011110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10011111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10100000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10100001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10100010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10100011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10100100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10100101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10100110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10100111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10101000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10101001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10101010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10101011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10101100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10101101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10101110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10101111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10110000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10110001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10110010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10110011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10110100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10110101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10110110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10110111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10111000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10111001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10111010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10111011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10111100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10111101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10111110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b10111111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11000000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11000001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11000010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11000011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11000100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11000101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11000110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11000111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11001000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11001001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11001010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11001011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11001100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11001101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11001110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11001111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11010000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11010001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11010010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11010011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11010100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11010101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11010110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11010111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11011000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11011001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11011010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11011011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11011100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11011101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11011110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11011111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11100000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11100001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11100010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11100011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11100100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11100101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11100110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11100111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11101000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11101001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11101010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11101011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11101100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11101101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11101110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11101111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11110000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11110001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11110010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11110011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11110100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11110101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11110110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11110111
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11111000
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11111001
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11111010
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11111011
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11111100
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11111101
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11111110
        CULL, CULL, CULL, CULL,
    ],
    [
        // 0b11111111
        CULL, CULL, CULL, CULL,
    ],
];
*/
fn main() {
    /*
    for i in 0..256 {
        println!("[ // 0b{i:08b}\nCULL,\nCULL,\nCULL,\nCULL,\n],")
    }
    */

    let mut canvas = unsafe { Image::uninit(16, 16) };

    canvas.fill([255u8, 0, 0]);
File::
    canvas[(1, 0)] = [0, 0, 0];
    canvas[(0, 1)][1] = 255;
    let region = canvas.region((0, 0)..(2, 2)).unwrap();
    {
        let mut encoder = png::Encoder::new(
            BufWriter::new(File::create("test.png").unwrap()),
            region.width() as u32,
            region.height() as u32,
        );
        let mut buf = unsafe {
            Box::new_uninit_slice(region.width() * region.height() * size_of::<[u8; 3]>())
                .assume_init()
        };
        region.cursor().read_exact(&mut buf).unwrap();

        encoder.set_color(png::ColorType::Rgb);
        encoder.set_compression(png::Compression::Best);
        encoder.set_depth(png::BitDepth::Eight);
        let mut encoder = encoder.write_header().unwrap();

        region.cursor().read_exact(&mut buf).unwrap();

        encoder.write_image_data(&buf).unwrap();
    }
    {}
}
