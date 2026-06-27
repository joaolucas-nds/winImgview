# STATUS.md — Estado Atual

> Arquivo **rolante**: descreve só o AGORA. O assistente lê no início para saber onde retomar.
> Item resolvido SAI daqui — vai para o CHANGELOG (se foi entrega) e/ou para o log da sessão.
> Médio e longo prazo NÃO ficam aqui — ficam no ROADMAP.

---

## Versão Atual
**[0.1.0]** — 2026-06-26 — Pesquisa de viewers concluída; ferramentas recomendadas identificadas

## ✅ Funcionando
- Pesquisa aprofundada de viewers Windows para SVG e WebP — concluída (ver HISTORICO.md § 1)
- Tabela comparativa de 8 candidatos montada (CONTEXT.md)
- Stack de build (Tauri) identificada caso Fase 4 seja necessária
- Três decisões de arquitetura registradas (DEC-001 a DEC-003 em DECISIONS.md)

## 🔧 Em Progresso
- **Adoção do ImageGlass 9.5** — a ser baixado, instalado em portable e testado na máquina do usuário

## ❌ Quebrado / Com Problema
- *(nenhum — projeto em fase de pesquisa/setup, nada quebrado ainda)*

## 📋 Backlog (curto prazo — itens acionáveis)
- [ ] Baixar ImageGlass 9.5.0.515 portable (.zip) do GitHub: `github.com/d2phap/ImageGlass/releases` — **não usar o .msi**
- [ ] Extrair para pasta fixa (ex: `C:\Ferramentas\ImageGlass\`) e criar atalho na área de trabalho
- [ ] Configurar ImageGlass como padrão para: `.svg`, `.webp`, `.png`, `.jpg`, `.gif`, `.avif`, `.heic`, `.bmp`, `.tiff`
- [ ] Testar: abrir pasta com SVGs + WebPs mistos → navegar com setas → zoom → verificar que SVG não pixela
- [ ] Testar: abrir `.webp` animado → verificar que animação roda dentro do viewer
- [ ] Opcional: baixar FastStone 8.5 portable (`faststone.org/FSViewerDownload.htm`) como backup para workflow com RAW/batch
- [ ] Avaliar gap após uso real (1–2 semanas): surgiu alguma necessidade que ImageGlass não cobre? → alimenta decisão da Fase 3

## 📁 Arquivos Críticos (não mexer sem contexto)
- `CONTEXT.md` — tabela de viewers, armadilhas e decisão de stack (Tauri); alterar só se stack mudar ou nova armadilha surgir
- `DECISIONS.md` — DEC-001 a DEC-003 já registradas; não reescrever entradas existentes

## 💬 Última Sessão
**2026-06-26** — Pesquisa completa de viewers Windows para SVG e WebP. Identificados 8 candidatos; ImageGlass 9.5 portable (GitHub) eleito como primário (SVG vetorial via WebView2 + WebP + 90+ formatos). FastStone 8.5 como backup raster. Honeyview/BandiView descartados. IrfanView/XnView inadequados para SVG. Próximo passo: baixar ImageGlass portable e testar na máquina.
