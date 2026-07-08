import { useState } from 'react'
import { useAnalysisStore } from '../../stores/analysisStore'
import { severityColor, type Severity, type Finding } from '../../lib/tauri'
import { useT, dateLocale } from '../../lib/i18n'

const SEVERITIES: Severity[] = ['Critical', 'High', 'Medium', 'Low', 'Info']

export function FindingsView() {
  const { findings } = useAnalysisStore()
  const [filter, setFilter] = useState<Severity | 'all'>('all')
  const [search, setSearch] = useState('')
  const [expanded, setExpanded] = useState<string | null>(null)
  const t = useT()
  const SEV_LABELS: Record<Severity, string> = {
    Critical: t('severity.Critical'), High: t('severity.High'), Medium: t('severity.Medium'),
    Low: t('severity.Low'), Info: t('severity.Info'),
  }

  const q = search.toLowerCase()
  const filtered = findings.filter(f => {
    if (filter !== 'all' && f.severity !== filter) return false
    if (q && !f.title.toLowerCase().includes(q) && !f.description.toLowerCase().includes(q)) return false
    return true
  })

  const counts = Object.fromEntries(
    SEVERITIES.map(s => [s, findings.filter(f => f.severity === s).length])
  ) as Record<Severity, number>

  return (
    <div className="h-full flex flex-col overflow-hidden">
      <div className="p-4 border-b border-[#30363d]">
        <div className="flex items-center gap-2 mb-3 flex-wrap">
          <button onClick={() => setFilter('all')}
            className={`px-3 py-1 rounded-full text-xs font-medium transition-colors ${filter === 'all' ? 'bg-[#30363d] text-[#e6edf3]' : 'text-[#8b949e] hover:text-[#e6edf3]'}`}>
            {t('findingsView.all')} ({findings.length})
          </button>
          {SEVERITIES.map(s => counts[s] > 0 && (
            <button key={s} onClick={() => setFilter(s)}
              className={`px-3 py-1 rounded-full text-xs font-medium transition-colors border ${filter === s ? 'text-white' : 'text-[#8b949e] hover:text-[#e6edf3] border-transparent'}`}
              style={filter === s ? { background: severityColor(s), borderColor: severityColor(s) } : { borderColor: 'transparent' }}>
              {SEV_LABELS[s]} ({counts[s]})
            </button>
          ))}
        </div>
        <input
          value={search}
          onChange={e => setSearch(e.target.value)}
          placeholder={t('findingsView.searchPlaceholder')}
          className="w-full bg-[#21262d] border border-[#30363d] rounded-md px-3 py-1.5 text-sm text-[#e6edf3] placeholder-[#8b949e] focus:outline-none focus:border-[#58a6ff]"
        />
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-2">
        {filtered.length === 0 ? (
          <div className="flex items-center justify-center h-32">
            <div className="text-center">
              <div className="text-3xl mb-2">{findings.length === 0 ? '✅' : '🔍'}</div>
              <div className="text-[#8b949e] text-sm">
                {findings.length === 0 ? t('findingsView.noFindingsGood') : t('findingsView.noMatches')}
              </div>
            </div>
          </div>
        ) : (
          filtered.map(f => <FindingCard key={f.id} finding={f}
            open={expanded === f.id}
            onToggle={() => setExpanded(expanded === f.id ? null : f.id)}
            labels={SEV_LABELS} />)
        )}
      </div>
    </div>
  )
}

function FindingCard({ finding: f, open, onToggle, labels }: {
  finding: Finding; open: boolean; onToggle: () => void; labels: Record<Severity, string>
}) {
  const t = useT()
  const color = severityColor(f.severity)
  return (
    <div className="bg-[#161b22] border border-[#30363d] rounded-lg overflow-hidden">
      <button onClick={onToggle} className="w-full text-left p-4 hover:bg-[#21262d] transition-colors">
        <div className="flex items-start gap-3">
          <div className="w-2 h-2 rounded-full mt-1.5 flex-shrink-0" style={{ background: color }} />
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-0.5">
              <span className="text-xs font-medium px-1.5 py-0.5 rounded" style={{ color, background: color + '20' }}>
                {labels[f.severity as Severity]}
              </span>
              <span className="text-xs text-[#8b949e]">{f.kind}</span>
            </div>
            <div className="text-sm font-medium text-[#e6edf3]">{f.title}</div>
            {!open && <div className="text-xs text-[#8b949e] mt-0.5 truncate">{f.description}</div>}
          </div>
          <span className="text-[#8b949e] text-xs">{open ? '▲' : '▼'}</span>
        </div>
      </button>
      {open && (
        <div className="px-4 pb-4 border-t border-[#30363d] space-y-3 pt-3">
          <div>
            <div className="text-xs text-[#8b949e] mb-1">{t('findingsView.description')}</div>
            <div className="text-sm text-[#e6edf3]">{f.description}</div>
          </div>
          <div>
            <div className="text-xs text-[#8b949e] mb-1">{t('findingsView.affectedItem')}</div>
            <code className="text-xs bg-[#0d1117] text-[#79c0ff] px-2 py-1 rounded block">{f.affected_item}</code>
          </div>
          <div>
            <div className="text-xs text-[#8b949e] mb-1">{t('findingsView.recommendation')}</div>
            <div className="text-sm text-[#3fb950]">{f.recommendation}</div>
          </div>
          <div className="text-xs text-[#8b949e]">
            {new Date(f.timestamp).toLocaleString(dateLocale())}
          </div>
        </div>
      )}
    </div>
  )
}
