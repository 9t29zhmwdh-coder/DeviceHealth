import { create } from 'zustand'

export type Lang = 'en' | 'de'

const STORAGE_KEY = 'devicehealth_lang'

interface Dict {
  [key: string]: string | Dict
}

const translations: Record<Lang, Dict> = {
  en: {
    nav: {
      dashboard: 'Overview',
      findings: 'Findings',
      processes: 'Processes',
      hardware: 'Hardware',
      history: 'History',
      settings: 'Settings',
    },
    severity: {
      Critical: 'Critical', High: 'High', Medium: 'Medium', Low: 'Low', Info: 'Info',
    },
    risk: {
      Critical: 'Critical', High: 'High', Medium: 'Medium', Low: 'Low', Unknown: 'Unknown', Safe: 'Safe',
    },
    grade: {
      Excellent: 'Excellent', Good: 'Good', Fair: 'Fair', Poor: 'Poor', Critical: 'Critical',
    },
    dashboard: {
      title: 'System Overview',
      lastAnalysis: 'Last analysis',
      aiOnline: 'AI online',
      aiOffline: 'AI offline',
      analyzing: 'Analyzing...',
      startScan: 'Start system analysis',
      noAnalysisYet: 'No analysis performed yet',
      analyzeNow: 'Analyze now',
      healthScore: 'Health Score',
      system: 'System',
      processesLabel: 'Processes',
      uptime: 'Uptime',
      findings: 'Findings',
      noIssuesFound: 'No issues found',
      viewAllFindings: 'View all findings',
      importantFindings: 'Important findings',
    },
    findingsView: {
      all: 'All',
      searchPlaceholder: 'Search findings...',
      noFindingsGood: 'No findings, system looks good!',
      noMatches: 'No matches',
      description: 'Description',
      affectedItem: 'Affected item',
      recommendation: 'Recommendation',
    },
    hardware: {
      noAnalysisYet: 'No analysis performed yet',
      system: 'System',
      os: 'Operating System',
      hostname: 'Hostname',
      cpu: 'CPU',
      cpuCores: 'CPU Cores',
      cpuFreq: 'CPU Clock',
      cpuUsage: 'CPU Usage',
      memory: 'Memory',
      disks: 'Disks',
      temperatures: 'Temperatures',
      network: 'Network',
      interface: 'Interface',
      received: 'Received',
      sent: 'Sent',
    },
    history: {
      noDataYet: 'No analysis data yet',
      runMultiple: 'Run several analyses to see the trend',
      scoreHistory: 'Score History',
      allSnapshots: 'All Snapshots',
      processesUnit: 'processes',
      findingsUnit: 'findings',
    },
    processList: {
      searchPlaceholder: 'Search process...',
      showSafe: 'Show safe processes',
      processesUnit: 'processes',
      name: 'Name',
      risk: 'Risk',
      flags: 'Flags',
      noProcessesFound: 'No processes found',
      pid: 'PID',
      status: 'Status',
      user: 'User',
      vendor: 'Vendor',
      path: 'Path',
      aiExplanation: 'AI explanation',
      explainProcess: 'Explain process',
      aiUnavailable: 'AI unavailable. Make sure Ollama is running.',
    },
    settings: {
      aiBackend: 'AI Backend (Ollama)',
      ollamaUrl: 'Ollama URL',
      model: 'Model',
      modelPlaceholder: 'e.g. llama3, mistral',
      testing: 'Testing...',
      testConnection: 'Test connection',
      connected: 'Connected',
      unreachable: 'Unreachable',
      analysis: 'Analysis',
      autoScanOnStartup: 'Analyze automatically on startup',
      showSafeProcesses: 'Show safe processes',
      scanInterval: 'Analysis interval (minutes)',
      thresholds: 'Thresholds',
      cpuWarning: 'CPU warning (%)',
      memWarning: 'RAM warning (%)',
      diskWarning: 'Disk warning (%)',
      tempWarning: 'Temperature warning (°C)',
      history: 'History',
      keepHistoryDays: 'Keep history (days)',
      cleanupNow: 'Clean up old history now',
      save: 'Save settings',
      saved: 'Saved',
    },
  },
  de: {
    nav: {
      dashboard: 'Übersicht',
      findings: 'Befunde',
      processes: 'Prozesse',
      hardware: 'Hardware',
      history: 'Verlauf',
      settings: 'Einstellungen',
    },
    severity: {
      Critical: 'Kritisch', High: 'Hoch', Medium: 'Mittel', Low: 'Niedrig', Info: 'Info',
    },
    risk: {
      Critical: 'Kritisch', High: 'Hoch', Medium: 'Mittel', Low: 'Niedrig', Unknown: 'Unbekannt', Safe: 'Sicher',
    },
    grade: {
      Excellent: 'Ausgezeichnet', Good: 'Gut', Fair: 'Akzeptabel', Poor: 'Schlecht', Critical: 'Kritisch',
    },
    dashboard: {
      title: 'Systemübersicht',
      lastAnalysis: 'Letzte Analyse',
      aiOnline: 'KI online',
      aiOffline: 'KI offline',
      analyzing: 'Analysiere...',
      startScan: 'Systemanalyse starten',
      noAnalysisYet: 'Noch keine Analyse durchgeführt',
      analyzeNow: 'Jetzt analysieren',
      healthScore: 'Gesundheitsscore',
      system: 'System',
      processesLabel: 'Prozesse',
      uptime: 'Betriebszeit',
      findings: 'Befunde',
      noIssuesFound: 'Keine Probleme gefunden',
      viewAllFindings: 'Alle Befunde anzeigen',
      importantFindings: 'Wichtige Befunde',
    },
    findingsView: {
      all: 'Alle',
      searchPlaceholder: 'Befunde durchsuchen...',
      noFindingsGood: 'Keine Befunde, System sieht gut aus!',
      noMatches: 'Keine Treffer',
      description: 'Beschreibung',
      affectedItem: 'Betroffenes Element',
      recommendation: 'Empfehlung',
    },
    hardware: {
      noAnalysisYet: 'Noch keine Analyse durchgeführt',
      system: 'System',
      os: 'Betriebssystem',
      hostname: 'Hostname',
      cpu: 'CPU',
      cpuCores: 'CPU-Kerne',
      cpuFreq: 'CPU-Takt',
      cpuUsage: 'CPU-Auslastung',
      memory: 'Arbeitsspeicher',
      disks: 'Laufwerke',
      temperatures: 'Temperaturen',
      network: 'Netzwerk',
      interface: 'Interface',
      received: 'Empfangen',
      sent: 'Gesendet',
    },
    history: {
      noDataYet: 'Noch keine Analysedaten vorhanden',
      runMultiple: 'Führe mehrere Analysen durch, um den Verlauf zu sehen',
      scoreHistory: 'Score-Verlauf',
      allSnapshots: 'Alle Snapshots',
      processesUnit: 'Prozesse',
      findingsUnit: 'Befunde',
    },
    processList: {
      searchPlaceholder: 'Prozess suchen...',
      showSafe: 'Sichere Prozesse anzeigen',
      processesUnit: 'Prozesse',
      name: 'Name',
      risk: 'Risiko',
      flags: 'Flags',
      noProcessesFound: 'Keine Prozesse gefunden',
      pid: 'PID',
      status: 'Status',
      user: 'Nutzer',
      vendor: 'Hersteller',
      path: 'Pfad',
      aiExplanation: 'KI-Erklärung',
      explainProcess: 'Prozess erklären',
      aiUnavailable: 'KI nicht verfügbar. Stelle sicher, dass Ollama läuft.',
    },
    settings: {
      aiBackend: 'KI-Backend (Ollama)',
      ollamaUrl: 'Ollama URL',
      model: 'Modell',
      modelPlaceholder: 'z.B. llama3, mistral',
      testing: 'Prüfe...',
      testConnection: 'Verbindung testen',
      connected: 'Verbunden',
      unreachable: 'Nicht erreichbar',
      analysis: 'Analyse',
      autoScanOnStartup: 'Beim Start automatisch analysieren',
      showSafeProcesses: 'Sichere Prozesse anzeigen',
      scanInterval: 'Analyseintervall (Minuten)',
      thresholds: 'Schwellenwerte',
      cpuWarning: 'CPU-Warnung (%)',
      memWarning: 'RAM-Warnung (%)',
      diskWarning: 'Festplatten-Warnung (%)',
      tempWarning: 'Temperatur-Warnung (°C)',
      history: 'Verlauf',
      keepHistoryDays: 'Verlauf behalten (Tage)',
      cleanupNow: 'Alten Verlauf jetzt bereinigen',
      save: 'Einstellungen speichern',
      saved: 'Gespeichert',
    },
  },
}

interface LangState {
  lang: Lang
  setLang: (lang: Lang) => void
  toggle: () => void
}

export const useLangStore = create<LangState>((set) => ({
  lang: (localStorage.getItem(STORAGE_KEY) as Lang) || 'en',
  setLang: (lang) => {
    localStorage.setItem(STORAGE_KEY, lang)
    set({ lang })
  },
  toggle: () => set((s) => {
    const next: Lang = s.lang === 'en' ? 'de' : 'en'
    localStorage.setItem(STORAGE_KEY, next)
    return { lang: next }
  }),
}))

export function getLang(): Lang {
  return useLangStore.getState().lang
}

function resolve(dict: Dict, path: string): string {
  const parts = path.split('.')
  let node: string | Dict | undefined = dict
  for (const p of parts) {
    node = typeof node === 'object' ? node[p] : undefined
  }
  return typeof node === 'string' ? node : path
}

export function t(path: string): string {
  return resolve(translations[getLang()], path)
}

export function useT() {
  const lang = useLangStore((s) => s.lang)
  return (path: string) => resolve(translations[lang], path)
}

export function dateLocale(): string {
  return getLang() === 'de' ? 'de-CH' : 'en-US'
}
