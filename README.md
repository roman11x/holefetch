# holefetch

A Linux system fetch tool that automatically themes itself from your wallpaper.

![holefetch screenshot](screenshot.png)

---

## Why holefetch?

Ever run fastfetch and wished it matched your desktop without touching a config file?
holefetch does exactly that, it extracts the dominant colours from your wallpaper
and applies them to the logo and info output automatically.

holefetch is designed for users of traditional desktop environments — GNOME, KDE,
XFCE, MATE, and Cinnamon. If you are running a tiling window manager with a custom
rice, fastfetch gives you the level of control you probably want. holefetch exists
for everyone else — the user who just wants a fetch tool that looks good with their
desktop without any configuration.

> holefetch is inspired by fastfetch and neofetch, and reuses fastfetch's ASCII
> logo collection at build time. If you need deep customisation, fastfetch is the
> right tool. holefetch exists for everyone else.

---

## Desktop Environment Support

| DE        | Wallpaper Detection | Tested |
|-----------|--------------------:|-------:|
| GNOME     | ✓                   | ✓      |
| KDE       | ✓ (best-effort)     | ✗      |
| XFCE      | ✓ (best-effort)     | ✗      |
| MATE      | ✓ (best-effort)     | ✗      |
| Cinnamon  | ✓ (best-effort)     | ✗      |

holefetch was developed and tested on Fedora Linux with GNOME. Support for other
desktop environments is implemented but not personally verified. If you encounter
issues on a specific DE, please open a GitHub issue.