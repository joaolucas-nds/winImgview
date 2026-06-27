# DECISIONS.md — Registro de Decisões

> Arquivo que **cresce devagar**. Guarda o PORQUÊ — o que o código sozinho não conta.
> Duas naturezas: **DEC** (decisões de arquitetura/design) e **FIX** (bugs graves resolvidos, para não repetir).
> Não reescreva entradas antigas; se uma decisão for substituída, marque «SUPERADA por DEC-N» e adicione a nova.

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

## DEC-002 — ImageGlass 9.5 + FastStone 8.5 como combinação de referência
**Data:** 2026-06-26 · **Status:** aceita (referência de pesquisa — superada em uso por DEC-004)

### Contexto
Nenhum viewer único é perfeito: SVG vetorial exige engine web, workflows com RAW pedem algo focado em raster.

### Decisão
- **ImageGlass 9.5 (Classic, portable):** padrão para SVG, WebP, PNG, AVIF, GIF, HEIC.
- **FastStone 8.5 (portable):** backup para RAW, batch rename/convert.

### Alternativas consideradas
- PicView: open source, leve, WebP animado. SVG pode pixelar (usa Magick.NET).
- ImageGlass 10 Beta 2: SVG nativo, mas instável.
- Build viewer: overengineering para o momento → revisitar na Fase 3.

### Consequências
Decisão de referência: como o build customizado (DEC-004) foi ativado, ImageGlass passa a ser referência de qualidade a igualar, não ferramenta adotada.

---

## DEC-003 — Baixar sempre da fonte oficial; portable em vez de instalador
**Data:** 2026-06-26 · **Status:** aceita

### Contexto
Agregadores frequentemente reempacotam freeware com adware. O MSI do ImageGlass inclui prompt Opera.

### Decisão
- ImageGlass: `github.com/d2phap/ImageGlass/releases` (portable `.zip`)
- FastStone: `faststone.org` (portable)
- Nunca agregadores de download.

### Consequências
Zero rastro no sistema. Atualização por substituição de pasta.

---

## DEC-004 — Pular F2/F3 e ir direto ao build customizado (F4)
**Data:** 2026-06-27 · **Status:** aceita

### Contexto
O ROADMAP previa F2 (adotar ImageGlass, testar) e F3 (avaliar gap após uso real) antes de F4 (build). O usuário solicitou iniciar o build imediatamente, sem passar pelas fases de adoção.

### Decisão
Iniciar F4 (build Tauri) diretamente. F2/F3 são formalmente puladas. A pesquisa da F1 serve como base de referência de qualidade (o que o ImageGlass faz bem vira critério de aceitação do build).

### Alternativas consideradas
- **Seguir o ROADMAP original:** adotar ImageGlass por 1–2 semanas antes de codar. Mais conservador, menor risco de construir o que já existe. Descartado por decisão do usuário.
- **Usar ImageGlass como base e forkar:** ImageGlass é MIT, mas é C#/.NET — stack diferente de Tauri/Rust. Curva de aprendizado e divergência de stack justificam build from scratch.

### Consequências
- Build from scratch em Tauri 2.x + Rust + WebView2.
- Critério de aceitação do MVP: igualar o ImageGlass 9.5 nas funcionalidades core (SVG vetorial, WebP, navegação, zoom). Funcionalidades extras (fundo personalizável, copy-to-path, export SVG→PNG) são diferenciais.
- Risco: pode-se descobrir após o build que ImageGlass já cobria tudo — risco aceito explicitamente.

---

## DEC-005 — Três estratégias de renderização no backend
**Data:** 2026-06-27 · **Status:** aceita

### Contexto
Diferentes formatos têm diferentes níveis de suporte no WebView2 embutido do Tauri. Uma estratégia única não serve para todos.

### Decisão
Três estratégias, decididas no backend por extensão do arquivo:

1. **SVG → iframe com asset://** — o iframe garante que o WebView2 processe o XML SVG como vetor, não como imagem rasterizada. `<img src=asset://svg>` rasterizaria.
2. **Raster nativo → img com asset://** — PNG, JPG, WebP, GIF, ICO são suportados nativamente pelo WebView2. Mais simples e eficiente.
3. **Raster não-nativo → base64 PNG** — TIFF, BMP, AVIF não são suportados pelo WebView2. O backend os decodifica com o crate `image` e converte para PNG, enviando como data URL.

### Alternativas consideradas
- **Tudo via base64:** funciona mas é ineficiente para arquivos grandes (PNG 4K → muito token de memória para serializar). Descartado.
- **File URI scheme (`file:///`):** não funciona no Tauri por segurança (CSP bloqueia). Descartado.
- **Crate `resvg` para SVG:** renderizaria SVG nativamente em Rust, mas rasteriza para bitmap. O objetivo é renderização vetorial via WebView2. Descartado para SVG (pode ser útil para thumbnails futuros).

### Consequências
Três code paths no frontend (`renderPayload`). A distinção é feita no backend e comunicada via campo `kind` no payload. Frontend é ignorante do formato real — só precisa saber qual elemento DOM usar e qual src definir.
