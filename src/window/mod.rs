pub mod functions;
pub mod gameloop;

const SIZE: f64 = 500.0;
const MARGIN: f64 = 10.0;
const FONT_SIZE: f64 = 120.0;
const BOX_PLACEMENT: [[f64; 2]; 9] = [
    [SIZE/6. + FONT_SIZE/3.,    SIZE/6. - FONT_SIZE/3.],
    [SIZE/2. + FONT_SIZE/3.,    SIZE/6. - FONT_SIZE/3.],
    [5.*SIZE/6. + FONT_SIZE/3., SIZE/6. - FONT_SIZE/3.],
    [SIZE/6. + FONT_SIZE/3.,    SIZE/2. - FONT_SIZE/3.],
    [SIZE/2. + FONT_SIZE/3.,    SIZE/2. - FONT_SIZE/3.],
    [5.*SIZE/6. + FONT_SIZE/3., SIZE/2. - FONT_SIZE/3.],
    [SIZE/6. + FONT_SIZE/3.,    5.*SIZE/6. - FONT_SIZE/3.],
    [SIZE/2. + FONT_SIZE/3.,    5.*SIZE/6. - FONT_SIZE/3.],
    [5.*SIZE/6. + FONT_SIZE/3., 5.*SIZE/6. - FONT_SIZE/3.],
];
