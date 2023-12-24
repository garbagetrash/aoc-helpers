use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Interval of integers [min...max] inclusive.
pub struct Interval {
    min: i64,
    max: i64,
}

impl Interval {
    pub fn new(min: i64, max: i64) -> Self {
        if max < min {
            panic!("Can't create an Interval with max < min");
        }
        Self { min, max }
    }

    pub fn len(&self) -> usize {
        (self.max - self.min + 1) as usize
    }

    pub fn union(&self, other: Interval) -> Vec<Interval> {
        if self.min > other.max || self.max < other.min {
            // Disjoint intervals
            return vec![*self, other];
        } else {
            // Overlapping, so create a new bigger Interval
            let new_max = if self.max > other.max {
                self.max
            } else {
                other.max
            };
            let new_min = if self.min < other.min {
                self.min
            } else {
                other.min
            };
            return vec![Interval::new(new_min, new_max)];
        }
    }

    pub fn intersect(&self, other: Interval) -> Option<Interval> {
        if self.min > other.max || self.max < other.min {
            // Disjoint intervals
            return None;
        } else {
            // Overlapping, so there is some valid intersection
            let new_max = if self.max < other.max {
                self.max
            } else {
                other.max
            };
            let new_min = if self.min > other.min {
                self.min
            } else {
                other.min
            };
            Some(Interval::new(new_min, new_max))
        }
    }

    pub fn difference(&self, other: Interval) -> Vec<Interval> {
        if self.min > other.max || self.max < other.min {
            // Disjoint intervals
            return vec![*self];
        } else {
            // Overlapping, so either split in two, or on either side.
            if other.min <= self.min {
                // Clipping off left side
                if other.max >= self.max {
                    // Nothing left
                    return vec![];
                }
                let newint = Interval::new(other.max + 1, self.max);
                if newint.len() > 0 {
                    return vec![newint];
                } else {
                    return vec![];
                }
            } else if other.max >= self.max {
                // Clipping off right side
                let newint = Interval::new(self.min, other.min - 1);
                if newint.len() > 0 {
                    return vec![newint];
                } else {
                    return vec![];
                }
            } else {
                // Split into 2. Check sizes of both new Intervals.
                let int1 = Interval::new(self.min, other.min - 1);
                let int2 = Interval::new(other.max + 1, self.max);
                let mut output = vec![];
                if int1.len() > 0 {
                    output.push(int1);
                }
                if int2.len() > 0 {
                    output.push(int2);
                }
                output
            }
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}..{}]", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::Interval;

    #[test]
    fn len() {
        let int1 = Interval::new(-2, 3);
        let int2 = Interval::new(3, 4);
        let int3 = Interval::new(6, 6);
        assert_eq!(int1.len(), 6);
        assert_eq!(int2.len(), 2);
        assert_eq!(int3.len(), 1);
    }

    #[test]
    fn union() {
        let int1 = Interval::new(-2, 3);
        let int2 = Interval::new(3, 4);
        let int3 = Interval::new(4, 6);
        assert_eq!(int1.union(int2), vec![Interval::new(-2, 4)]);
        assert_eq!(int1.union(int3), vec![int1, int3]);
        assert_eq!(int3.union(int1), vec![int3, int1]);
    }

    #[test]
    fn intersect() {
        let int1 = Interval::new(-2, 3);
        let int2 = Interval::new(3, 4);
        let int3 = Interval::new(-1, 6);
        let int4 = Interval::new(7, 16);
        assert_eq!(int1.intersect(int2), Some(Interval::new(3, 3)));
        assert_eq!(int1.intersect(int3), Some(Interval::new(-1, 3)));
        assert_eq!(int3.intersect(int1), Some(Interval::new(-1, 3)));
        assert_eq!(int2.intersect(int3), Some(int2));
        assert_eq!(int1.intersect(int4), None);
    }

    #[test]
    fn difference() {
        let int1 = Interval::new(-2, 3);
        let int2 = Interval::new(3, 4);
        let int3 = Interval::new(-1, 6);
        let int4 = Interval::new(-5, 0);
        let int5 = Interval::new(-1, 0);
        let int6 = Interval::new(-2, 0);
        let int7 = Interval::new(0, 3);
        assert_eq!(int1.difference(int2), vec![Interval::new(-2, 2)]);
        assert_eq!(int1.difference(int3), vec![Interval::new(-2, -2)]);
        assert_eq!(int1.difference(int4), vec![Interval::new(1, 3)]);
        assert_eq!(int4.difference(int1), vec![Interval::new(-5, -3)]);
        assert_eq!(
            int1.difference(int5),
            vec![Interval::new(-2, -2), Interval::new(1, 3)]
        );
        assert_eq!(int1.difference(int6), vec![Interval::new(1, 3)]);
        assert_eq!(int1.difference(int7), vec![Interval::new(-2, -1)]);
    }
}
