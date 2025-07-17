use palette::{Lab, Hsl, IntoColor};

fn main() {
    // Проблемный Lab цвет из вывода
    let lab_color = Lab::new(95.90, 80.09, 67.20);
    
    // Конвертируем в HSL
    let hsl: Hsl = lab_color.into_color();
    
    println!("Lab: lab({:.2}, {:.2}, {:.2})", lab_color.l, lab_color.a, lab_color.b);
    println!("HSL: hsl({:.0}, {:.1}%, {:.1}%)", hsl.hue.into_degrees(), hsl.saturation * 100.0, hsl.lightness * 100.0);
    
    // Также проверим RGB промежуточный результат
    let rgb: palette::Srgb = lab_color.into_color();
    println!("RGB: rgb({}, {}, {})", 
        (rgb.red * 255.0) as u8, 
        (rgb.green * 255.0) as u8, 
        (rgb.blue * 255.0) as u8);
}
