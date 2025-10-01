// import { showToast } from '@/main'
import axios from 'axios'

const api = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || window.location.origin + '/api',
  timeout: 120000,
  headers: {
    'Content-Type': 'application/json',
  },
})

api.interceptors.request.use(
  (config) => {
    const api_key = localStorage.getItem('api_key')
    if (api_key) {
      config.headers['api-key'] = api_key
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  },
)

api.interceptors.response.use(
  (response) => {
    return response
  },
  async (error) => {
    // We add a custom property `_retry` to the original request config
    // to prevent infinite loops if the refresh token also fails or if
    // a subsequent request with the refreshed token still results in a 401.
    if (error.response && error.response.status === 401) {
      localStorage.removeItem('api_key')
      window.location.replace('/login')
      return new Promise(() => {})
    }
    // if (error.response && error.response.data && error.response.data.error) {
    //   showToast('error', error.response.data.error, 'error', 4000)
    // } else {
    //   showToast('error', 'An unexpected error occurred.', 'error', 4000)
    // }
    return Promise.reject(error)
  },
)

export default api
