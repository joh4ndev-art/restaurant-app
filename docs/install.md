# Installation Guide

## Supported platforms

- **Linux** — Ubuntu 22.04+, Debian 12+, Linux Mint 21+, Pop!_OS 22.04+, Zorin OS 17+, and derivatives.
- **Any Linux** — via AppImage (universal, no installation required).

Windows and macOS builds are possible but not currently packaged.

## System requirements

| | Minimum | Recommended |
|---|---|---|
| RAM | 2 GB | 4 GB |
| Disk | 200 MB | 500 MB |
| Display | 1024×700 | 1280×800 |
| OS | 64-bit Linux with GTK3 + WebKit2GTK | Ubuntu 22.04 LTS or equivalent |

The app runs comfortably on 10-year-old hardware.

---

## Option 1 — Ubuntu / Debian / Mint / Pop!_OS (recommended)

### 1. Download the `.deb`

Get the latest from the [Releases page](../../releases):

```
Restaurant POS_x.y.z_amd64.deb
```

### 2. Install

```bash
sudo dpkg -i "Restaurant POS_x.y.z_amd64.deb"
```

If there are missing dependencies:

```bash
sudo apt install -f
```

### 3. Launch

Open the applications menu (Super key), search for **"Restaurant POS"**,
and click the icon. Alternatively, run from the terminal:

```bash
restaurant-app
```

### Uninstall

```bash
sudo dpkg -r restaurant-pos
```

Your data is preserved. To remove it too:

```bash
rm -rf ~/.local/share/com.johan.restaurant-app
```

---

## Option 2 — Any Linux (AppImage)

### 1. Download the `.AppImage`

Get the latest from the [Releases page](../../releases):

```
Restaurant POS_x.y.z_amd64.AppImage
```

### 2. Make it executable

```bash
chmod +x "Restaurant POS_x.y.z_amd64.AppImage"
```

### 3. Run

Double-click the file, or run from the terminal:

```bash
./"Restaurant POS_x.y.z_amd64.AppImage"
```

No installation, no `sudo`, no system changes. The AppImage is portable —
you can copy it to a USB stick and run it on any compatible Linux.

---

## First steps after installation

1. **Open the app.** It starts in free mode (no login).
2. Go to **Settings** and configure:
   - Restaurant name, address, phone, and receipt footer.
   - Language (English / Español).
   - Receipt folder.
3. Go to **Products** and create categories and products.
4. Go to **Tables** and create your tables.
5. Start selling from the **POS** view.

For detailed instructions, see the [user manual](user-manual.en.md).

---

## Troubleshooting

### "Command not found: restaurant-app"

The app is installed but not in your PATH. Either open it from the
applications menu, or run:

```bash
/usr/bin/restaurant-app
```

### "Dependency is not satisfiable"

Run:

```bash
sudo apt install -f
```

### The window opens but is blank

Usually a WebView issue. Ensure `webkit2gtk-4.1` is installed:

```bash
sudo apt install libwebkit2gtk-4.1-0
```

### Fonts look wrong

Install common fonts:

```bash
sudo apt install fonts-dejavu fonts-liberation
```

### How do I update to a newer version?

Download the new `.deb` and reinstall:

```bash
sudo dpkg -i "Restaurant POS_NEW_VERSION_amd64.deb"
```

Your data is preserved automatically.