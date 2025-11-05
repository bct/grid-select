use crate::grid::GridPosition;

// the dimension of the given ring
//
// e.g. ring 1 has width & height 3
fn ring_diameter(ring: usize) -> usize {
    ring * 2 + 1
}

// the number of positions within the given ring
//
// e.g. positions_in_ring(1) => 4:
//
//   x
//  x x
//   x
fn positions_in_ring(ring: usize) -> usize {
    if ring == 0 { 1 } else { ring * 4 }
}

// the number of positions within the given ring and all the rings it contains
//
// e.g. for ring == 1:
//
//   x
//  xxx
//   x
#[allow(dead_code)]
fn total_positions_within(ring: usize) -> usize {
    4 * (ring + 1) * ring / 2 + 1
}

// the width & height of a grid that can contain the desired number of positions.
// we count the entire diameter of partial rings.
//
// e.g. to position 6 items we need 3 rings, with a total diameter of 5.
//
// this function could also be expressed as:
//
//      let mut ring = 0;
//      while total_positions_within(ring) < desired_position_count {
//          ring += 1;
//      }
//      ring_diameter(ring)
pub fn sufficient_diameter(desired_position_count: usize) -> usize {
    if desired_position_count == 0 {
        return 0;
    }

    // the inverse of total_positions_within.
    let ps = desired_position_count as f32;
    let largest_ring = (-1. + (2. * ps - 1.).sqrt()) / 2.;
    ring_diameter(largest_ring.ceil() as usize)
}

pub struct SpiralGenerator {
    // the next position we're going to return
    next_result: GridPosition,

    // the ring we're currently generating
    // ring 0: 1 position
    // ring 1: 4 positions
    ring: usize,

    // the number of positions remaining to be placed in the current ring
    ring_positions_remaining: usize,

    // the direction to move when we place the next position
    dx: i8,
    dy: i8,
}

impl SpiralGenerator {
    pub fn new() -> SpiralGenerator {
        SpiralGenerator {
            next_result: GridPosition::new(0, 0),
            ring: 0,
            ring_positions_remaining: 0,

            dx: 0,
            dy: 0,
        }
    }
}

impl Iterator for SpiralGenerator {
    type Item = GridPosition;

    // generate the next grid position in the spiral.
    //
    // we're generating successive diamond-shaped rings, beginning with a single
    // position in the centre at 0,0.
    fn next(&mut self) -> Option<Self::Item> {
        // copy the current position before returning it
        let current_pos = GridPosition {
            x: self.next_result.x,
            y: self.next_result.y,
        };

        if self.ring_positions_remaining == 0 {
            // the ring is full, begin a new ring
            self.ring += 1;

            // the number of rows / columns in the ring
            self.ring_positions_remaining = positions_in_ring(self.ring);

            // first entry of the new ring is in the bottommost position
            self.next_result.x = 0;
            self.next_result.y = self.ring.try_into().unwrap();

            // initially we move up and to the right
            self.dx = 1;
            self.dy = -1;
        } else {
            self.next_result.x += self.dx;
            self.next_result.y += self.dy;

            // we're crossing the vertical axis; "bounce" off a horizontal wall.
            if self.next_result.x == 0 {
                self.dy *= -1;
            }

            // we're crossing the horizontal axis; "bounce" off a vertical wall.
            if self.next_result.y == 0 {
                self.dx *= -1;
            }
        }

        self.ring_positions_remaining -= 1;

        Some(current_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut sg = SpiralGenerator::new();

        // ring 0
        assert_eq!(sg.next(), Some(GridPosition::new(0, 0)));

        // ring 1
        assert_eq!(sg.next(), Some(GridPosition::new(0, 1)));
        assert_eq!(sg.next(), Some(GridPosition::new(1, 0)));
        assert_eq!(sg.next(), Some(GridPosition::new(0, -1)));
        assert_eq!(sg.next(), Some(GridPosition::new(-1, 0)));

        // ring 2
        assert_eq!(sg.next(), Some(GridPosition::new(0, 2)));
        assert_eq!(sg.next(), Some(GridPosition::new(1, 1)));
        assert_eq!(sg.next(), Some(GridPosition::new(2, 0)));
        assert_eq!(sg.next(), Some(GridPosition::new(1, -1)));
        assert_eq!(sg.next(), Some(GridPosition::new(0, -2)));
    }

    #[test]
    fn test_ring_diameter() {
        assert_eq!(1, ring_diameter(0));
        assert_eq!(3, ring_diameter(1));
        assert_eq!(5, ring_diameter(2));
        assert_eq!(7, ring_diameter(3));
    }

    #[test]
    fn test_positions_in_ring() {
        assert_eq!(1, positions_in_ring(0));
        assert_eq!(4, positions_in_ring(1));
        assert_eq!(8, positions_in_ring(2));
        assert_eq!(12, positions_in_ring(3));
    }

    #[test]
    fn test_sufficient_diameter() {
        assert_eq!(0, sufficient_diameter(0));
        assert_eq!(1, sufficient_diameter(1));
        assert_eq!(3, sufficient_diameter(2));
        assert_eq!(3, sufficient_diameter(3));
        assert_eq!(3, sufficient_diameter(4));
        assert_eq!(3, sufficient_diameter(5));
        assert_eq!(5, sufficient_diameter(6));
    }
}
