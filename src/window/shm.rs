use super::Window;
use smithay_client_toolkit::shm::{Shm, ShmHandler};

impl ShmHandler for Window {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}
