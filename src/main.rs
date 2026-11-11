// yami-shot — скриншоты для Wayland: заморозить экран, выделить, разметить, скопировать, сохранить,
// прочитать текст или QR. Работает сам по себе, на любом композиторе с layer-shell и screencopy.
//
//   yami-shot [area|window|screen] [--delay СЕК]
//   yami-shot ocr ФАЙЛ                 текст с картинки
//   yami-shot shot [-o ПАПКА] [--lang en]   снимки оверлея в PNG без Wayland

mod setup;
mod app;
mod cfg;
mod draw;
mod i18n;
mod notify;
mod ocr;
mod snap;

use snap::Mode;
use std::path::PathBuf;
use yami_px::{font, gfx, icons, look, ui};

const HELP: &str = "yami-shot [area|window|screen] [--delay SECONDS]
yami-shot ocr FILE
yami-shot install|uninstall|deps
yami-shot shot [-o DIR] [--lang en] [--frame PNG]

settings: ~/.config/yami/shot.json, look: ~/.config/yami/style.json and style.shot.json";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(c) = yami_px::install::dispatch(&setup::SPEC, &args) {
        std::process::exit(c);
    }
    let mut mode = Mode::Area;
    let mut delay = 0;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--version" => return println!("yami-shot {}", env!("CARGO_PKG_VERSION")),
            "-h" | "--help" => return println!("{HELP}"),
            "shot" => std::process::exit(shot(&args[1..])),
            "ocr" => {
                i18n::set(&look::Look::load("shot").lang);
                std::process::exit(ocr::cli(&args[1..]));
            }
            "area" => mode = Mode::Area,
            "window" => mode = Mode::Window,
            "screen" => mode = Mode::Screen,
            "--delay" => delay = it.next().and_then(|d| d.parse().ok()).unwrap_or(0),
            other => {
                eprintln!("yami-shot: unknown argument: {other}\n{HELP}");
                std::process::exit(2);
            }
        }
    }
    let Some(lock) = take_lock() else { return };
    let look = look::Look::load("shot");
    i18n::set(&look.lang);
    std::process::exit(app::run(mode, delay, look, cfg::Cfg::load(), lock));
}

/// Один оверлей на сеанс: повторное нажатие закрывает открытый. None — так и вышло, этому процессу
/// делать нечего; Some(None) — замка нет (нет XDG_RUNTIME_DIR), работаем без него
fn take_lock() -> Option<Option<std::fs::File>> {
    use std::io::{Read, Seek, Write};
    use std::os::unix::io::AsRawFd;
    let Some(dir) = std::env::var_os("XDG_RUNTIME_DIR") else { return Some(None) };
    let Ok(mut f) = std::fs::OpenOptions::new().create(true).truncate(false).read(true).write(true).open(PathBuf::from(dir).join("yami-shot.lock")) else { return Some(None) };
    // SAFETY: flock на своём открытом файле; снимается при закрытии
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        if let Ok(pid) = s.trim().parse::<i32>() {
            // SAFETY: простой сигнал процессу из своего же файла замка
            unsafe { libc::kill(pid, libc::SIGTERM) };
        }
        return None;
    }
    let _ = f.set_len(0);
    let _ = f.rewind();
    let _ = write!(f, "{}", std::process::id());
    Some(Some(f))
}

/// offscreen: выбор области и разметка на нарисованном «рабочем столе»
fn shot(args: &[String]) -> i32 {
    let mut out = PathBuf::from("shots");
    let mut lang = String::new();
    let mut frame = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "-o" => out = it.next().map(PathBuf::from).unwrap_or(out),
            "--lang" => lang = it.next().cloned().unwrap_or_default(),
            // настоящий кадр экрана под оверлеем (картинки для README)
            "--frame" => frame = it.next().and_then(|f| image::open(f).ok()).map(|i| i.to_rgb8()),
            _ => {}
        }
    }
    let (w, h) = frame.as_ref().map_or((1280, 720), |f| (f.width() as i32, f.height() as i32));
    let frame_px = frame.map(|f| f.pixels().map(|p| 0xff000000 | (p[0] as u32) << 16 | (p[1] as u32) << 8 | p[2] as u32).collect::<Vec<u32>>());
    let mut look = look::Look::load("shot");
    look.animations = false;
    if !lang.is_empty() {
        look.lang = lang.clone();
    }
    i18n::set(&look.lang);
    // русская надпись без перевода — ошибка на любом языке, кроме русского; `--fit`: надписи, которым тесно
    let audit = !matches!(yami_px::i18n::normalize(&lang), "ru" | "");
    i18n::audit(audit);
    yami_px::gfx::watch_text(args.iter().any(|a| a == "--fit"));
    let _ = std::fs::create_dir_all(&out);
    let mut fonts = font::Fonts::with(look.font("shot"));
    let mut ic = icons::Icons::new("");
    let t0 = std::time::Instant::now();
    let mut bad = 0;
    for (name, edit) in [("pick", false), ("edit", true)] {
        let sn = snap::Snap::mock(w, h, edit, &look.theme, frame_px.clone());
        let mut buf = vec![0u32; (w * h) as usize];
        let mut ui = ui::Ui { cv: gfx::Canvas::new(&mut buf, w, h), f: &mut fonts, ic: &mut ic, t: look.theme, zones: vec![], ptr: None, down: false, anim: false, blink: false, tr: i18n::tr };
        draw::draw(&mut ui, &sn, true, t0, "", w, h);
        drop(ui);
        let rgba: Vec<u8> = buf.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8, 255]).collect();
        let f = out.join(format!("{name}.png"));
        if image::save_buffer(&f, &rgba, w as u32, h as u32, image::ExtendedColorType::Rgba8).is_err() {
            bad += 1;
        }
        println!("{}", f.display());
        for p in yami_px::gfx::fit_report() {
            println!("  layout [{name}]: {p}");
            bad += 1;
        }
    }
    let missed = i18n::take_missed();
    for m in &missed {
        eprintln!("no translation: {m}");
    }
    i32::from(bad > 0 || !missed.is_empty())
}
