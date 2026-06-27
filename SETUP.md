# SETUP.md — Como rodar o WinImgView na sua máquina

> Leia do início ao fim antes de rodar qualquer comando.
> Tempo estimado para o primeiro build: 10–20 min (download de dependências Rust).

---

## Pré-requisitos (instalar uma vez)

### 1. Rust + Cargo
Baixe e rode o instalador oficial:
```
https://rustup.rs
```
Clique em "Download rustup-init.exe (64-bit)" → execute → aceite os padrões (opção 1).
Feche e reabra o CMD após instalar.

Verifique:
```
rustc --version
cargo --version
```

### 2. WebView2 Runtime
Já vem instalado no Windows 10 (atualização 1803+) e Windows 11 via Microsoft Edge.
Se não tiver (ex: Windows Server ou install limpo):
```
https://developer.microsoft.com/microsoft-edge/webview2/
```
Baixe "Evergreen Bootstrapper" e execute.

### 3. Node.js (para o Tauri CLI)
Baixe o instalador LTS de:
```
https://nodejs.org
```
Verifique:
```
node --version
npm --version
```

### 4. Build tools do Visual Studio (compilador C++ — necessário para Rust no Windows)
Baixe o Visual Studio Build Tools:
```
https://visualstudio.microsoft.com/visual-cpp-build-tools/
```
Na instalação, selecione: **"Desktop development with C++"** → instalar.

---

## Primeiro build

No CMD, dentro da pasta do projeto (onde está o `package.json`):

```
npm install
npm run build
```

O executável instalável ficará em:
```
src-tauri\target\release\bundle\nsis\WinImgView_0.2.0_x64-setup.exe
```

Ou, para rodar sem instalar (portable):
```
src-tauri\target\release\winimgview.exe
```

---

## Modo desenvolvimento (hot-reload)

```
npm run dev
```

Abre a janela do app com DevTools disponível (F12).
Alterações no frontend (HTML/CSS/JS) refletem ao salvar.
Alterações no Rust exigem recompilação (automática no `tauri dev`).

---

## Associar extensões no Windows (duplo-clique abre no WinImgView)

Após o build, copie `winimgview.exe` para uma pasta fixa, ex:
```
C:\Ferramentas\WinImgView\winimgview.exe
```

Para cada extensão que quiser associar (`.svg`, `.webp`, `.png`, etc.):
1. Clique com botão direito em um arquivo dessa extensão no Explorer
2. "Abrir com" → "Escolher outro aplicativo"
3. "Mais aplicativos" → "Procurar outro aplicativo neste computador"
4. Aponte para `C:\Ferramentas\WinImgView\winimgview.exe`
5. Marque "Sempre usar este aplicativo"

---

## Atalhos de teclado

| Tecla            | Ação                                      |
|------------------|-------------------------------------------|
| `←` / `→`        | Imagem anterior / próxima na pasta        |
| Scroll do mouse  | Zoom (centrado no cursor)                 |
| `Ctrl +`         | Zoom in                                   |
| `Ctrl -`         | Zoom out                                  |
| `Ctrl 0`         | Reset zoom (fit)                          |
| `Ctrl 1`         | Zoom 100%                                 |
| `F11`            | Fullscreen toggle                         |
| `B`              | Ciclo de fundo (escuro / branco / preto / xadrez) |
| `Ctrl Shift C`   | Copia o caminho do arquivo atual          |
| Arrastar arquivo | Abre o arquivo arrastado                  |

---

## Formatos suportados

| Formato | Suporte |
|---------|---------|
| SVG     | ✅ Vetorial real (WebView2) — zoom infinito sem pixel |
| WebP    | ✅ Estático e animado |
| PNG     | ✅ |
| JPG     | ✅ |
| GIF     | ✅ (animado) |
| BMP     | ✅ (via conversão interna) |
| TIFF    | ✅ (via conversão interna) |
| AVIF    | ✅ (via conversão interna) |
| ICO     | ✅ |
