use crate::grid;

pub struct State {
    pub grid: grid::Grid,

    // the current cursor position
    pub cursor_position: grid::GridPosition,

    // should we shut down?
    pub should_exit: bool,

    // does the entire window need to be redrawn?
    pub needs_redraw: bool,

    // the last cursor position we rendered
    pub rendered_cursor_position: grid::GridPosition,
}

impl<'a> State {
    pub fn new(grid: grid::Grid) -> State {
        State {
            grid: grid,
            cursor_position: grid::GridPosition::new(0, 0),
            should_exit: false,
            needs_redraw: true,
            rendered_cursor_position: grid::GridPosition::new(0, 0),
        }
    }

    pub fn cursor_move_left(&mut self) {
        let new_x = self.cursor_position.x - 1;
        if self.grid.item_at(new_x, self.cursor_position.y).is_some() {
            self.cursor_position.x = new_x;
        }
    }

    pub fn cursor_move_down(&mut self) {
        let new_y = self.cursor_position.y + 1;
        if self.grid.item_at(self.cursor_position.x, new_y).is_some() {
            self.cursor_position.y = new_y;
        }
    }

    pub fn cursor_move_up(&mut self) {
        let new_y = self.cursor_position.y - 1;
        if self.grid.item_at(self.cursor_position.x, new_y).is_some() {
            self.cursor_position.y = new_y;
        }
    }

    pub fn cursor_move_right(&mut self) {
        let new_x = self.cursor_position.x + 1;
        if self.grid.item_at(new_x, self.cursor_position.y).is_some() {
            self.cursor_position.x = new_x;
        }
    }

    pub fn get_selected_value(&self) -> &str {
        let maybe_grid_item = self
            .grid
            .item_at(self.cursor_position.x, self.cursor_position.y);

        match maybe_grid_item {
            None => {
                panic!(
                    "could not select nonexistent item at x={}, y={}",
                    self.cursor_position.x, self.cursor_position.y
                )
            }
            Some(grid_item) => grid_item.value.as_str(),
        }
    }

    pub fn cursor_needs_rerender(&self) -> bool {
        self.rendered_cursor_position != self.cursor_position
    }
}
