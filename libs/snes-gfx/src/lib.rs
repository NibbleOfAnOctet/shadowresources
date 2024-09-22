
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
        assert_eq!(palette.count_palettes(palette::Format::BPP4), 1);
        assert_eq!(palette.count_palettes(palette::Format::BPP2), 4);

        let palette = Palette::load(&[0xff, 0x7f].repeat(32));
        assert_eq!(palette.count_palettes(palette::Format::BPP4), 2);
        assert_eq!(palette.count_palettes(palette::Format::BPP2), 8);
    }

    #[test]
    fn palette_returns_correct_rgb_values() {
        let mut palettes = [0xff, 0x7f].repeat(16);
        palettes.append(&mut [0x21, 0x04].repeat(16));

        let palette = Palette::load(&palettes);
        assert_eq!(palette.get_rgb_color(0, 0, palette::Format::BPP4), [248, 248, 248]);
        assert_eq!(palette.get_rgb_color(1, 0, palette::Format::BPP4), [8, 8, 8]);
        assert_eq!(palette.get_rgb_color(0, 0, palette::Format::BPP2), [248, 248, 248]);
        assert_eq!(palette.get_rgb_color(4, 0, palette::Format::BPP2), [8, 8, 8]);
        assert_ne!(palette.get_rgb_color(0, 0, palette::Format::BPP2), [8, 8, 8]);
    }
}
