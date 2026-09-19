import { reactive } from 'vue'

/**
 * Remplace window.confirm()/prompt() : WKWebView (macOS) n'implémente pas
 * window.prompt(), et window.confirm() est peu fiable dans les webviews
 * Tauri. On utilise donc nos propres modales, pilotées de façon impérative.
 */

interface ConfirmState {
  visible: boolean
  message: string
  requireTypeConfirm: boolean
  resolve?: (v: boolean) => void
}

interface PromptState {
  visible: boolean
  message: string
  value: string
  resolve?: (v: string | null) => void
}

export const confirmState = reactive<ConfirmState>({
  visible: false,
  message: '',
  requireTypeConfirm: false,
})

export const promptState = reactive<PromptState>({
  visible: false,
  message: '',
  value: '',
})

export function confirmDialog(message: string, requireTypeConfirm = false): Promise<boolean> {
  confirmState.message = message
  confirmState.requireTypeConfirm = requireTypeConfirm
  confirmState.visible = true
  return new Promise((resolve) => {
    confirmState.resolve = resolve
  })
}

export function promptDialog(message: string, defaultValue = ''): Promise<string | null> {
  promptState.message = message
  promptState.value = defaultValue
  promptState.visible = true
  return new Promise((resolve) => {
    promptState.resolve = resolve
  })
}

export function resolveConfirm(value: boolean) {
  confirmState.visible = false
  confirmState.resolve?.(value)
  confirmState.resolve = undefined
}

export function resolvePrompt(value: string | null) {
  promptState.visible = false
  promptState.resolve?.(value)
  promptState.resolve = undefined
}
