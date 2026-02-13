import { showToast, i18n } from '@/main'
import axios from 'axios'
import { refreshToken, type SideEffect } from '../api-schema'
import { formatBp } from '../helpers'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'
import { useUserStore } from '@/stores/user'

const serializeParams = (params: Record<string, unknown>): string => {
  const parts: string[] = []
  for (const [key, value] of Object.entries(params)) {
    if (value === null || value === undefined) continue
    if (Array.isArray(value)) {
      for (const item of value) {
        parts.push(`${encodeURIComponent(key)}[]=${encodeURIComponent(String(item))}`)
      }
    } else {
      parts.push(`${encodeURIComponent(key)}=${encodeURIComponent(String(value))}`)
    }
  }
  return parts.join('&')
}

const api = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
  paramsSerializer: serializeParams,
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
    const sideEffects: [SideEffect] = response.data.side_effects
    const bonusPoints = sideEffects.find((e) => e.type === 'bonus_points')
    if (bonusPoints) {
      const { t } = i18n.global
      const { bonus_points_decimal_places, bonus_points_alias } = usePublicArcadiaSettingsStore()
      useUserStore().bonus_points += bonusPoints.amount
      showToast('', t('side_effects.bonus_points_earned', [formatBp(bonusPoints.amount, bonus_points_decimal_places), bonus_points_alias]), 'success', 4000)
    }
    return response
  },
  async (error) => {
    const originalRequest = error.config
    // We add a custom property `_retry` to the original request config
    // to prevent infinite loops if the refresh token also fails or if
    // a subsequent request with the refreshed token still results in a 401.
    if (error.response && error.response.data === 'jwt token expired' && !originalRequest._retry) {
      originalRequest._retry = true
      const refreshTokenn = localStorage.getItem('refreshToken')!
      if (refreshToken) {
        try {
          const tokens = await refreshToken({
            refresh_token: refreshTokenn,
          })
          localStorage.setItem('token', tokens.token)
          localStorage.setItem('refreshToken', tokens.refresh_token)
          originalRequest.headers.Authorization = `Bearer ${tokens.token}`
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
