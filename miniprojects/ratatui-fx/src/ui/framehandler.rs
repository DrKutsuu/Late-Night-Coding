use ratatui::layout::Rect;
use crate::ui::display::UIAreas;
/*
    FrameHandlers are "dumb" views that simply draw their state (T) or other given input to a terminal frame (the screen)
 */
pub trait FrameHandler<T> {
    /*
        When a FrameHandler "handles" a frame it essentially just draws it's input / content to the frame it is provided a frame by a View/Higher level UI component
     */
    fn handle_frame(&mut self, frame: &mut ratatui::Frame, data: &mut T);
}


impl<T> dyn FrameHandler<T> {
}