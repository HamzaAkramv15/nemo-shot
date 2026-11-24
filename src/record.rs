// Запись экрана: `nemo-shot record [screen|area] [--audio] [--delay СЕК]` начинает, повторный `nemo-shot record` (или `record stop`)
// заканчивает. Пишет wf-recorder (тот же screencopy, что у grim): видео в папку снимков, в имени .mp4. Выбор области — slurp, если он есть.

use crate::cfg::Cfg;
use crate::i18n::select;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn state_file() -> PathBuf {
    PathBuf::from(std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into())).join("nemo-record.pid")
}

fn alive(pid: i32) -> bool {
    // SAFETY: сигнал 0 только проверяет, что процесс есть
    unsafe { libc::kill(pid, 0) == 0 }
}

/// pid и файл идущей записи
pub fn running() -> Option<(i32, String)> {
    let t = std::fs::read_to_string(state_file()).ok()?;
    let (pid, file) = t.trim().split_once(' ')?;
    let pid: i32 = pid.parse().ok()?;
    alive(pid).then(|| (pid, file.to_string()))
}

fn say(summary: &str, body: &str) {
    crate::notify::send(summary, body, None, &[], 0);
}

fn out_dir(cfg: &Cfg) -> PathBuf {
    if !cfg.dir.is_empty() {
        return PathBuf::from(&cfg.dir);
    }
    nemo_px::paths::home().join("Videos")
}

/// монитор выхода по умолчанию (то, что играет): имя приёмника из pactl + «.monitor»
fn default_monitor() -> Option<String> {
    let o = Command::new("pactl").arg("get-default-sink").output().ok()?;
    let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
    (o.status.success() && !s.is_empty()).then(|| format!("{s}.monitor"))
}

pub fn run(args: &[String], cfg: &Cfg) -> i32 {
    let mut area = false;
    let mut audio = false;
    let mut delay = 0u64;
    let mut stop = false;
    let mut output = None;
    let mut region = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "area" => area = true,
            "screen" => {}
            "stop" => stop = true,
            "--audio" => audio = true,
            "--delay" => delay = it.next().and_then(|d| d.parse().ok()).unwrap_or(0),
            // из оверлея: монитор и область в общих координатах ("x,y WxH")
            "--output" => output = it.next().cloned(),
            "--region" => region = it.next().cloned(),
            other => {
                eprintln!("nemo-shot record: unknown argument: {other}");
                return 2;
            }
        }
    }
    // уже идёт: любое повторное обращение останавливает
    if let Some((pid, file)) = running() {
        // SAFETY: SIGINT — штатный способ закончить wf-recorder: он дописывает файл
        unsafe { libc::kill(pid, libc::SIGINT) };
        for _ in 0..50 {
            if !alive(pid) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let _ = std::fs::remove_file(state_file());
        say(select("запись сохранена", "recording saved"), &file);
        return 0;
    }
    if stop {
        say(select("запись не идёт", "no recording in progress"), "");
        return 1;
    }
    if !nemo_px::paths::find_exe("wf-recorder").is_some() {
        say(select("запись экрана не установлена", "screen recording is not installed"), select("нужна программа wf-recorder (sudo dnf install wf-recorder)", "needs wf-recorder (sudo dnf install wf-recorder)"));
        return 1;
    }
    let mut geom = region;
    if area && geom.is_none() {
        match Command::new("slurp").output() {
            Ok(o) if o.status.success() && !o.stdout.is_empty() => geom = Some(String::from_utf8_lossy(&o.stdout).trim().to_string()),
            Ok(_) => return 1, // выбор отменили
            Err(_) => {
                say(select("запись области не установлена", "area recording is not installed"), select("для выбора области нужна программа slurp", "choosing an area needs slurp"));
                return 1;
            }
        }
    }
    if delay > 0 {
        std::thread::sleep(std::time::Duration::from_secs(delay));
    }
    let dir = out_dir(cfg);
    let _ = std::fs::create_dir_all(&dir);
    let tm = nemo_px::time::local_now();
    let file = dir.join(format!("{}-{:04}-{:02}-{:02}_{:02}-{:02}-{:02}.mp4", cfg.prefix, tm.year, tm.mon, tm.day, tm.hour, tm.min, tm.sec));
    let mut c = Command::new("wf-recorder");
    c.arg("-f").arg(&file);
    // звук: флаг --audio или настройка recordAudio (mic — источник по умолчанию, system — монитор выхода по умолчанию)
    match (audio, cfg.record_audio.as_str()) {
        (_, "system") => match default_monitor() {
            Some(d) => {
                c.arg(format!("--audio={d}"));
            }
            None => {
                c.arg("--audio");
            }
        },
        (true, _) | (_, "mic") => {
            c.arg("--audio");
        }
        _ => {}
    }
    if let Some(g) = &geom {
        c.args(["-g", g]);
    } else if let Some(o) = &output {
        c.args(["-o", o]);
    }
    c.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    match c.spawn() {
        Ok(child) => {
            let _ = std::fs::write(state_file(), format!("{} {}", child.id(), file.display()));
            say(select("запись идёт", "recording"), select("тот же жест — остановить", "run the same shortcut to stop"));
            0
        }
        Err(e) => {
            say(select("запись не началась", "recording did not start"), &e.to_string());
            1
        }
    }
}
