export type ConnectingStepStatus = 'pending' | 'loading' | 'done' | 'warning'
export type ConnectingStep = { key: string; label: string; status: ConnectingStepStatus }
export type JobProgressPayload = { steps: ConnectingStep[] }
export type JobCompletePayload = {
  id: number
  name: string
  serverType: string
  hostname: string
  port: number
}
export type JobErrorPayload = { message: string; errorType?: 'networkError' | 'appError' }
