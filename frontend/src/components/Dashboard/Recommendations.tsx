import { useState } from 'react'
import { useAnalysisStore } from '../../stores/analysisStore'
import { api, riskColor, type Recommendation } from '../../lib/tauri'
import { useT } from '../../lib/i18n'

/**
 * The recommendations were computed on every scan but shown nowhere. Quitting
 * asks first: the dialog names the process and what is lost.
 */
export function Recommendations() {
  const { recommendations, processes, runScan } = useAnalysisStore()
  const t = useT()
  const [pending, setPending] = useState<Recommendation | null>(null)
  const [message, setMessage] = useState<{ ok: boolean; text: string } | null>(null)

  if (recommendations.length === 0) return null

  const processOf = (r: Recommendation) => processes.find(p => String(p.pid) === r.target)

  const confirmQuit = async () => {
    const rec = pending
    const proc = rec && processOf(rec)
    setPending(null)
    if (!rec || !proc) return
    try {
      await api.quitProcess(proc.pid, proc.name)
      setMessage({ ok: true, text: t('recommendations.quit').replace('{name}', proc.name) })
      await runScan()
    } catch (e) {
      setMessage({ ok: false, text: String(e) })
    }
  }

  return (
    <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-4 mb-6">
      <h3 className="text-sm font-medium text-[#e6edf3] mb-3">💡 {t('recommendations.title')}</h3>
      {message && (
        <div className={`text-xs mb-3 ${message.ok ? 'text-[#3fb950]' : 'text-[#f85149]'}`}>{message.text}</div>
      )}
      <div className="space-y-2">
        {recommendations.map(r => (
          <div key={r.id} className="flex items-start gap-3 p-2.5 bg-[#0d1117] rounded-md">
            <div className="w-2 h-2 rounded-full mt-1.5 shrink-0" style={{ background: riskColor(r.risk_to_system) }} />
            <div className="flex-1 min-w-0">
              <div className="text-sm text-[#e6edf3]">{r.title}</div>
              <div className="text-xs text-[#8b949e] mt-0.5">{r.description}</div>
            </div>
            {r.action_kind === 'KillProcess' && processOf(r) && (
              <button onClick={() => setPending(r)}
                className="shrink-0 px-3 py-1 text-xs border border-[#30363d] hover:border-[#f85149] hover:text-[#f85149] text-[#e6edf3] rounded-md transition-colors">
                {t('recommendations.quitButton')}
              </button>
            )}
          </div>
        ))}
      </div>

      {pending && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
          <div className="bg-[#161b22] border border-[#30363d] rounded-xl p-6 w-[440px]">
            <h4 className="text-base font-semibold text-[#e6edf3] mb-2">{pending.title}?</h4>
            <p className="text-sm text-[#8b949e] mb-5">{pending.description}</p>
            <div className="flex justify-end gap-2">
              <button onClick={() => setPending(null)} className="px-4 py-2 text-sm text-[#8b949e] hover:text-[#e6edf3]">
                {t('recommendations.cancel')}
              </button>
              <button onClick={confirmQuit} className="px-4 py-2 text-sm bg-[#da3633] hover:bg-[#f85149] text-white rounded-md">
                {t('recommendations.quitButton')}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
