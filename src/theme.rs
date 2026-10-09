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
        let theme = ThemeBuilder::with_source(Rgb::from_u32(seed))
            .contrast_level(0.0)
            .build();
        let palettes = DynamicScheme::from_spec(
            theme.source.into(),
            Variant::TonalSpot,
            false,
            Some(0.0),
            Platform::Phone,
            SpecVersion::Spec2021,
        );

        Self {
            light: theme.schemes.light,
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
        ("primary", scheme.primary),
        ("on-primary", scheme.on_primary),
        ("primary-container", scheme.primary_container),
        ("on-primary-container", scheme.on_primary_container),
        ("secondary", scheme.secondary),
        ("on-secondary", scheme.on_secondary),
        ("secondary-container", scheme.secondary_container),
        ("tertiary", scheme.tertiary),
        ("on-tertiary", scheme.on_tertiary),
        ("tertiary-container", scheme.tertiary_container),
        ("error", scheme.error),
        ("on-error", scheme.on_error),
        ("surface", scheme.surface),
        ("on-surface", scheme.on_surface),
        ("surface-variant", scheme.surface_variant),
        ("on-surface-variant", scheme.on_surface_variant),
        ("surface-container-low", scheme.surface_container_low),
        ("surface-container", scheme.surface_container),
        ("surface-container-high", scheme.surface_container_high),
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
