# DECISIONS.md — Registro de Decisões

> Arquivo que **cresce devagar**. Guarda o PORQUÊ — o que o código sozinho não conta.
> Duas naturezas: **DEC** (decisões de arquitetura/design) e **FIX** (bugs graves resolvidos, para não repetir).
> Não reescreva entradas antigas; se uma decisão for substituída, marque «SUPERADA por DEC-N» e adicione a nova.
> Quando passar de ~700 linhas, mova as mais antigas para `DECISIONS-archive.md`.

---

## DEC-001 — Pesquisar antes de construir
**Data:** 2026-06-26 · **Status:** aceita

### Contexto
O instinto inicial era construir um viewer customizado, mas viewers de imagem são um domínio maduro com dezenas de soluções gratuitas. Construir sem pesquisar seria desperdiçar esforço de desenvolvimento para um problema que pode já estar resolvido.

### Decisão
Fazer pesquisa aprofundada de ferramentas existentes antes de qualquer linha de código. Build só ocorre se a pesquisa confirmar gap real e não coberto pelas alternativas.

### Alternativas consideradas
- **Build first** — custo mínimo de 2–4 semanas para um viewer funcional com qualidade comparável ao ImageGlass; descartado porque a pesquisa levou apenas horas e identificou solução madura.

### Consequências
Pesquisa concluída em 1 sessão. ImageGlass 9.5 cobre ~90% das necessidades com qualidade de produção. O build (Fase 4) permanece como opção condicional se gap real surgir após uso.

---

## DEC-002 — ImageGlass 9.5 + FastStone 8.5 como combinação primária
**Data:** 2026-06-26 · **Status:** aceita

### Contexto
Nenhum viewer único é perfeito para todos os casos de uso: SVG vetorial exige engine web (WebView2/Chromium), enquanto workflows avançados com RAW, batch e edição leve pedem algo mais focado em raster. Dividir responsabilidades evita compromissos.

### Decisão
- **ImageGlass 9.5 (Classic, portable):** padrão para SVG, WebP, PNG, AVIF, GIF, HEIC e todos os formatos modernos. Renderização SVG vetorial via WebView2 — sem pixel no zoom.
- **FastStone 8.5 (portable):** backup para workflows com RAW (Canon CR2/3, Nikon NEF, Sony ARW), batch rename/convert, e operações de edição leve que ImageGlass não oferece. **Não usar para SVG.**

### Alternativas consideradas
- **Só ImageGlass:** cobre SVG/WebP, mas menos poderoso para RAW/batch.
- **Só FastStone:** excelente para raster, mas sem SVG vetorial — o problema original não seria resolvido.
- **PicView:** open source, leve, bom para WebP animado e WebP. SVG pode pixelar (usa Magick.NET). Opção válida mas ImageGlass é mais maduro.
- **ImageGlass 10 Beta 2:** SVG nativo (sem WebView2), melhor stack. Descartado por instabilidade beta.
- **Construir viewer:** overengineering para o momento; revisitar na Fase 3 se gap confirmado.

### Consequências
Dois executáveis portables para manter (sem instalação). ImageGlass no modo SVG perde: rotação, flip, crop, color picker, print — limitação do WebView2 que v10 resolve. FastStone não deve jamais ser default para `.svg`.

---

## DEC-003 — Baixar sempre da fonte oficial; portable em vez de instalador
**Data:** 2026-06-26 · **Status:** aceita

### Contexto
Agregadores de download (Softonic, CNET, UptoDown, FileHippo) frequentemente reempacotam freeware com bundle de adware. O próprio instalador MSI do ImageGlass inclui um prompt para instalar o navegador Opera (patrocínio da Bandisoft, que distribuía o ImageGlass). A versão portable (.zip) não tem esse prompt.

### Decisão
- Sempre baixar de `github.com/d2phap/ImageGlass/releases` (portable `.zip`) para ImageGlass.
- Sempre baixar de `faststone.org` (link direto `FSViewerSetup.exe` ou portable) para FastStone.
- Nunca usar agregadores de download para nenhuma ferramenta deste projeto.
- Preferir portable sempre que disponível: zero rastro no registro, portabilidade, fácil atualização por substituição de pasta.

### Alternativas consideradas
- **MSI do site oficial do ImageGlass:** funciona, mas exige atenção durante o setup para recusar a Opera. Risco real de instalar browser indesejado em clique descuidado.
- **Winget / Chocolatey:** opção legítima (`winget install ImageGlass`). Mantém instalação gerenciada. Trade-off: não é portable, e winget resolve do repositório de manifests que pode estar desatualizado.

### Consequências
Processo de download ligeiramente mais manual (ir ao GitHub Releases) mas garante ausência de bundle e zero rastro no sistema. Atualização é: baixar novo ZIP, extrair sobre a pasta antiga, manter configurações que ficam em `%AppData%\ImageGlass`.
