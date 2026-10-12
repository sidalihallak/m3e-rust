use material_colors::{
    color::Rgb,
    dynamic_color::{DynamicScheme, Platform, SpecVersion, Variant},
    scheme::Scheme,
    theme::ThemeBuilder,
};

pub const SEEDS: [(&str, u32); 3] = [
    ("Violet", 0xff6750a4),
    ("Teal", 0xff006a60),
    ("Rose", 0xff9c4146),
];

pub struct ThemePreview {
    pub light: Scheme,
    pub dark: Scheme,
    pub palettes: DynamicScheme,
}

impl ThemePreview {
    pub fn from_seed(seed: u32) -> Self {
        Self::from_options(seed, ThemeVariant::TonalSpot, ThemeContrast::Standard)
    }

    pub fn from_options(seed: u32, variant: ThemeVariant, contrast: ThemeContrast) -> Self {
        let theme = ThemeBuilder::with_source(Rgb::from_u32(seed))
            .variant(variant.into())
            .contrast_level(contrast.level())
            .build();
        let palettes = DynamicScheme::from_spec(
            theme.source.into(),
            variant.into(),
            false,
            Some(contrast.level()),
            Platform::Phone,
            SpecVersion::Spec2021,
        );

        let mut light = theme.schemes.light;
        // Expressive 2021 light on-container roles follow upstream tone 30.
        if contrast == ThemeContrast::Standard
            && !matches!(variant, ThemeVariant::Content | ThemeVariant::Fidelity)
        {
            light.on_primary_container = palettes.primary_palette.tone(30);
            light.on_secondary_container = palettes.secondary_palette.tone(30);
            light.on_tertiary_container = palettes.tertiary_palette.tone(30);
            light.on_error_container = palettes.error_palette.tone(30);
        }
        Self {
            light,
            dark: theme.schemes.dark,
            palettes,
        }
    }
}

pub fn css_rgb(color: Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
}

/// Convert the MCU semantic scheme to CSS custom properties consumed by the UI.
pub fn css_variables(scheme: &Scheme) -> String {
    let roles = [
        ("primary-dim", scheme.primary),
        ("secondary-dim", scheme.secondary),
        ("tertiary-dim", scheme.tertiary),
        ("error-dim", scheme.error),
        ("inverse-primary", scheme.inverse_primary),
        ("error-container", scheme.error_container),
        ("on-error-container", scheme.on_error_container),
        ("surface-dim", scheme.surface_dim),
        ("surface-bright", scheme.surface_bright),
        ("surface-tint", scheme.surface_tint),
        ("surface-container-lowest", scheme.surface_container_lowest),
        ("primary-fixed", scheme.primary_fixed),
        ("primary-fixed-dim", scheme.primary_fixed_dim),
        ("on-primary-fixed", scheme.on_primary_fixed),
        ("on-primary-fixed-variant", scheme.on_primary_fixed_variant),
        ("secondary-fixed", scheme.secondary_fixed),
        ("secondary-fixed-dim", scheme.secondary_fixed_dim),
        ("on-secondary-fixed", scheme.on_secondary_fixed),
        (
            "on-secondary-fixed-variant",
            scheme.on_secondary_fixed_variant,
        ),
        ("tertiary-fixed", scheme.tertiary_fixed),
        ("tertiary-fixed-dim", scheme.tertiary_fixed_dim),
        ("on-tertiary-fixed", scheme.on_tertiary_fixed),
        (
            "on-tertiary-fixed-variant",
            scheme.on_tertiary_fixed_variant,
        ),
        ("background", scheme.background),
        ("on-background", scheme.on_background),
        ("scrim", scheme.scrim),
        ("shadow", scheme.shadow),
        ("primary", scheme.primary),
        ("on-primary", scheme.on_primary),
        ("primary-container", scheme.primary_container),
        ("on-primary-container", scheme.on_primary_container),
        ("secondary", scheme.secondary),
        ("on-secondary", scheme.on_secondary),
        ("secondary-container", scheme.secondary_container),
        ("on-secondary-container", scheme.on_secondary_container),
        ("tertiary", scheme.tertiary),
        ("on-tertiary", scheme.on_tertiary),
        ("tertiary-container", scheme.tertiary_container),
        ("on-tertiary-container", scheme.on_tertiary_container),
        ("error", scheme.error),
        ("on-error", scheme.on_error),
        ("surface", scheme.surface),
        ("on-surface", scheme.on_surface),
        ("surface-variant", scheme.surface_variant),
        ("on-surface-variant", scheme.on_surface_variant),
        ("surface-container-low", scheme.surface_container_low),
        ("surface-container", scheme.surface_container),
        ("surface-container-high", scheme.surface_container_high),
        (
            "surface-container-highest",
            scheme.surface_container_highest,
        ),
        ("outline", scheme.outline),
        ("outline-variant", scheme.outline_variant),
        ("inverse-surface", scheme.inverse_surface),
        ("inverse-on-surface", scheme.inverse_on_surface),
    ];

    roles
        .iter()
        .map(|(name, color)| format!("--md-sys-color-{name}: {};", css_rgb(*color)))
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ThemeVariant {
    #[default]
    TonalSpot,
    Content,
    Fidelity,
    Expressive,
    Neutral,
    Monochrome,
    Vibrant,
}
impl From<ThemeVariant> for Variant {
    fn from(value: ThemeVariant) -> Self {
        match value {
            ThemeVariant::TonalSpot => Self::TonalSpot,
            ThemeVariant::Content => Self::Content,
            ThemeVariant::Fidelity => Self::Fidelity,
            ThemeVariant::Expressive => Self::Expressive,
            ThemeVariant::Neutral => Self::Neutral,
            ThemeVariant::Monochrome => Self::Monochrome,
            ThemeVariant::Vibrant => Self::Vibrant,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ThemeContrast {
    #[default]
    Standard,
    Medium,
    High,
}
impl ThemeContrast {
    pub fn level(self) -> f64 {
        match self {
            Self::Standard => 0.0,
            Self::Medium => 0.5,
            Self::High => 1.0,
        }
    }
}

/// Rank decoded opaque image pixels using Material Celebi quantization and HCT scoring.
/// Empty or wholly neutral images use the baseline violet fallback.
/// Browser decoding/resampling belongs to ImageThemePicker; native callers can supply RGB pixels.
pub fn image_seed_candidates(pixels: &[u32]) -> Vec<u32> {
    use material_colors::{
        quantize::{Quantizer, QuantizerCelebi},
        score::Score,
    };
    let pixels: Vec<Rgb> = pixels
        .iter()
        .take(16384)
        .map(|p| Rgb::from_u32(*p))
        .collect();
    if pixels.is_empty() {
        return vec![SEEDS[0].1];
    }
    let quantized = QuantizerCelebi::quantize(&pixels, 128);
    Score::score(
        &quantized.color_to_count,
        Some(4),
        Some(Rgb::from_u32(SEEDS[0].1)),
        Some(true),
    )
    .into_iter()
    .map(|c| 0xff000000 | ((c.red as u32) << 16) | ((c.green as u32) << 8) | c.blue as u32)
    .collect()
}
