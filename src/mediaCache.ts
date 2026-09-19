import { api } from './api'

const cache = new Map<string, Promise<string>>()

export function loadMediaUrl(chemin: string): Promise<string> {
  let entry = cache.get(chemin)
  if (!entry) {
    entry = api.readMedia(chemin)
    cache.set(chemin, entry)
  }
  return entry
}

export function invalidateMedia(chemin: string) {
  cache.delete(chemin)
}
