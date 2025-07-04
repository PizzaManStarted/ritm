use egui::Color32;

struct Palette {
    color1: Color32,
    color2: Color32,
    color3: Color32,
    color4: Color32,
    color5: Color32,
}

pub struct Palettes {
    palettes : Vec<Palette>,
    current_palette_id: usize
}


impl Default for Palettes {

    fn default() -> Self {
        Self {
            palettes: vec![
                Palette { 
                    color1: Color32::WHITE,
                    color2: Color32::WHITE,
                    color3: Color32::WHITE,
                    color4: Color32::WHITE,
                    color5: Color32::WHITE,
                }
            ],
            current_palette_id: 0,
        }
    }
}


impl Palettes {
    
    fn color1(&self) -> Color32 { self.palettes.get(self.current_palette_id).unwrap().color1 }
    fn color2(&self) -> Color32 { self.palettes.get(self.current_palette_id).unwrap().color2 }
    fn color3(&self) -> Color32 { self.palettes.get(self.current_palette_id).unwrap().color3 }
    fn color4(&self) -> Color32 { self.palettes.get(self.current_palette_id).unwrap().color4 }
    fn color5(&self) -> Color32 { self.palettes.get(self.current_palette_id).unwrap().color5 }
}