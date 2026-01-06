import { showToast } from '@/main'
import axios from 'axios'
import type { LoginResponse } from '../api-schema'

const api = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
})

api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token')
    if (token && !config.url?.includes('/login') && !config.url?.includes('/register')) {
      config.headers.Authorization = `Bearer ${token}`
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
    const originalRequest = error.config
    // We add a custom property `_retry` to the original request config
    // to prevent infinite loops if the refresh token also fails or if
    // a subsequent request with the refreshed token still results in a 401.
    if (error.response && error.response.data === 'jwt token expired' && !originalRequest._retry) {
      originalRequest._retry = true
      const refreshToken = localStorage.getItem('refreshToken')
      if (refreshToken) {
        try {
          const tokens = await api.post<LoginResponse>('/api/auth/refresh-token', {
            refresh_token: refreshToken,
          })
          localStorage.setItem('token', tokens.data.token)
          localStorage.setItem('refreshToken', tokens.data.refresh_token)
          originalRequest.headers.Authorization = `Bearer ${tokens.data.token}`
          return api(originalRequest) // Return the promise of the re-attempted request
        } catch (refreshError) {
          console.error('Failed to refresh token:', refreshError)
          localStorage.removeItem('token')
          localStorage.removeItem('refreshToken')
          window.location.replace('/login')
          return Promise.reject(refreshError)
        }
      }
    }
    if (error.response && error.response.status === 401) {
      localStorage.removeItem('token')
      localStorage.removeItem('refreshToken')
      window.location.replace('/login')
      return new Promise(() => {})
    }
    if (error.response && error.response.data && error.response.data.error) {
      showToast('error', error.response.data.error, 'error', 4000)
    } else {
      showToast('error', 'An unexpected error occurred.', 'error', 4000)
    }
    return Promise.reject(error)
  },
)

export default api
