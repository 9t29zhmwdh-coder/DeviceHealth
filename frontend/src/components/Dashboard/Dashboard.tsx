import { RadialBarChart, RadialBar, ResponsiveContainer, PieChart, Pie, Cell, Tooltip } from 'recharts'
import { useAnalysisStore } from '../../stores/analysisStore'
import { api, gradeColor, gradeLabel, severityColor, uptimeHuman, formatBytes, type Severity } from '../../lib/tauri'

type Tab = 'dashboard' | 'processes' | 'findings' | 'hardware' | 'history' | 'settings'
interface Props { onNavigate: (tab: Tab) => void }

const SEVERITY_ORDER: Severity[] = ['Critical', 'High', 'Medium', 'Low', 'Info']
const SEVERITY_LABELS: Record<Severity, string> = {
  Critical: 'Kritisch', High: 'Hoch', Medium: 'Mittel', Low: 'Niedrig', Info: 'Info',
}

export function Dashboard({ onNavigate }: Props) {
  const { snapshot, findings, running, setRunning, setSnapshot, loadAll, ollamaOnline, setOllamaOnline } = useAnalysisStore()

  const handleScan = async () => {
    setRunning(true)
    try {
      const snap = await api.runAnalysis()
      setSnapshot(snap)
      await loadAll()
    } finally {
      setRunning(false)
    }
    const ok = await api.checkOllama().catch(() => false)
    setOllamaOnline(ok)
  }

  const score = snapshot?.score ?? null
  const grade = snapshot?.grade ?? null
  const color = grade ? gradeColor(grade) : '#8b949e'

  const pieData = snapshot ? SEVERITY_ORDER
    .map(s => ({
      name: SEVERITY_LABELS[s],
      value: snapshot.finding_counts[s.toLowerCase() as keyof typeof snapshot.finding_counts],
      color: severityColor(s),
    }))
    .filter(d => d.value > 0) : []

  const radialData = score != null ? [{ value: score, fill: color }] : []

  return (
    <div className="h-full overflow-y-auto p-6">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-xl font-semibold text-[#e6edf3]">Systemübersicht</h1>
          {snapshot && (
            <div className="text-xs text-[#8b949e] mt-0.5">
              Letzte Analyse: {new Date(snapshot.timestamp).toLocaleString('de-CH')}
            </div>
          )}
        </div>
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-1.5">
            <div className={`w-1.5 h-1.5 rounded-full ${ollamaOnline ? 'bg-[#3fb950]' : 'bg-[#f85149]'}`} />
            <span className="text-xs text-[#8b949e]">{ollamaOnline ? 'KI online' : 'KI offline'}</span>
          </div>
          <button
            onClick={handleScan}
            disabled={running}
            className="px-4 py-2 text-sm bg-[#238636] hover:bg-[#2ea043] text-white rounded-lg transition-colors disabled:opacity-50 flex items-center gap-2"
          >
            {running ? (
              <><span className="animate-spin">⟳</span> Analysiere…</>
            ) : (
              '🔍 Systemanalyse starten'
            )}
          </button>
        </div>
      </div>

      {!snapshot ? (
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <div className="text-5xl mb-3">🩺</div>
            <div className="text-[#8b949e] text-sm mb-4">Noch keine Analyse durchgeführt</div>
            <button onClick={handleScan} className="px-6 py-2.5 bg-[#238636] hover:bg-[#2ea043] text-white rounded-lg transition-colors text-sm">
              Jetzt analysieren
            </button>
          </div>
        </div>
      ) : (
        <>
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
            {/* Health Score Gauge */}
            <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-6 flex flex-col items-center">
              <h3 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-4">Gesundheitsscore</h3>
              <div className="relative w-48 h-48">
                <ResponsiveContainer width="100%" height="100%">
                  <RadialBarChart innerRadius="60%" outerRadius="90%" data={radialData} startAngle={180} endAngle={-180}>
                    <RadialBar dataKey="value" cornerRadius={8} background={{ fill: '#21262d' }} />
                  </RadialBarChart>
                </ResponsiveContainer>
                <div className="absolute inset-0 flex flex-col items-center justify-center">
                  <span className="text-4xl font-bold" style={{ color }}>{score}</span>
                  <span className="text-xs mt-1" style={{ color }}>{grade ? gradeLabel(grade) : ''}</span>
                </div>
              </div>
            </div>

            {/* Stats */}
            <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-6">
              <h3 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-4">System</h3>
              <div className="space-y-3">
                <StatRow label="CPU" value={`${snapshot.cpu_usage.toFixed(1)}%`}
                  bar pct={snapshot.cpu_usage} color={snapshot.cpu_usage > 80 ? '#f85149' : '#3fb950'} />
                <StatRow label="RAM" value={`${snapshot.memory_used_pct.toFixed(1)}%`}
                  bar pct={snapshot.memory_used_pct} color={snapshot.memory_used_pct > 85 ? '#f0883e' : '#58a6ff'} />
                <StatRow label="Prozesse" value={String(snapshot.process_count)} />
                <StatRow label="Betriebszeit" value={uptimeHuman(snapshot.uptime_seconds)} />
              </div>
            </div>

            {/* Findings summary */}
            <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-6">
              <h3 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-4">Befunde</h3>
              {pieData.length === 0 ? (
                <div className="flex items-center justify-center h-32 text-[#3fb950] text-sm">
                  ✅ Keine Probleme gefunden
                </div>
              ) : (
                <div className="flex items-center gap-4">
                  <ResponsiveContainer width={100} height={100}>
                    <PieChart>
                      <Pie data={pieData} dataKey="value" cx="50%" cy="50%" outerRadius={45} innerRadius={28}>
                        {pieData.map((d, i) => <Cell key={i} fill={d.color} />)}
                      </Pie>
                    </PieChart>
                  </ResponsiveContainer>
                  <div className="flex-1 space-y-1">
                    {pieData.map(d => (
                      <div key={d.name} className="flex items-center justify-between text-xs">
                        <div className="flex items-center gap-1.5">
                          <div className="w-2 h-2 rounded-full" style={{ background: d.color }} />
                          <span className="text-[#8b949e]">{d.name}</span>
                        </div>
                        <span className="text-[#e6edf3] font-medium">{d.value}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
              <button
                onClick={() => onNavigate('findings')}
                className="mt-3 w-full text-xs text-[#58a6ff] hover:underline text-left"
              >
                Alle Befunde anzeigen →
              </button>
            </div>
          </div>

          {/* Top Findings */}
          {findings.filter(f => f.severity === 'Critical' || f.severity === 'High').length > 0 && (
            <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 mb-6">
              <h3 className="text-sm font-medium text-[#e6edf3] mb-3">🚨 Wichtige Befunde</h3>
              <div className="space-y-2">
                {findings
                  .filter(f => f.severity === 'Critical' || f.severity === 'High')
                  .slice(0, 5)
                  .map(f => (
                    <div key={f.id} className="flex items-start gap-3 p-2.5 bg-[#0d1117] rounded-md">
                      <div className="w-2 h-2 rounded-full mt-1.5 flex-shrink-0"
                        style={{ background: severityColor(f.severity) }} />
                      <div className="flex-1 min-w-0">
                        <div className="text-sm text-[#e6edf3]">{f.title}</div>
                        <div className="text-xs text-[#8b949e] mt-0.5 truncate">{f.recommendation}</div>
                      </div>
                    </div>
                  ))}
              </div>
              <button
                onClick={() => onNavigate('findings')}
                className="mt-3 text-xs text-[#58a6ff] hover:underline"
              >
                Alle Befunde anzeigen →
              </button>
            </div>
          )}
        </>
      )}
    </div>
  )
}

function StatRow({ label, value, bar, pct, color }: {
  label: string; value: string; bar?: boolean; pct?: number; color?: string
}) {
  return (
    <div>
      <div className="flex justify-between text-xs mb-1">
        <span className="text-[#8b949e]">{label}</span>
        <span className="text-[#e6edf3] font-medium">{value}</span>
      </div>
      {bar && pct != null && (
        <div className="h-1.5 bg-[#21262d] rounded-full overflow-hidden">
          <div className="h-full rounded-full transition-all" style={{ width: `${Math.min(pct, 100)}%`, background: color }} />
        </div>
      )}
    </div>
  )
}
