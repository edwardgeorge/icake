pub enum MinMax {
    Missing,
    Present { min: usize, max: usize },
}

impl MinMax {
    pub fn new() -> Self {
        MinMax::Missing
    }
    pub fn difference(&self) -> Option<usize> {
        match self {
            MinMax::Missing => None,
            MinMax::Present { min, max } => Some(max - min),
        }
    }
}

impl From<usize> for MinMax {
    fn from(val: usize) -> MinMax {
        MinMax::Present { min: val, max: val }
    }
}

impl std::ops::Mul for MinMax {
    type Output = MinMax;
    fn mul(self, rhs: Self) -> MinMax {
        match (self, rhs) {
            (MinMax::Missing, r) => r,
            (l, MinMax::Missing) => l,
            (
                MinMax::Present {
                    min: lmin,
                    max: lmax,
                },
                MinMax::Present {
                    min: rmin,
                    max: rmax,
                },
            ) => MinMax::Present {
                min: std::cmp::min(lmin, rmin),
                max: std::cmp::max(lmax, rmax),
            },
        }
    }
}
