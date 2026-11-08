# yami-shot

Скриншоты для Wayland: заморозить экран, выделить, разметить, скопировать, прочитать текст и QR. Часть рабочего стола [yami](https://github.com/Trendorin/yami), которая работает и сама по себе: бери в любой Wayland-райс, шелл yami не нужен.

## Установка

```sh
./install.sh
```

Нужен Rust 1.89 или новее, root не нужен. Ставится в `~/.local`. `./install.sh --deps` — что нужно от системы, `./install.sh --uninstall` — удалить.

## Как пользоваться

Работает на любом композиторе с layer-shell и screencopy. Назначь клавишу: `yami-shot [area|window|screen] [--delay СЕК]`, второй запуск закрывает открытый оверлей. Режиму окна нужен niri. Настройки: `~/.config/yami/shot.json` (`dir`, `copy`, `save`, `loupe`, `notify`, `delay`, `confirm`, `prefix`).

## Внешний вид

Без настроек программа выглядит нейтрально: системный шрифт, скруглённые углы, мягкие тени, светлая
или тёмная тема как в системе. Поменять можно всё:

| Файл | Что |
|---|---|
| `~/.config/yami/style.json` | вид всех модулей yami |
| `~/.config/yami/style.shot.json` | чем этот модуль отличается (только эти ключи) |

| Ключ | Значения |
|---|---|
| `preset` | `plain` (нейтральный) или `yami` (пиксельный) |
| `accent` | `#rrggbb` |
| `light` | `true`, `false`; нет ключа — как в системе |
| `corners` | `notch`, `round`, `square` |
| `radius` | радиус углов панелей в пикселях |
| `shadow` | `hard`, `soft`, `none` |
| `reveal` | `fade`, `none`, `dither`, `scan`, `noise`, `wipe`, `blocks` |
| `font` | семейство шрифта |
| `animations`, `animSpeed` | вкл/выкл, скорость |
| `colors` | любой из `bg panel surface surfaceHi codeBg border dim accent deep glow bright selection text textDim textFaint onAccent textHi ok warn` как `#rrggbb` |
| `lang` | `ru`, `en`, `pt-BR`; пусто — как в системе |

Файлы перечитываются при изменении, программа перекрашивается без перезапуска.

## Лицензия

MIT
