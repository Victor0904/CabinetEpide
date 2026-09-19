import { loadMediaUrl } from './mediaCache'
import type { FicheDetail } from './types'

function escHtml(str: string) {
  return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
}

/** Construit le fichier HTML autonome (images embarquées en base64) téléchargé pour une fiche. */
export async function buildFicheHtml(f: FicheDetail, chemin: string): Promise<string> {
  const heroB64 = f.photo_1 ? await loadMediaUrl(f.photo_1).catch(() => null) : null
  const mediaImages = f.medias.filter((m) => m.type === 'image')
  const mediaPdfs = f.medias.filter((m) => m.type !== 'image')
  const mediaB64s = await Promise.all(mediaImages.map((m) => loadMediaUrl(m.chemin).catch(() => null)))

  const date = new Date().toLocaleDateString('fr-FR', { day: '2-digit', month: 'long', year: 'numeric' })

  const imagesSupplHTML = mediaImages
    .map((m, i) =>
      mediaB64s[i]
        ? `<div class="media-item"><img src="${mediaB64s[i]}" alt="${escHtml(m.nom_original ?? '')}"><p>${escHtml(m.nom_original ?? '')}</p></div>`
        : '',
    )
    .join('')

  const pdfsHTML = mediaPdfs.length
    ? `<div class="section"><h3>Documents PDF</h3>${mediaPdfs.map((m) => `<p>📄 ${escHtml(m.nom_original ?? '')}</p>`).join('')}</div>`
    : ''

  return `<!DOCTYPE html>
<html lang="fr">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>${escHtml(f.titre)}</title>
<style>
*{box-sizing:border-box;margin:0;padding:0;font-family:'Segoe UI',Arial,sans-serif}
body{background:#f3f4f6;padding:30px;color:#111827}
.fiche{background:white;max-width:800px;margin:0 auto;border-radius:12px;overflow:hidden;box-shadow:0 4px 20px rgba(0,0,0,.12)}
.fiche-header{background:linear-gradient(135deg,#1e3a8a,#1e40af);color:white;padding:22px 28px}
.chemin{font-size:.75rem;opacity:.65;margin-bottom:6px;letter-spacing:.3px}
h1{font-size:1.5rem;font-weight:800;margin-bottom:8px;line-height:1.2}
.synonymes{font-size:.88rem;opacity:.8;font-style:italic}
.fiche-image{background:#111827;display:flex;justify-content:center;max-height:520px;overflow:hidden}
.fiche-image img{width:100%;max-height:520px;object-fit:contain;display:block}
.fiche-body{padding:28px}
.section{margin-bottom:22px}
.section h3{font-size:.72rem;font-weight:800;text-transform:uppercase;letter-spacing:1.5px;color:#1e3a8a;border-left:3px solid #1e3a8a;padding-left:8px;margin-bottom:10px}
.section p{font-size:.92rem;color:#374151;line-height:1.75;white-space:pre-wrap}
.medias-grid{display:flex;flex-wrap:wrap;gap:12px;margin-top:8px}
.media-item{text-align:center}
.media-item img{width:130px;height:130px;object-fit:cover;border-radius:8px;border:1px solid #e5e7eb;display:block}
.media-item p{font-size:.65rem;color:#6b7280;margin-top:3px;max-width:130px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.footer{text-align:center;padding:14px;font-size:.75rem;color:#9ca3af;border-top:1px solid #e5e7eb;margin-top:10px}
@media print{body{background:white;padding:0}.fiche{box-shadow:none;border-radius:0;max-width:100%}}
</style>
</head>
<body>
<div class="fiche">
  <div class="fiche-header">
    ${chemin ? `<div class="chemin">${escHtml(chemin)}</div>` : ''}
    <h1>${escHtml(f.titre)}</h1>
    ${f.synonymes ? `<div class="synonymes">Synonymes : ${escHtml(f.synonymes)}</div>` : ''}
  </div>
  ${heroB64 ? `<div class="fiche-image"><img src="${heroB64}" alt="${escHtml(f.titre)}"></div>` : ''}
  <div class="fiche-body">
    ${f.descriptif ? `<div class="section"><h3>Commentaire</h3><p>${escHtml(f.descriptif)}</p></div>` : ''}
    ${imagesSupplHTML ? `<div class="section"><h3>Images supplémentaires</h3><div class="medias-grid">${imagesSupplHTML}</div></div>` : ''}
    ${pdfsHTML}
    <div class="footer">Cabinet Épidémiologique &mdash; ${date}</div>
  </div>
</div>
</body>
</html>`
}
