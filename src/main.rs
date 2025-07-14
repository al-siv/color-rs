use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use palette::{Lab, Srgb, FromColor, IntoColor};

#[derive(Parser)]
#[command(name = "color-rs")]
#[command(about = "A CLI tool for color gradient calculations using LAB color space")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a gradient between two colors using LAB color space
    Gradient(GradientArgs),
}

#[derive(Args)]
struct GradientArgs {
    /// Starting color in HEX format (e.g., #FF0000)
    #[arg(long, value_name = "HEX")]
    start_color: String,

    /// Starting position as percentage (e.g., 20 for 20%)
    #[arg(long, value_name = "PERCENT")]
    start_position: u8,

    /// Ending color in HEX format (e.g., #0000FF)
    #[arg(long, value_name = "HEX")]
    end_color: String,

    /// Ending position as percentage (e.g., 80 for 80%)
    #[arg(long, value_name = "PERCENT")]
    end_position: u8,

    /// Smoothing coefficient for gradient curve (default: 2.0)
    #[arg(long, default_value = "2.0")]
    smoothing: f64,

    /// Additional curve tension parameter (default: 0.5)
    #[arg(long, default_value = "0.5")]
    tension: f64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gradient(args) => generate_gradient(args)?,
    }

    Ok(())
}

fn parse_hex_color(hex: &str) -> Result<Lab> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(anyhow!("Invalid HEX color format. Expected format: #RRGGBB"));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    let rgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    Ok(Lab::from_color(rgb))
}

fn lab_to_hex(lab: Lab) -> String {
    let rgb: Srgb = lab.into_color();
    let r = (rgb.red * 255.0).round() as u8;
    let g = (rgb.green * 255.0).round() as u8;
    let b = (rgb.blue * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn smooth_interpolate(t: f64, smoothing: f64, tension: f64) -> f64 {
    // Создаем сглаженную кривую используя комбинацию функций
    let smooth_t = smoothstep(0.0, 1.0, t);
    
    // Применяем дополнительное сглаживание
    let powered_t = smooth_t.powf(smoothing);
    
    // Добавляем напряжение для более естественной кривой
    let tension_factor = 1.0 + tension * (1.0 - 2.0 * (smooth_t - 0.5).abs());
    
    (powered_t * tension_factor).clamp(0.0, 1.0)
}

fn interpolate_lab(start: Lab, end: Lab, t: f64) -> Lab {
    let t = t as f32;
    Lab::new(
        start.l + (end.l - start.l) * t,
        start.a + (end.a - start.a) * t,
        start.b + (end.b - start.b) * t,
    )
}

fn generate_gradient(args: GradientArgs) -> Result<()> {
    // Проверяем корректность позиций
    if args.start_position >= args.end_position {
        return Err(anyhow!("Start position must be less than end position"));
    }
    if args.start_position > 100 || args.end_position > 100 {
        return Err(anyhow!("Positions must be between 0 and 100"));
    }

    // Парсим цвета
    let start_lab = parse_hex_color(&args.start_color)?;
    let end_lab = parse_hex_color(&args.end_color)?;

    // Генерируем градиент
    for position in args.start_position..=args.end_position {
        // Нормализуем позицию в диапазон [0, 1]
        let normalized_t = (position - args.start_position) as f64 
            / (args.end_position - args.start_position) as f64;
        
        // Применяем сглаживание
        let smooth_t = smooth_interpolate(normalized_t, args.smoothing, args.tension);
        
        // Интерполируем цвет в LAB пространстве
        let interpolated_lab = interpolate_lab(start_lab, end_lab, smooth_t);
        
        // Конвертируем обратно в HEX
        let hex_color = lab_to_hex(interpolated_lab);
        
        println!("{}%: {}", position, hex_color);
    }

    Ok(())
}
