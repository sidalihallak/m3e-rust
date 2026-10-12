use super::motion::next_id;
use crate::theme::{css_rgb, image_seed_candidates};
use dioxus::prelude::*;
use material_colors::color::Rgb;

/// Local image decoding with bounded RGB sampling and Rust MCU seed ranking.
/// Selection is controlled by the caller. Image bytes never leave the browser.
#[component]
pub fn ImageThemePicker(
    #[props(default="Choose theme image".to_string())] label: String,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    #[props(default)] onseedchange: EventHandler<u32>,
) -> Element {
    let id = use_hook(|| next_id("m3-image-theme"));
    let mut candidates = use_signal(Vec::<u32>::new);
    let mut status = use_signal(|| "Choose an image to extract theme colors.".to_string());
    let mut busy = use_signal(|| false);
    let mut selected = use_signal(|| None::<u32>);
    use_effect({
        let id = id.clone();
        move || {
            let script = include_str!("image_theme.js").replace("__ROOT_ID__", &format!("{id:?}"));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok((kind, pixels)) = eval.recv::<(String, Vec<u32>)>().await {
                    match kind.as_str() {
                        "loading" => {
                            busy.set(true);
                            status.set("Reading image colors…".into());
                        }
                        "pixels" => {
                            let found = image_seed_candidates(&pixels);
                            let seed = found[0];
                            candidates.set(found);
                            selected.set(Some(seed));
                            busy.set(false);
                            status.set(format!(
                                "Image colors ready. Seed {}",
                                css_rgb(Rgb::from_u32(seed))
                            ));
                            onseedchange.call(seed);
                        }
                        _ => {
                            busy.set(false);
                            status.set(kind);
                        }
                    }
                }
            });
        }
    });
    use_drop({
        let id = id.clone();
        move || {
            spawn(async move {
                let _ = document::eval(&format!("window.__m3ImageThemes?.[{id:?}]?.();")).await;
            });
        }
    });
    rsx! {div{id:id.clone(),class:"m3-image-theme {class}",aria_busy:busy(),
        label{class:"m3-image-label",r#for:"{id}-file","{label}"}
        input{id:"{id}-file",r#type:"file",accept:"image/*",disabled,aria_describedby:"{id}-status"}
        img{class:"m3-image-preview",alt:"Selected image used to generate theme colors",hidden:true}
        p{id:"{id}-status",role:"status","{status}"}
        div{class:"m3-image-swatches",for color in candidates(){
            super::Button {disabled,variant:super::ButtonVariant::Tonal,toggle:true,selected:selected()==Some(color),onclick:move |_|{selected.set(Some(color));status.set(format!("Selected seed {}",css_rgb(Rgb::from_u32(color))));onseedchange.call(color);},
                span{class:"m3-seed-swatch",style:format!("background:{}",css_rgb(Rgb::from_u32(color))),aria_hidden:"true"}
                "{css_rgb(Rgb::from_u32(color))}"
            }
        }}
    }}
}
