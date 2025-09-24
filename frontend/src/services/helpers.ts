import type { NewIndexer } from './api/indexerService'

export const getConfigurableIndexers = (): NewIndexer[] => {
  return [
    {
      name: 'Redacted',
      auth_data: {
        api_key: {
          value: '',
          explanation: 'Get it from your profile\'s settings, in "Access Settings"',
        },
      },
    },
  ]
}
