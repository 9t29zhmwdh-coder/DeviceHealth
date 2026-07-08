import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, CartesianGrid } from 'recharts'
import { useAnalysisStore } from '../../stores/analysisStore'
import { gradeColor, gradeLabel, type HealthSnapshot } from '../../lib/tauri'
import { useT, dateLocale } from '../../lib/i18n'

export function HistoryView() {
  const { history } = useAnalysisStore()
  const t = useT()

  if (history.length === 0) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center">
          <div className="text-4xl mb-2">📈</div>
          <div className="text-[#8b949e] text-sm">{t('history.noDataYet')}</div>
          <div className="text-xs text-[#8b949e] mt-1">{t('history.runMultiple')}</div>
        </div>
      </div>
    )
  }

  const chartData = [...history]
    .sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime())
    .map(s => ({
      time: new Date(s.timestamp).toLocaleDateString(dateLocale(), { day: '2-digit', month: '2-digit', hour: '2-digit', minute: '2-digit' }),
      score: s.score,
      cpu: Math.round(s.cpu_usage),
      mem: Math.round(s.memory_used_pct),
    }))

  return (
    <div className="h-full overflow-y-auto p-6 space-y-6">
      <section>
        <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-3">{t('history.scoreHistory')}</h2>
        <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4">
          <ResponsiveContainer width="100%" height={200}>
            <LineChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#21262d" />
              <XAxis dataKey="time" tick={{ fill: '#8b949e', fontSize: 10 }} />
              <YAxis domain={[0, 100]} tick={{ fill: '#8b949e', fontSize: 10 }} />
              <Tooltip
                contentStyle={{ background: '#161b22', border: '1px solid #30363d', borderRadius: 6, fontSize: 12 }}
                labelStyle={{ color: '#8b949e' }}
                itemStyle={{ color: '#58a6ff' }}
              />
              <Line type="monotone" dataKey="score" stroke="#58a6ff" strokeWidth={2} dot={{ r: 3, fill: '#58a6ff' }} name="Score" />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </section>

      <section>
        <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-3">CPU &amp; RAM</h2>
        <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4">
          <ResponsiveContainer width="100%" height={160}>
            <LineChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" stroke="#21262d" />
              <XAxis dataKey="time" tick={{ fill: '#8b949e', fontSize: 10 }} />
              <YAxis domain={[0, 100]} tick={{ fill: '#8b949e', fontSize: 10 }} />
              <Tooltip
                contentStyle={{ background: '#161b22', border: '1px solid #30363d', borderRadius: 6, fontSize: 12 }}
                labelStyle={{ color: '#8b949e' }}
              />
              <Line type="monotone" dataKey="cpu" stroke="#f0883e" strokeWidth={1.5} dot={false} name="CPU %" />
              <Line type="monotone" dataKey="mem" stroke="#79c0ff" strokeWidth={1.5} dot={false} name="RAM %" />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </section>

      <section>
        <h2 className="text-xs font-semibold text-[#8b949e] uppercase tracking-wider mb-3">{t('history.allSnapshots')} ({history.length})</h2>
        <div className="space-y-2">
          {[...history]
            .sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime())
            .map(s => <SnapshotRow key={s.id} snapshot={s} />)}
        </div>
      </section>
    </div>
  )
}

function SnapshotRow({ snapshot: s }: { snapshot: HealthSnapshot }) {
  const t = useT()
  const color = gradeColor(s.grade)
  const total = s.finding_counts.critical + s.finding_counts.high + s.finding_counts.medium
  return (
    <div className="bg-[#161b22] border border-[#30363d] rounded-lg p-3 flex items-center gap-4">
      <div className="w-10 h-10 rounded-full flex items-center justify-center flex-shrink-0 font-bold text-sm"
        style={{ background: color + '20', color }}>
        {s.score}
      </div>
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <span className="text-xs font-medium" style={{ color }}>{gradeLabel(s.grade)}</span>
          <span className="text-xs text-[#8b949e]">{new Date(s.timestamp).toLocaleString(dateLocale())}</span>
        </div>
        <div className="text-xs text-[#8b949e] mt-0.5">
          {s.process_count} {t('history.processesUnit')} · {total} {t('history.findingsUnit')} · CPU {s.cpu_usage.toFixed(0)}% · RAM {s.memory_used_pct.toFixed(0)}%
        </div>
      </div>
    </div>
  )
}
