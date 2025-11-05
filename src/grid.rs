use crate::spiral;

// the largest grid that we'll accept.
// we're too slow to handle large grids right now.
// dimension 7 fits 25 items.
const MAX_DIMENSION: usize = 7;

// an x/y coordinate, relative to the centre position.
#[derive(Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct GridPosition {
    pub x: i8,
    pub y: i8,
}

impl GridPosition {
    pub fn new(x: i8, y: i8) -> GridPosition {
        GridPosition { x, y }
    }
}

#[derive(Clone)]
pub struct GridItem {
    pub value: String,
    pub position: GridPosition,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,

    pub grid: Vec<Vec<Option<GridItem>>>,
}

impl Grid {
    pub fn new(item_names: &Vec<String>) -> anyhow::Result<Grid> {
        let dimension = spiral::sufficient_diameter(item_names.len());
        anyhow::ensure!(
            dimension <= MAX_DIMENSION,
            "grid is too large: {} > {}",
            dimension,
            MAX_DIMENSION
        );

        let mut grid: Vec<Vec<Option<GridItem>>> = Vec::with_capacity(dimension);
        for _ in 0..dimension {
            grid.push(vec![None; dimension]);
        }

        let mut result = Grid {
            width: dimension,
            height: dimension,
            grid: grid,
        };

        let positions = spiral::SpiralGenerator::new();
        for (item_name, pos) in item_names.iter().zip(positions) {
            let (idx_x, idx_y) = result.rel_to_abs(pos.x, pos.y).unwrap();
            let item = GridItem {
                value: item_name.to_string(),
                position: pos,
            };
            result.grid[idx_x][idx_y] = Some(item);
        }

        Ok(result)
    }

    // convert a position relative to the centre of the grid to a position in our 2d vec
    fn rel_to_abs(&self, x: i8, y: i8) -> Option<(usize, usize)> {
        let width = self.width;
        let height = self.height;

        let idx_x = (width / 2).checked_add_signed(x as isize)?;
        let idx_y = (height / 2).checked_add_signed(y as isize)?;

        if idx_x >= width || idx_y >= height {
            return None;
        }

        Some((idx_x, idx_y))
    }

    pub fn item_at(&self, x: i8, y: i8) -> Option<&GridItem> {
        let (idx_x, idx_y) = self.rel_to_abs(x, y)?;
        self.grid[idx_x][idx_y].as_ref()
    }

    pub fn items_iter(&self) -> GridSpiralIterator {
        GridSpiralIterator::new(self)
    }
}

pub struct GridSpiralIterator<'a> {
    grid: &'a Grid,
    positions: spiral::SpiralGenerator,
}

impl<'a> GridSpiralIterator<'_> {
    pub fn new(grid: &Grid) -> GridSpiralIterator {
        GridSpiralIterator {
            grid,
            positions: spiral::SpiralGenerator::new(),
        }
    }
}

impl<'a> Iterator for GridSpiralIterator<'a> {
    type Item = &'a GridItem;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.positions.next();
        position.and_then(|p| self.grid.item_at(p.x, p.y))
    }
}
