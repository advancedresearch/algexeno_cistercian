#![deny(missing_docs)]

//! # Algexeno-Cistercian
//! Visual Algexenotation with Cistercian numerals as hyperprimes

use algexenotation::Algexeno;

/// Settings for converting drawing shape into strokes.
pub struct Settings {
    /// The resolution of circle symbols.
    pub circle_resolution: u32,
}

/// Represents a drawing shape.
pub enum Draw {
    /// Draw a circle.
    Circle {
        /// Center point of the circle.
        center: [f64; 2],
        /// Radius of the circle.
        radius: f64
    },
    /// Draw a line.
    Line {
        /// The point to draw a line from.
        from: [f64; 2],
        /// The point to draw a line to.
        to: [f64; 2]
    },
    /// Draw a sequence of shapes.
    Seq(Vec<Draw>),
    /// Mirror up and down, with an optional separator line.
    ///
    /// The separator is added automatically if the
    /// text moves to the right to make room for symbols.
    MirrorV(bool, Box<(Draw, Draw)>),
    /// Chain commands (uses a circle as vertical separator).
    Chain(Box<(Draw, Draw)>),
    /// Quote commands (uses small lines to signify start and end).
    Quote(Box<(Draw, Draw)>),
}

const TAU: f64 = 6.283185307179586;

impl Draw {
    /// Converts from Algexeno number to itself.
    pub fn from_algexeno(alg: &Algexeno) -> Result<Self, ()> {
        use Draw::*;
        use Algexeno::*;
        use algexenotation::Op::*;

        match alg {
            Orig(0) => Ok(Circle {center: [0.5, 0.5], radius: 0.5}),
            Orig(1) => Ok(Seq(vec![
                Draw::from_algexeno(&Orig(0))?,
                Line {from: [0.5, 0.0], to: [0.5, 0.5]},
            ])),
            Const(0) => Ok(Line {from: [0.5, 1.0], to: [0.5, 0.0]}),
            // 3'.
            Const(1) => Ok(Seq(vec![
                Line {from: [0.5, 1.0], to: [0.5, 0.0]},
                Line {from: [0.5, 0.0], to: [0.75, 0.0]},
            ])),
            // 5'.
            Const(2) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.25], to: [0.75, 0.25]},
            ])),
            // 11'.
            Const(3) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.0], to: [0.75, 0.25]},
            ])),
            // 31'.
            Const(4) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.25], to: [0.75, 0.0]},
            ])),
            // 127'.
            Const(5) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.0], to: [0.75, 0.0]},
                Line {from: [0.75, 0.0], to: [0.5, 0.25]},
            ])),
            // 709'.
            Const(6) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.75, 0.0], to: [0.75, 0.25]},
            ])),
            // 5381'.
            Const(7) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.0], to: [0.75, 0.0]},
                Line {from: [0.75, 0.0], to: [0.75, 0.25]},
            ])),
            // 52711'.
            Const(8) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.25], to: [0.75, 0.25]},
                Line {from: [0.75, 0.25], to: [0.75, 0.0]},
            ])),
            // 648391'.
            Const(9) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.0], to: [0.75, 0.0]},
                Line {from: [0.75, 0.0], to: [0.75, 0.25]},
                Line {from: [0.75, 0.25], to: [0.5, 0.25]},
            ])),
            // 9737333'.
            Const(10) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.0], to: [0.25, 0.0]},
            ])),
            // 174440041'.
            Const(11) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.25], to: [0.25, 0.25]},
            ])),
            // 3657500101'.
            Const(12) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.0], to: [0.25, 0.25]},
            ])),
            // 88362852307'.
            Const(13) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.25], to: [0.25, 0.0]},
            ])),
            // 2428095424619'.
            Const(14) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.0], to: [0.25, 0.0]},
                Line {from: [0.25, 0.0], to: [0.5, 0.25]},
            ])),
            // 75063692618249'.
            Const(15) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.25, 0.0], to: [0.25, 0.25]},
            ])),
            // 2586559730396077'.
            Const(16) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.0], to: [0.25, 0.0]},
                Line {from: [0.25, 0.0], to: [0.25, 0.25]},
            ])),
            // 98552043847093519'.
            Const(17) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.25], to: [0.25, 0.25]},
                Line {from: [0.25, 0.25], to: [0.25, 0.0]},
            ])),
            // 4123221751654370051'.
            Const(18) => Ok(Seq(vec![
                Draw::from_algexeno(&Const(0))?,
                Line {from: [0.5, 0.0], to: [0.25, 0.0]},
                Line {from: [0.25, 0.0], to: [0.25, 0.25]},
                Line {from: [0.25, 0.25], to: [0.5, 0.25]},
            ])),
            Bin(op, ab) => {
                match op {
                    Pow => {
                        let sep = match &**ab {
                            (Const(0), _) | (_, Const(0)) => true,
                            _ => false
                        };
                        Ok(MirrorV(sep, Box::new((
                            Draw::from_algexeno(&ab.0)?,
                            Draw::from_algexeno(&ab.1)?,
                        ))))
                    }
                    Mul => {
                        Ok(Chain(Box::new((
                            Draw::from_algexeno(&ab.0)?,
                            Draw::from_algexeno(&ab.1)?,
                        ))))
                    }
                    Add => {
                        Ok(Quote(Box::new((
                            Draw::from_algexeno(&ab.0)?,
                            Draw::from_algexeno(&ab.1)?,
                        ))))
                    }
                }
            }
            _ => Err(()),
        }
    }

    /// Converts drawing shape to strokes.
    ///
    /// Returns a list of lines to stroke with time,
    /// such that the rendering engine can simulate writing of the symbols.
    pub fn to_strokes(&self, scale: bool, strokes: &mut Vec<([f64; 2], [f64; 2], f64)>, settings: &Settings) -> f64 {
        use Draw::*;

        match self {
            Line {from, to} => {
                strokes.push((*from, *to, 1.0));
                0.0
            }
            Circle {center, radius} => {
                let n = settings.circle_resolution as f64;
                for i in 0..settings.circle_resolution {
                    let rad = (i as f64 / n) * TAU - TAU / 4.0;
                    let nrad = (i + 1) as f64 / n * TAU - TAU / 4.0;
                    let x = rad.cos() * radius + center[0];
                    let y = rad.sin() * radius + center[1];
                    let nx = nrad.cos() * radius + center[0];
                    let ny = nrad.sin() * radius + center[1];
                    strokes.push(([x, y], [nx, ny], 1.0 / n));
                }
                0.0
            }
            Seq(seq) => {
                for s in seq {
                    s.to_strokes(scale, strokes, settings);
                }
                0.0
            }
            MirrorV(sep, ab) => {
                let mut a_strokes: Vec<([f64; 2], [f64; 2], f64)> = vec![];
                let av = ab.1.to_strokes(scale, &mut a_strokes, settings);
                let mut b_strokes: Vec<([f64; 2], [f64; 2], f64)> = vec![];
                let bv = ab.0.to_strokes(scale, &mut b_strokes, settings);
                let av = av * 0.75;
                let bv = bv * 0.75;
                let mv = av.max(bv);
                let sep = *sep || (mv != 0.0);
                if sep {
                    strokes.push(([0.1, 0.5], [mv + 0.9, 0.5], 1.0));
                }
                for (x, y, t) in &a_strokes {
                    let x = if sep {
                        [(mv - av) * 0.5 + 0.5 + (x[0] - 0.5) * 0.75, x[1] * 0.45]
                    } else {
                        [(mv - av) * 0.5 + 0.5 + (x[0] - 0.5) * 0.75, x[1] * 0.5]
                    };
                    let y = if sep {
                        [(mv - av) * 0.5 + 0.5 + (y[0] - 0.5) * 0.75, y[1] * 0.45]
                    } else {
                        [(mv - av) * 0.5 + 0.5 + (y[0] - 0.5) * 0.75, y[1] * 0.5]
                    };
                    strokes.push((x, y, *t));
                }
                for (x, y, t) in &b_strokes {
                    let x = if sep {
                        [(mv - bv) * 0.5 + 0.5 + (x[0] - 0.5) * 0.75, 0.55 + (1.0 - x[1]) * 0.45]
                    } else {
                        [(mv - bv) * 0.5 + 0.5 + (x[0] - 0.5) * 0.75, 0.5 + (1.0 - x[1]) * 0.5]
                    };
                    let y = if sep {
                        [(mv - bv) * 0.5 + 0.5 + (y[0] - 0.5) * 0.75, 0.55 + (1.0 - y[1]) * 0.45]
                    } else {
                        [(mv - bv) * 0.5 + 0.5 + (y[0] - 0.5) * 0.75, 0.5 + (1.0 - y[1]) * 0.5]
                    };
                    strokes.push((x, y, *t));
                }
                mv
            }
            Chain(ab) => {
                let mut a_strokes: Vec<([f64; 2], [f64; 2], f64)> = vec![];
                let av = ab.0.to_strokes(scale, &mut a_strokes, settings);
                let mut b_strokes: Vec<([f64; 2], [f64; 2], f64)> = vec![];
                let bv = ab.1.to_strokes(scale, &mut b_strokes, settings);
                for (x, y, t) in &a_strokes {
                    let x2 = if scale {
                        [x[0] * 0.5, x[1]]
                    } else {
                        [x[0], x[1]]
                    };
                    let y2 = if scale {
                        [y[0] * 0.5, y[1]]
                    } else {
                        [y[0], y[1]]
                    };
                    strokes.push((x2, y2, *t));
                }
                if scale {
                    Circle {center: [0.5, 0.5], radius: 0.1}
                } else {
                    Circle {center: [av + 1.0, 0.5], radius: 0.1}
                }.to_strokes(scale, strokes, settings);
                for (x, y, t) in &b_strokes {
                    let x2 = if scale {
                        [0.5 + x[0] * 0.5, x[1]]
                    } else {
                        [av + 1.0 + x[0], x[1]]
                    };
                    let y2 = if scale {
                        [0.5 + y[0] * 0.5, y[1]]
                    } else {
                        [av + 1.0 + y[0], y[1]]
                    };
                    strokes.push((x2, y2, *t));
                }
                if scale {0.0} else {av + bv + 1.0}
            }
            Quote(ab) => {
                let mut a_strokes: Vec<([f64; 2], [f64; 2], f64)> = vec![];
                let av = ab.0.to_strokes(scale, &mut a_strokes, settings);
                let mut b_strokes: Vec<([f64; 2], [f64; 2], f64)> = vec![];
                let bv = ab.1.to_strokes(scale, &mut b_strokes, settings);
                if scale {
                    strokes.push(([0.0, 0.95], [0.0, 1.0], 1.0));
                } else {
                    strokes.push(([0.0, 0.95], [0.0, 1.0], 1.0));
                }
                for (x, y, t) in &a_strokes {
                    let x2 = if scale {
                        [x[0] * 0.5, x[1]]
                    } else {
                        [x[0], x[1]]
                    };
                    let y2 = if scale {
                        [y[0] * 0.5, y[1]]
                    } else {
                        [y[0], y[1]]
                    };
                    strokes.push((x2, y2, *t));
                }
                for (x, y, t) in &b_strokes {
                    let x2 = if scale {
                        [0.5 + x[0] * 0.5, x[1]]
                    } else {
                        [1.0 + x[0], x[1]]
                    };
                    let y2 = if scale {
                        [0.5 + y[0] * 0.5, y[1]]
                    } else {
                        [1.0 + y[0], y[1]]
                    };
                    strokes.push((x2, y2, *t));
                }
                if scale {
                    strokes.push(([1.0, 0.0], [1.0, 0.05], 1.0));
                } else {
                    strokes.push(([av + bv + 2.0, 0.0], [av + bv + 2.0, 0.05], 1.0));
                }
                if scale {0.0} else {av + bv + 1.1}
            }
        }
    }
}

#[cfg(test)]
mod tests {
}
