#![allow(dead_code)]

pub mod palette;
pub mod tilemap;
pub mod tileset;

#[cfg(test)]
mod tests {
    use palette::Palette;

    use super::*;

    #[test]
    fn palette_returns_correct_number_of_palettes() {
        let palette = Palette::load(&[0xff, 0x7f].repeat(16));
        assert_eq!(palette.count_palettes(), 1);

        let palette = Palette::load(&[0xff, 0x7f].repeat(32));
        assert_eq!(palette.count_palettes(), 2);
    }

    #[test]
    fn palette_returns_correct_rgb_values() {
        let mut palettes = [0xff, 0x7f].repeat(16);
        palettes.append(&mut [0x21, 0x04].repeat(16));

        let palette = Palette::load(&palettes);
        assert_eq!(palette.get_rgb_color(0, 0), [255, 255, 255]);
        assert_eq!(palette.get_rgb_color(1, 0), [8, 8, 8]);
    }
}
