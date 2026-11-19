// Снимок окна на niri без оверлея. Окна для выбора мышью берут положение плитки из `niri msg windows`, а
// niri 26.04 его не отдаёт (tile_pos_in_workspace_view = null у всех окон), так что выбор окна давно
// не находил ни одного. Вместо этого просим сам niri: `screenshot-window` снимает сфокусированное окно
// точно (со скруглёнными углами, даже если оно частично за экраном) и кладёт его в буфер обмена; отсюда мы
// берём PNG, пишем в папку по настройкам и говорим об этом тем же уведомлением. None — niri нет или не
// вышло: тогда работает обычный оверлей.

use crate::cfg::Cfg;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use nemo_px::paths::find_exe;

/// картинка из буфера обмена (PNG); None — там не картинка
fn clipboard_png() -> Option<Vec<u8>> {
    let o = Command::new("wl-paste").args(["--no-newline", "--type", "image/png"]).stderr(Stdio::null()).output().ok()?;
    (o.status.success() && o.stdout.starts_with(b"\x89PNG")).then_some(o.stdout)
}

/// один снимок окна; Some(код выхода), если этим путём всё сделано
pub fn run(cfg: &Cfg) -> Option<i32> {
    if std::env::var_os("NIRI_SOCKET").is_none() || find_exe("niri").is_none() || find_exe("wl-paste").is_none() {
        return None;
    }
    let before = clipboard_png();
    let ok = Command::new("niri")
        .args(["msg", "action", "screenshot-window", "--write-to-disk", "false", "--show-pointer", "false"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success());
    if !ok {
        return None;
    }
    // niri кладёт картинку в буфер на следующем кадре: ждём, пока она там изменится
    let t0 = Instant::now();
    let png = loop {
        if let Some(p) = clipboard_png() {
            if before.as_ref() != Some(&p) {
                break p;
            }
        }
        if t0.elapsed() > Duration::from_millis(2500) {
            return None;
        }
        std::thread::sleep(Duration::from_millis(60));
    };
    let mut saved = None;
    if cfg.save || !cfg.copy {
        let dir = cfg.folder();
        let path = dir.join(crate::snap::file_name(&cfg.prefix));
        let _ = std::fs::create_dir_all(&dir);
        match std::fs::write(&path, &png) {
            Ok(()) => saved = Some(path),
            Err(e) => {
                crate::notify::send(crate::i18n::tr("скриншот не сохранился"), &e.to_string(), None, &[], 0);
                return Some(1);
            }
        }
    }
    if cfg.notify {
        let summary = if saved.is_some() { crate::i18n::tr("скриншот сохранён") } else { crate::i18n::tr("скриншот в буфере обмена") };
        let body = saved.as_ref().map(|p| nemo_px::paths::pretty(p)).unwrap_or_default();
        crate::notify::send(summary, &body, None, &[], 0);
    }
    Some(0)
}
