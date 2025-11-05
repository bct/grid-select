use crate::grid::GridPosition;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Space {
    pub width: f32,
    pub height: f32,
}

impl Space {
    pub fn scale(&self, s: f32) -> Space {
        Space {
            width: self.width * s,
            height: self.height * s,
        }
    }
}

// take a grid position and return the space it occupies on the screen
pub fn grid_position_to_screen(
    screen_space: &Space,
    grid_position: &GridPosition,
    width: f32,
    height: f32,
    margin: f32,
) -> (ScreenPosition, Space) {
    // the top left pixel of grid item 0,0
    let x_0 = screen_space.width / 2. - width / 2.;
    let y_0 = screen_space.height / 2. - height / 2.;

    let x = x_0 + (width + margin) * grid_position.x as f32;
    let y = y_0 + (height + margin) * grid_position.y as f32;

    (ScreenPosition { x, y }, Space { width, height })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            grid_position_to_screen(
                &Space {
                    width: 1000.,
                    height: 1000.
                },
                &GridPosition::new(0, 0),
                100.,
                50.,
                0.
            ),
            (
                ScreenPosition { x: 450., y: 475. },
                Space {
                    width: 100.,
                    height: 50.
                }
            )
        );

        assert_eq!(
            grid_position_to_screen(
                &Space {
                    width: 1000.,
                    height: 1000.
                },
                &GridPosition::new(0, 1),
                100.,
                50.,
                0.
            ),
            (
                ScreenPosition { x: 450., y: 525. },
                Space {
                    width: 100.,
                    height: 50.
                }
            )
        );

        assert_eq!(
            grid_position_to_screen(
                &Space {
                    width: 1000.,
                    height: 1000.
                },
                &GridPosition::new(1, 0),
                100.,
                50.,
                0.
            ),
            (
                ScreenPosition { x: 550., y: 475. },
                Space {
                    width: 100.,
                    height: 50.
                }
            )
        );
    }
}
