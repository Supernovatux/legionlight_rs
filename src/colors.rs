use serde::Deserialize;

#[derive(Default, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub wallpaper: String,
    pub alpha: String,
    pub special: Special,
    pub colors: Colors,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Special {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    pub color0: hex_color::HexColor,
    pub color1: hex_color::HexColor,
    pub color2: hex_color::HexColor,
    pub color3: hex_color::HexColor,
    pub color4: hex_color::HexColor,
    pub color5: hex_color::HexColor,
    pub color6: hex_color::HexColor,
    pub color7: hex_color::HexColor,
    pub color8: hex_color::HexColor,
    pub color9: hex_color::HexColor,
    pub color10: hex_color::HexColor,
    pub color11: hex_color::HexColor,
    pub color12: hex_color::HexColor,
    pub color13: hex_color::HexColor,
    pub color14: hex_color::HexColor,
    pub color15: hex_color::HexColor,
}
