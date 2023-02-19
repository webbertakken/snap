//everything is handled by this struct
pub struct Widget {
    margin: egui::Vec2,
}

//our methods for it
impl Widget {
    pub fn new() -> Self {
        Self {
            //The Vec2 type is just a 2D vector made from 2 f32s.
            //this is (0.0, 0.0) btw
            margin: egui::Vec2::default(),
        }
    }
    //the method we want to use when we draw
    pub fn update<R>(&mut self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) {
        ui.horizontal(|ui| {
            //get all the size we have before drawing
            let total_size: egui::Vec2 = ui.available_size();

            //we can only add space in one direction, so here is horizontal.
            ui.add_space(self.margin.x);

            //the show method for a frame returns an InnerResponse<R> type where R is any data we want
            //however were not interested in that, so here R is '()' (called unit, it's like void in C)
            let frame_response = egui::Frame::none().show(ui, add_contents);

            //in the InnerResponse we can retrive a 'egui::Rect' type which describes
            //the area of the screen that frame takes up, but we just want the size :P
            let frame_size = frame_response.response.rect.size();
            let available_space_after = total_size - frame_size;

            //available_space_after includes both sides, but we only want how big one side is
            self.margin = available_space_after / 2.0;
        });
    }
}
