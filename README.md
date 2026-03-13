![twitter-image](https://github.com/user-attachments/assets/b6161cfd-ddfa-4c09-9fbb-ab5a2d6961fc)

<p align="center">
  <p align="center">Char - The AI notepad for <strong>private</strong> meetings</p>
  <p align="center">
   <a href="https://deepwiki.com/fastrepl/char"><img src="https://deepwiki.com/badge.svg" alt="Ask DeepWiki"></a>
   <a href="https://char.com/discord" target="_blank"><img src="https://img.shields.io/static/v1?label=Join%20our&message=Discord&color=blue&logo=Discord" alt="Discord"></a>
   <a href="https://x.com/getcharnotes" target="_blank"><img src="https://img.shields.io/static/v1?label=Follow%20us%20on&message=X&color=black&logo=x" alt="X"></a>
  </p>
</p>
   
## What is Char?

Char is an AI notetaking app specifically designed to take meeting notes. With Char, you can transcribe all kinds of meetings whether it be online or offline.

- **Listens** to your meetings so you can only jot down important stuff
- **No bots** joining your meetings - Char listens directly to sounds coming in & out of your computer
- Crafts perfect **summaries** based on your memos, right after the meeting is over
- You can run Char completely **offline** by using LM Studio or Ollama

You can also use it for taking notes for lectures or organizing your thoughts.

## Installation

### macOS

```bash
brew install --cask fastrepl/fastrepl/char
```

Or download directly from [char.com/download](https://char.com/download) (public beta).

### Ubuntu / Linux

**1. Install system dependencies**

```bash
sudo apt install -y \
  build-essential pkg-config \
  libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev \
  libasound2-dev libpulse-dev \
  libjavascriptcoregtk-4.1-dev libsoup-3.0-dev \
  patchelf
```

**2. Install Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**3. Install Node.js (v22+) and pnpm**

```bash
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs
corepack enable && corepack prepare pnpm@10.32.1 --activate
```

**4. Build and run**

```bash
pnpm install
pnpm -F @hypr/desktop tauri:dev
```

### Windows

**1. Install prerequisites**

- [Visual Studio Build Tools](https://aka.ms/vs/17/release/vs_BuildTools.exe) — select the **Desktop development with C++** workload
- [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) — required by Tauri
- [Rust](https://rustup.rs)
- [Node.js v22+](https://nodejs.org) and pnpm (`npm install -g pnpm`)

**2. Build and run**

```bash
pnpm install
pnpm -F @hypr/desktop tauri:dev
```

### Platform feature matrix

| Feature | macOS | Linux | Windows |
|---------|-------|-------|---------|
| Audio capture (mic + speaker) | ✅ CoreAudio | ✅ PulseAudio/PipeWire | ✅ WASAPI |
| Audio device management | ✅ | ✅ | ✅ |
| Microphone usage detection | ✅ | ✅ | ✅ stub |
| Note-taking & editor | ✅ | ✅ | ✅ |
| Local transcription (STT) | ✅ | ✅ | ✅ |
| Language / locale detection | ✅ | ✅ | ✅ |
| Sleep / wake detection | ✅ | ✅ | ✅ |
| Sync, Google/Outlook calendar | ✅ | ✅ | ✅ |
| Apple Calendar & Contacts | ✅ | ❌ | ❌ |
| Zoom mute detection | ✅ | ❌ | ❌ |
| Dock menu | ✅ | ❌ | ❌ |

## Highlights

### Notepad

Char is designed to take notes easily during meetings. Just jot down stuff you think are important!

<img width="732" height="612" alt="Screenshot 2025-11-23 at 2 38 20 PM" src="https://github.com/user-attachments/assets/268ab859-a194-484b-b895-bc640df18dd4" />

### Realtime Transcript

While you stay engaged in the conversation, Char captures every detail so you don't have to type frantically.

<img width="688" height="568" alt="Screenshot 2025-11-23 at 2 35 47 PM" src="https://github.com/user-attachments/assets/e63ce73f-1a5f-49ce-a14d-dd8ba161e5bc" />

### From Memos to Summaries

Once the meeting is over, Char will craft a personalized summary based on your memos — which is not mandatory. Char will still create great summaries without your notes.

![offline enhancing-1](https://github.com/user-attachments/assets/13af787b-2f6e-4877-b90f-719edc45fb75)

### Truly Local

If you noticed the GIF above, you can see that Char works without any internet connection available. Just set up LM Studio or Ollama to operate Char in air-gapped environments!

<img width="780" height="585" alt="no-wifi" src="https://github.com/user-attachments/assets/ecf08a9e-3b6c-4fb6-ab38-0bc572f54859" />

> **Note on accounts:** During onboarding, Char creates an account so you can experience the full product — including cloud-powered transcription and summarization — at its best quality. All your notes, transcripts, and data are stored locally on your machine in a local SQLite database. If you prefer not to keep an account, you can request deletion anytime at [char.com/app/account](https://char.com/app/account). Char will continue to work fully offline with a local LLM.

### Bring Your Own LLM

Prefer something custom? You can swap in your own language model:

- Run local models via Ollama
- Use approved third-party APIs like Gemini, Claude, or Azure-hosted GPT
- Stay compliant with whatever your org allows

Char plays nice with whatever stack you're running.

<img width="912" height="712" alt="Screenshot 2025-11-23 at 2 41 03 PM" src="https://github.com/user-attachments/assets/a6552c99-acbc-4d47-9d21-7f1925989344" />

### Note Templates

Prefer a certain style? Choose from predefined templates like bullet points, agenda-based, or paragraph summary. Or create your own.

Check out our [template gallery](https://char.com/templates) and add your own [here](https://github.com/fastrepl/char/tree/main/apps/web/content/templates).

### AI Chat

Ask follow-ups right inside your notes:

- "What were the action items?"
- "Rewrite this in simpler language"
- "Translate to Spanish"

<img width="959" height="712" alt="image" src="https://github.com/user-attachments/assets/52b7dc14-906f-445f-91f9-b0089d40a495" />

### Integrations

- Apple Calendar, Contacts
- Obsidian
- Coming soon: Notion, Slack, Hubspot, Salesforce

<img width="912" height="712" alt="image" src="https://github.com/user-attachments/assets/ab559e54-fda5-4c8c-97d7-ba1b9d134cc8" />

## Development

### Tech stack

| Layer | Technology |
|-------|-----------|
| Desktop shell | [Tauri 2](https://tauri.app) (Rust + WebView) |
| Frontend | React 19, TanStack Router, TipTap editor |
| Local data store | TinyBase (SQLite-backed) |
| UI state | Zustand |
| Forms / queries | TanStack Form + TanStack Query |
| Styling | Tailwind CSS v4 |
| Web app | TanStack Start (SSR) |
| API server | Axum (Rust) |
| Audio / STT | CPAL, PulseAudio, WASAPI, CoreAudio, Whisper, Cactus |

### Commands

| Command | Description |
|---------|-------------|
| `pnpm -F @hypr/desktop tauri:dev` | Run desktop app in dev mode |
| `pnpm -F @hypr/web dev` | Run web app in dev mode |
| `pnpm exec dprint fmt` | Format all code |
| `pnpm -r typecheck` | TypeScript type-check all packages |
| `cargo check` | Rust type-check all crates |

### Repository layout

```
char/
├── apps/
│   ├── desktop/        # Tauri desktop app (React + Rust)
│   │   └── src-tauri/  # Rust backend, Tauri config
│   ├── web/            # Marketing & web app (TanStack Start)
│   └── api/            # REST API server (Axum)
├── packages/
│   ├── store/          # TinyBase schema — central data model
│   ├── ui/             # Shared component library
│   └── utils/          # Shared utilities (cn, etc.)
├── crates/             # 130+ Rust library crates
│   ├── audio*/         # Cross-platform audio capture
│   ├── detect/         # Mic/app/sleep detection (all platforms)
│   └── transcribe*/    # Whisper / Cactus STT pipeline
└── plugins/            # 46+ Tauri plugins
```

### Code conventions

- Format via `dprint` after every change (`pnpm exec dprint fmt`)
- TypeScript: use `useForm` (TanStack Form) and `useQuery`/`useMutation` (TanStack Query) — no manual state for forms/mutations
- Classnames: always use `cn` from `@hypr/utils`
- Animation: use `motion/react`, not `framer-motion`
- Comments: only where the logic is non-obvious; explain *why*, not *what*
