# stoic-cli

A beautiful Stoic philosophy TUI — read Marcus Aurelius, Seneca & Epictetus in your terminal.

## Demo

![stoic-cli demo](assets/demo.gif)

## Install

```bash
npm install -g stoic-cli
```

Or via shell script:
```bash
curl -fsSL https://raw.githubusercontent.com/whoisyurii/stoic-cli/main/install.sh | sh
```

## Usage

### Interactive TUI
```bash
stoic
```

### Read a specific passage
```bash
stoic read meditations 4:3
```

### Read a full book/letter
```bash
stoic read seneca 13
```

### Read a range of sections
```bash
stoic read discourses 1:2-5
```

### Search all stoic texts
```bash
stoic search "virtue"
```

### Random stoic passage
```bash
stoic random
```

### Daily stoic passage
```bash
stoic daily
```

### Replay intro animation
```bash
stoic intro
```

When you run `stoic` with no arguments, it launches a full-screen terminal browser:

- **Three-panel layout:** Works → Books/Letters → Text
- **Six languages:** English, Русский, Français, Deutsch, Latina, Ἑλληνικά
- **Five themes:** Obsidian, Marble, Parchment, Bronze, Terminal
- **Keyboard-driven:** Navigate with arrows/hjkl, search with `/`
- **Fully offline:** All texts bundled in the binary

## Keybindings

| Key | Action |
|-----|--------|
| `←→` / `hl` | Switch panels |
| `↑↓` / `jk` | Navigate items |
| `Enter` | Select / Open |
| `/` | Search |
| `t` | Cycle theme |
| `v` | Cycle language |
| `qq` | Quit |

## Supported Texts

| Work | Author | Original |
|------|--------|----------|
| Meditations | Marcus Aurelius | Greek |
| Discourses | Epictetus | Greek |
| Moral Letters | Seneca | Latin |

## License

MIT
