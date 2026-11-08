// Что ставит `yami-shot install`: файлы, вшитые в бинарь, и вопросы к человеку (yami_px::install).

use yami_px::install::Spec;

pub const SPEC: Spec = Spec {
    name: "yami-shot",
    version: env!("CARGO_PKG_VERSION"),
    about: ["скриншоты: разметка, текст и QR со снимка", "screenshots: markup, text and QR from a shot", "capturas: anotações, texto e QR"], // i18n-ok
    deps: "grim, wl-clipboard; optional: tesseract, zbar, yami-files or zenity",
    assets: &[],
    choices: &[],
};
