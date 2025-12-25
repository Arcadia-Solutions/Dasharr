export const TRACKER_COLORS: Record<string, string> = {
  Redacted: '#DC2626',
  Orpheus: '#178C36',
  GazelleGames: '#10B981',
  Broadcasthenet: '#F59E0B',
  Anthelion: '#8B5CF6',
  PhoenixProject: '#EC4899',
  AnimeBytes: '#ED106A',
  Blutopia: '#3654D8',
  Aither: '#56BAED',
  LST: '#6366F1',
  OldToons: '#14B8A6',
  ReelFlix: '#F44336',
  ItaTorrents: '#9682DE',
  OnlyEncodes: '#8ADE82',
  SeedPool: '#4991F6',
  YuScene: '#6FC3DF',
  UploadCX: '#8B5A2B',
  FearNoPeer: '#DC2626',
  MyAnonamouse: '#059669',
  Yoinked: '#7C3AED',
  DarkPeers: '#1F2937',
  Rastastugan: '#BE123C',
  HomieHelpDesk: '#0891B2',
  Racing4Everyone: '#2C2C2C',
}

export function getTrackerColor(trackerName: string): string {
  return TRACKER_COLORS[trackerName] ?? '#34D399'
}
