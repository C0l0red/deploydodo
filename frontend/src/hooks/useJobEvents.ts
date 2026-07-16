import { useEffect } from 'react'
import type {
  ConnectingStep,
  JobCompletePayload,
  JobErrorPayload,
  JobProgressPayload,
} from '@/api/types'
import { getAuthToken } from '@/api/client'

type UseJobEventsOptions = {
  onProgress: (steps: ConnectingStep[]) => void
  onComplete: (payload: JobCompletePayload) => void
  onError: (payload: JobErrorPayload) => void
}

export function useJobEvents(
  jobId: string,
  { onProgress, onComplete, onError }: UseJobEventsOptions,
) {
  useEffect(() => {
    const es = new EventSource(`/api/jobs/${jobId}/events?token=${getAuthToken()}`)

    es.addEventListener('progress', (e: MessageEvent) => {
      const payload = JSON.parse(e.data) as JobProgressPayload
      onProgress(payload.steps)
    })

    es.addEventListener('complete', (e: MessageEvent) => {
      const payload = JSON.parse(e.data) as JobCompletePayload
      es.close()
      onComplete(payload)
    })

    // Named 'error' events from the server carry a data payload.
    // Native EventSource network errors also fire this listener but have no data.
    es.addEventListener('error', (e: Event) => {
      const data = (e as MessageEvent).data
      es.close()
      if (data) {
        onError(JSON.parse(data) as JobErrorPayload)
      } else {
        onError({
          errorType: 'networkError',
          message: 'Lost connection to server. Please try again.',
        })
      }
    })

    return () => {
      es.close()
    }
  }, [jobId]) // eslint-disable-line react-hooks/exhaustive-deps
}
