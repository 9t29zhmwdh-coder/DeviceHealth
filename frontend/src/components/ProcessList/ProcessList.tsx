import { useState } from 'react'
import { useAnalysisStore } from '../../stores/analysisStore'
import { api, riskColor, formatBytes, type ProcessEntry, type RiskLevel } from '../../lib/tauri'
import { useT } from '../../lib/i18n'

const RISK_ORDER: RiskLevel[] = ['Critical', 'High', 'Medium', 'Low', 'Unknown', 'Safe']

export function ProcessList() {
  const { processes, showSafe, setShowSafe } = useAnalysisStore()
  const [search, setSearch] = useState('')
  const [sortBy, setSortBy] = useState<'cpu' | 'mem' | 'name' | 'risk'>('cpu')
  const [sortDir, setSortDir] = useState<1 | -1>(-1)
  const [selected, setSelected] = useState<ProcessEntry | null>(null)
  const [explanation, setExplanation] = useState<string | null>(null)
  const [explaining, setExplaining] = useState(false)
  const t = useT()
  const RISK_LABELS: Record<RiskLevel, string> = {
    Critical: t('risk.Critical'), High: t('risk.High'), Medium: t('risk.Medium'),
    Low: t('risk.Low'), Unknown: t('risk.Unknown'), Safe: t('risk.Safe'),
  }

  const q = search.toLowerCase()
  const filtered = processes
    .filter(p => !q || p.name.toLowerCase().includes(q) || (p.description ?? '').toLowerCase().includes(q))
    .sort((a, b) => {
      const factor = sortDir
      if (sortBy === 'cpu') return (a.cpu_usage - b.cpu_usage) * factor
      if (sortBy === 'mem') return (a.memory_bytes - b.memory_bytes) * factor
      if (sortBy === 'risk') return (RISK_ORDER.indexOf(a.risk) - RISK_ORDER.indexOf(b.risk)) * factor
      return a.name.localeCompare(b.name) * factor
    })

  const handleSort = (col: typeof sortBy) => {
    if (col === sortBy) setSortDir(d => (d === -1 ? 1 : -1))
    else { setSortBy(col); setSortDir(-1) }
  }

  const handleExplain = async (p: ProcessEntry) => {
    setSelected(p)
    setExplanation(null)
    setExplaining(true)
    try {
      const result = await api.explainProcess(
        p.name, p.description ?? null, p.cpu_usage, p.memory_bytes / 1024 / 1024
      )
      setExplanation(result)
    } catch {
      setExplanation(t('processList.aiUnavailable'))
    } finally {
      setExplaining(false)
    }
  }

  const SortBtn = ({ col, label }: { col: typeof sortBy; label: string }) => (
    <button onClick={() => handleSort(col)}
      className={`text-xs uppercase tracking-wider transition-colors ${sortBy === col ? 'text-[#58a6ff]' : 'text-[#8b949e] hover:text-[#e6edf3]'}`}>
      {label}{sortBy === col ? (sortDir === -1 ? ' ↓' : ' ↑') : ''}
    </button>
  )

  return (
    <div className="h-full flex overflow-hidden">
      <div className="flex-1 flex flex-col overflow-hidden">
        <div className="p-4 border-b border-[#30363d] flex items-center gap-3">
          <input
            value={search}
            onChange={e => setSearch(e.target.value)}
            placeholder={t('processList.searchPlaceholder')}
            className="flex-1 bg-[#21262d] border border-[#30363d] rounded-md px-3 py-1.5 text-sm text-[#e6edf3] placeholder-[#8b949e] focus:outline-none focus:border-[#58a6ff]"
          />
          <label className="flex items-center gap-2 text-xs text-[#8b949e] cursor-pointer select-none">
            <input type="checkbox" checked={showSafe} onChange={e => setShowSafe(e.target.checked)}
              className="accent-[#58a6ff]" />
            {t('processList.showSafe')}
          </label>
          <span className="text-xs text-[#8b949e]">{filtered.length} {t('processList.processesUnit')}</span>
        </div>

        <div className="px-4 py-2 grid grid-cols-[1fr_80px_80px_80px_70px_32px] gap-2 border-b border-[#30363d]">
          <SortBtn col="name" label={t('processList.name')} />
          <SortBtn col="cpu" label="CPU" />
          <SortBtn col="mem" label="RAM" />
          <SortBtn col="risk" label={t('processList.risk')} />
          <span className="text-xs text-[#8b949e] uppercase tracking-wider">{t('processList.flags')}</span>
          <span />
        </div>

        <div className="flex-1 overflow-y-auto">
          {filtered.map(p => (
            <div key={p.pid}
              onClick={() => handleExplain(p)}
              className={`px-4 py-2 grid grid-cols-[1fr_80px_80px_80px_70px_32px] gap-2 items-center cursor-pointer border-b border-[#21262d] hover:bg-[#161b22] transition-colors
                ${selected?.pid === p.pid ? 'bg-[#161b22]' : ''}`}>
              <div className="min-w-0">
                <div className="flex items-center gap-1.5">
                  {p.is_zombie && <span className="text-[10px] bg-[#f8514920] text-[#f85149] px-1 rounded">Z</span>}
                  {p.is_telemetry && <span className="text-[10px] bg-[#d2992220] text-[#d29922] px-1 rounded">T</span>}
                  <span className="text-sm text-[#e6edf3] truncate">{p.name}</span>
                </div>
                {p.description && (
                  <div className="text-xs text-[#8b949e] truncate">{p.description}</div>
                )}
              </div>
              <span className={`text-xs font-mono ${p.cpu_usage > 50 ? 'text-[#f0883e]' : p.cpu_usage > 20 ? 'text-[#d29922]' : 'text-[#8b949e]'}`}>
                {p.cpu_usage.toFixed(1)}%
              </span>
              <span className="text-xs font-mono text-[#8b949e]">{formatBytes(p.memory_bytes)}</span>
              <span className="text-xs font-medium" style={{ color: riskColor(p.risk) }}>
                {RISK_LABELS[p.risk]}
              </span>
              <div className="flex gap-1 flex-wrap">
                {p.flags.slice(0, 2).map(f => (
                  <span key={f} className="text-[9px] bg-[#30363d] text-[#8b949e] px-1 rounded">{f}</span>
                ))}
              </div>
              <span className="text-xs text-[#8b949e]">▶</span>
            </div>
          ))}
          {filtered.length === 0 && (
            <div className="flex items-center justify-center h-32 text-[#8b949e] text-sm">
              {t('processList.noProcessesFound')}
            </div>
          )}
        </div>
      </div>

      {/* Detail panel */}
      {selected && (
        <div className="w-80 border-l border-[#30363d] flex flex-col bg-[#0d1117] overflow-hidden">
          <div className="p-4 border-b border-[#30363d] flex items-start justify-between">
            <div>
              <div className="text-sm font-medium text-[#e6edf3]">{selected.name}</div>
              <div className="text-xs text-[#8b949e] mt-0.5">{t('processList.pid')} {selected.pid}</div>
            </div>
            <button onClick={() => setSelected(null)} className="text-[#8b949e] hover:text-[#e6edf3] text-lg leading-none">×</button>
          </div>
          <div className="flex-1 overflow-y-auto p-4 space-y-3">
            <InfoRow label={t('processList.risk')} value={RISK_LABELS[selected.risk]} color={riskColor(selected.risk)} />
            <InfoRow label="CPU" value={`${selected.cpu_usage.toFixed(1)}%`} />
            <InfoRow label="RAM" value={formatBytes(selected.memory_bytes)} />
            <InfoRow label={t('processList.status')} value={selected.status} />
            {selected.user && <InfoRow label={t('processList.user')} value={selected.user} />}
            {selected.vendor && <InfoRow label={t('processList.vendor')} value={selected.vendor} />}
            {selected.exe_path && (
              <div>
                <div className="text-xs text-[#8b949e] mb-1">{t('processList.path')}</div>
                <div className="text-xs font-mono text-[#e6edf3] break-all bg-[#161b22] p-2 rounded">{selected.exe_path}</div>
              </div>
            )}
            {selected.flags.length > 0 && (
              <div>
                <div className="text-xs text-[#8b949e] mb-1">{t('processList.flags')}</div>
                <div className="flex flex-wrap gap-1">
                  {selected.flags.map(f => (
                    <span key={f} className="text-xs bg-[#21262d] text-[#8b949e] px-2 py-0.5 rounded">{f}</span>
                  ))}
                </div>
              </div>
            )}
            <div className="border-t border-[#30363d] pt-3">
              <div className="text-xs text-[#8b949e] mb-2">{t('processList.aiExplanation')}</div>
              {explaining ? (
                <div className="flex items-center gap-2 text-xs text-[#8b949e]">
                  <span className="animate-spin">⟳</span> {t('dashboard.analyzing')}
                </div>
              ) : explanation ? (
                <div className="text-xs text-[#e6edf3] leading-relaxed">{explanation}</div>
              ) : (
                <button onClick={() => handleExplain(selected)}
                  className="text-xs text-[#58a6ff] hover:underline">
                  {t('processList.explainProcess')} →
                </button>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

function InfoRow({ label, value, color }: { label: string; value: string; color?: string }) {
  return (
    <div className="flex justify-between text-xs">
      <span className="text-[#8b949e]">{label}</span>
      <span style={color ? { color } : undefined} className="text-[#e6edf3]">{value}</span>
    </div>
  )
}
