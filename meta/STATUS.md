# STATUS.md — Estado Atual

> Arquivo **rolante**: descreve só o AGORA. O assistente lê no início para saber onde retomar.
> Item resolvido SAI daqui — vai para o CHANGELOG (se foi entrega) e/ou para o log da sessão.
> Médio e longo prazo NÃO ficam aqui — ficam no ROADMAP.

---

## Versão Atual
**[0.2.0]** — 2026-06-27 — Scaffolding MVP do viewer Tauri implementado

## ✅ Funcionando (código gerado, aguarda build na máquina do usuário)
- Estrutura Tauri 2.x completa: `Cargo.toml`, `tauri.conf.json`, `capabilities/`, `build.rs`
- Backend Rust (`lib.rs`): `open_file`, `next_image`, `prev_image`, `get_position`
- Navegação de pasta: lista imagens, ordena por nome, wrap-around nas bordas
- Estratégia de renderização por tipo: SVG → iframe asset://, raster nativo → img asset://, TIFF/BMP/AVIF → base64 PNG
- Frontend HTML/CSS/JS: zoom (roda + teclado), pan (arrastar), drag & drop, atalhos, ciclo de fundo, barra de status
- Abertura via argv[1] (duplo-clique no Explorer emite evento `image-loaded` para o frontend)
- SETUP.md com guia completo de instalação

## 🔧 Em Progresso
- **Build na máquina do usuário** — rodar `npm install && npm run build` para gerar o `.exe`

## ❌ Quebrado / Com Problema
- *(nada ainda — código não foi compilado/testado na máquina real)*

## 📋 Backlog (curto prazo — itens acionáveis)
- [ ] Instalar pré-requisitos (Rust, Node.js, Build Tools C++) conforme SETUP.md
- [ ] `npm install && npm run build` na pasta do projeto
- [ ] Testar: duplo-clique num SVG → abre viewer → zoom → sem pixel
- [ ] Testar: WebP animado → animação roda dentro do viewer
- [ ] Testar: navegação ← → entre imagens de uma pasta
- [ ] Testar: drag & drop de arquivo para a janela
- [ ] Testar: `B` alterna fundo, `Ctrl+Shift+C` copia caminho, `F11` fullscreen
- [ ] Verificar ícone de aplicativo (adicionar .ico real em `src-tauri/icons/`)
- [ ] Corrigir bugs encontrados após teste real
- [ ] Registrar extensões no Windows (SETUP.md § "Associar extensões")

## 📁 Arquivos Críticos (não mexer sem contexto)
- `src-tauri/src/lib.rs` — toda a lógica de backend: comandos Tauri, navegação, carga de imagem
- `frontend/js/main.js` — toda a lógica de frontend: zoom, pan, atalhos, renderização
- `src-tauri/tauri.conf.json` — configuração do app (identifier, janela, bundle)
- `src-tauri/Cargo.toml` — dependências Rust

## 💬 Última Sessão
**2026-06-27** — Decisão de pular F2/F3 e ir direto ao build customizado (DEC-004). Scaffolding completo do MVP gerado: backend Rust com navegação de pasta e 3 estratégias de renderização, frontend com zoom/pan/drag-drop/atalhos. Aguarda primeiro build e testes na máquina do usuário.
