use palette::{FromColor, Gradient, Lch, Srgb, Srgba};
use rand::Rng;

pub fn sample_gradient(gradient: &Gradient<Lch>, rng: &mut impl Rng) -> Srgba<u8> {
    let lch_color = gradient.get(rng.gen_range(0.0..=1.0));
    let (r, g, b) = Srgb::from_color(lch_color).into_components();

    Srgba::<u8>::new(
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
        255,
    )
}
