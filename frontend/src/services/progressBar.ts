import { ref } from 'vue'

const CREEP_INTERVAL_MS = 200
// let the view start its requests before considering the page loaded
const SETTLE_DELAY_MS = 150
const HIDE_DELAY_MS = 300

export const barVisible = ref(false)
export const progress = ref(0)

let creepTimer: ReturnType<typeof setInterval> | null = null
let settleTimer: ReturnType<typeof setTimeout> | null = null
let hideTimer: ReturnType<typeof setTimeout> | null = null
let pendingRequests = 0
let navigating = false

const finish = () => {
  if (!barVisible.value) return
  if (creepTimer) {
    clearInterval(creepTimer)
    creepTimer = null
  }
  progress.value = 100
  hideTimer = setTimeout(() => {
    barVisible.value = false
    progress.value = 0
    hideTimer = null
  }, HIDE_DELAY_MS)
}

const scheduleSettle = () => {
  if (settleTimer) clearTimeout(settleTimer)
  settleTimer = setTimeout(() => {
    settleTimer = null
    if (!navigating && pendingRequests === 0) finish()
  }, SETTLE_DELAY_MS)
}

/** Called when the url changes, the bar then stays until the page stops fetching data. */
export const startPageProgress = () => {
  navigating = true
  if (settleTimer) {
    clearTimeout(settleTimer)
    settleTimer = null
  }
  if (hideTimer) {
    clearTimeout(hideTimer)
    hideTimer = null
  }
  progress.value = 0
  barVisible.value = true
  if (!creepTimer) {
    // asymptotic creep, never reaches 100 on its own
    creepTimer = setInterval(() => (progress.value += (90 - progress.value) * 0.15), CREEP_INTERVAL_MS)
  }
}

export const endPageNavigation = () => {
  navigating = false
  scheduleSettle()
}

export const trackRequestStart = () => (pendingRequests += 1)

export const trackRequestEnd = () => {
  pendingRequests = Math.max(0, pendingRequests - 1)
  if (pendingRequests === 0) scheduleSettle()
}
