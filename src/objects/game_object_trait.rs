use termion::raw::RawTerminal;
use std::io::StdoutLock;

pub trait GameObjectTrait {
    fn get_position(&self) -> (u16, u16);

    fn get_model_bytes(&self) -> &[u8];

    fn move_object(&mut self, x: u16, y: u16);

    fn get_size(&self) -> (u16, u16, u16, u16);

    fn is_alive(&self) -> bool;

    fn destroy(&mut self);

    fn draw(&self, stdout: &mut RawTerminal<StdoutLock>);

    fn is_collide(&self, object: &impl GameObjectTrait) -> bool {
        let (o1_x, o1_y, o1_width, o1_height) = self.get_size();
        let (o2_x, o2_y, o2_width, o2_height) = object.get_size();

        o1_y >= o2_y
            && o1_y <= o2_y + o2_height
            && o1_x >= o2_x
            && o1_x <= o2_x + o2_width - 1
    }
}