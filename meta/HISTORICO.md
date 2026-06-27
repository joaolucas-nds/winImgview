# HISTORICO.md — Conhecimento Consolidado

> **Opcional.** Arquivo-baú para conhecimento denso que já foi aprendido e não muda mais.
> Não é lido no início da sessão; o assistente consulta sob demanda quando o assunto aparecer.

---

## 1. Pesquisa de Viewers Windows para SVG/WebP (2026-06-26)

### O problema central do SVG em viewers nativos

SVG (Scalable Vector Graphics) é um formato XML que descreve formas, caminhos, gradientes e animações de forma matemática — sem pixels. Para exibir com qualidade, o viewer precisa **interpretar esse XML e desenhar na tela em tempo real**, como um browser faz. Isso é chamado de "renderização vetorial".

A maioria dos viewers de imagem tradicionais (IrfanView, FastStone, XnView, Honeyview) foi construída para **formatos raster** (PNG, JPG, GIF) — arquivos que são grades de pixels. Quando encontram um SVG, fazem o que sabem: **convertem o SVG para um bitmap de tamanho fixo** (geralmente na dimensão original do arquivo) e então exibem esse bitmap. Resultado: funciona bem no zoom 100%, mas qualquer zoom acima disso mostra pixels — exatamente o oposto do propósito do SVG.

A única solução real é ter um **engine web** (Chromium, Gecko, WebKit) renderizando o SVG. Daí as duas abordagens que funcionam:
1. Abrir no browser — funciona, mas tira do contexto de trabalho e não permite navegar para a próxima imagem da pasta.
2. Embutir um engine web dentro do viewer — o que ImageGlass 9.5 faz via WebView2.

### Tabela Comparativa Completa (pesquisa jun/2026)

| Viewer | Versão (jun/2026) | SVG | WebP anim. | Open Source | Portable | Gratuito | Observação crítica |
|---|---|---|---|---|---|---|---|
| ImageGlass | 9.5.0.515 (stable) | ✅ Vetorial (WebView2) | ✅ | ✅ MIT | ✅ | ✅ Classic | **Primário recomendado** |
| ImageGlass | 10.0 Beta 2 | ✅ Vetorial nativo | ✅ | ✅ MIT | ✅ | ✅ Classic | Beta — aguardar GA |
| PicView | 3.6.x | ⚠️ Magick (rasteriza) | ✅ | ✅ GPL | ✅ | ✅ | SVG pode pixelar; WebP excelente |
| FastStone | 8.5 (2026-06-24) | ❌ Não suporta | ❌ só estático | ❌ | ✅ | ✅ pessoal | **Melhor para RAW/raster/batch** |
| IrfanView | 4.73 | ❌ Rasteriza | ✅ c/ plugin | ❌ | ✅ | ✅ pessoal | Fórum oficial sugere Chrome para SVG |
| XnView MP | 1.8.x | ⚠️ Rasteriza | ✅ | ❌ | ✅ | ✅ pessoal | SVG perde transparência, sem galeria |
| Honeyview | 5.53 (mai/2024) | ❌ | ⚠️ | ❌ | ✅ | ✅ | **DESCONTINUADO** — substituto BandiView é ad-supported |
| Smart SVG Viewer | 0.x Alpha | ✅ Vetorial | ❌ | ❌ | ✅ | ✅ | Só SVG, instável, sem desenvolvimento ativo |
| Nomacs | 3.17 | ⚠️ Qt rasteriza | ✅ | ✅ | ✅ | ✅ | SVG rasterizado; sem suporte ativo recente |
| qView | 6.1 | ⚠️ Qt rasteriza | ✅ | ✅ | ✅ | ✅ | Minimalista; SVG rasterizado |

### Análise por caso de uso

**"Preciso abrir SVG com zoom perfeito (sem pixel)"**
→ ImageGlass 9.5 portable (GitHub). Ponto final. Nenhuma outra opção gratuita faz isso com qualidade de produção no Windows em jun/2026.

**"Preciso de WebP animado"**
→ ImageGlass 9.5 ou PicView. FastStone 8.5 ainda não suporta WebP animado (suporta estático).

**"Tenho pastas de fotos RAW (Canon, Nikon, Sony)"**
→ FastStone 8.5. Melhor suporte RAW, edição leve, conversão batch, renomear em lote.

**"Quero algo ultra-leve e open source para WebP/PNG/JPG"**
→ PicView. Mais leve que ImageGlass, zero tracking, WebP animado, boa galeria.

**"Quero tudo em um único programa"**
→ ImageGlass 9.5. Não é perfeito para RAW e batch, mas cobre 95% dos casos de uso cotidianos.

### Por que Honeyview foi descontinuado?

A Bandisoft (criadora do Honeyview e do BandiZip) decidiu em 2024 descontinuar o Honeyview e lançar o **BandiView** como substituto. BandiView surgiu como gratuito mas rapidamente passou a ser trial/freemium com limitações e anúncios. Honeyview 5.53 (mai/2024) foi a última versão e não terá mais atualizações — sem correções de segurança, sem novos formatos. Evitar ambos.

### Por que IrfanView é mencionado mas não é opção para SVG?

IrfanView é um dos viewers mais antigos e respeitados para Windows (desde 1996), mas foi construído inteiramente para formatos raster. O suporte a SVG existe via plugin mas usa uma biblioteca raster (libsvg ou similar) que converte para bitmap. O próprio fórum oficial do IrfanView recomenda abrir SVGs no browser para qualidade máxima. Para PNG, JPG, GIF, ICO, BMP e dezenas de formatos raster, IrfanView ainda é excelente.

### Detalhes do ImageGlass 10 Beta

ImageGlass 10 representa uma reescrita significativa da engine de renderização. A principal novidade é a renderização SVG **nativa** — sem depender do WebView2. Isso resolve a limitação do v9 (onde SVG desabilitava rotação, flip, crop, etc. por estar em modo WebView2). A versão Beta 2 saiu em jun/2026. Pontos negativos: ainda é beta, o canal de relatórios de bugs está ativo, e algumas features do v9 estão sendo portadas. Recomendação: usar v9.5 para produção e testar v10 Beta em paralelo numa instalação separada.

### Download seguro — links validados (jun/2026)

- **ImageGlass 9.5 portable:** `https://github.com/d2phap/ImageGlass/releases` → baixar `ImageGlass_9.x.x.xxx_x64_portable.zip`
- **FastStone 8.5 portable:** `https://www.faststone.org/FSViewerDownload.htm` → "Portable version"
- **PicView portable:** `https://github.com/Ruben2776/PicView/releases` → baixar o `.zip`

Nunca usar: Softonic, CNET/Download.com, UptoDown, FileHippo, MajorGeeks (risco de bundle). Usar sempre GitHub oficial ou site oficial do projeto.
