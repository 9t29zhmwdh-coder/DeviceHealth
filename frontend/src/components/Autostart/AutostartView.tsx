import { useAnalysisStore } from '../../stores/analysisStore'
import { riskColor } from '../../lib/tauri'
import { useT } from '../../lib/i18n'

/** The autostart scan only ever fed a "more than 30 entries" finding; this lists what it found. */
export function AutostartView() {
  const { autostart, snapshot } = useAnalysisStore()
  const t = useT()

  return (
    <div className="h-full overflow-y-auto p-6">
      <h1 className="text-xl font-semibold text-[#e6edf3] mb-1">{t('autostart.title')}</h1>
      <p className="text-sm text-[#8b949e] mb-6 max-w-3xl">{t('autostart.hint')}</p>
      {!snapshot ? (
        <div className="text-sm text-[#8b949e]">{t('dashboard.noAnalysisYet')}</div>
      ) : autostart.length === 0 ? (
        <div className="text-sm text-[#8b949e]">{t('autostart.empty')}</div>
      ) : (
        <div className="bg-[#161b22] border border-[#30363d] rounded-xl divide-y divide-[#21262d]">
          {autostart.map(e => (
            <div key={e.id} className="flex items-center gap-4 px-4 py-2.5">
              <div className="w-2 h-2 rounded-full shrink-0" style={{ background: riskColor(e.risk) }} />
              <div className="flex-1 min-w-0">
                <div className="text-sm text-[#e6edf3] truncate">{e.name}</div>
                <div className="text-xs text-[#8b949e] truncate" title={e.command}>{e.command}</div>
              </div>
              <span className="text-xs text-[#8b949e] shrink-0">{e.location}</span>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
