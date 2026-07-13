# Lucid Spell

[Download latest release](https://github.com/l1ngus/lucid-spell/releases)

A desktop translator powered by LLMs. Built with Tauri 2 + React + TypeScript + Tailwind CSS.

## Features

- Translate text using any OpenAI-compatible API (remote or local)
- Language auto-detection via `whatlang`
- Text-to-speech with Microsoft Edge TTS voices
- Custom dictionaries & flashcards
- Proxy support (SOCKS5/HTTP/HTTPS)
- Keyboard shortcuts

## Stack

| Layer | Tech |
|-------|------|
| Frontend | React 19, TypeScript, Tailwind CSS 4, TanStack Query, Radix UI |
| Backend | Rust, Tauri 2, async-openai, rodio, msedge-tts |
| State | Zustand stores + React contexts |
