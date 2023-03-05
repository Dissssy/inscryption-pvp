#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FixedOrVariable<T>
where
    T: Copy,
{
    Fixed(T),
    Variable(T),
}

impl<T: Copy> FixedOrVariable<T> {
    pub fn put(&mut self, value: T) {
        match self {
            Self::Fixed(_) => {}
            Self::Variable(v) => *v = value,
        }
    }
    pub fn get(&self) -> T {
        match self {
            Self::Fixed(v) => *v,
            Self::Variable(v) => *v,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Change {
    PercentageMax(f32),
    PercentageAmount(f32),
    Fixed(u8),
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Direction {
    Left,
    Right,
}

pub struct Range<T> {
    min: T,
    max: T,
}

impl<T: std::cmp::Ord> Range<T> {
    pub fn new(min: T, max: T) -> Result<Self, RangeError> {
        if min > max {
            Err(RangeError::MinGreaterThanMax)
        } else {
            Ok(Self { min, max })
        }
    }
    pub fn compare(&self, value: T) -> RangeComparison {
        if value < self.min {
            RangeComparison::LessThanMin
        } else if value > self.max {
            RangeComparison::GreaterThanMax
        } else {
            RangeComparison::WithinRange
        }
    }
}

#[derive(Debug)]
pub enum RangeError {
    MinGreaterThanMax,
}

pub enum RangeComparison {
    LessThanMin,
    GreaterThanMax,
    WithinRange,
}
