import { useState } from 'react'
import { useNavigate } from '@tanstack/react-router'

const PENDING_JOB_KEY = 'deploydodo:pending_job_id'
import type { JobCompletePayload, JobErrorPayload } from '@/api/types'
import { PageLayout } from '@/layouts/PageLayout'
import { Stepper } from './Stepper'
import { SelectView, type ServerOptionId } from './SelectView'
import { LocalServerView } from './LocalServerView'
import { RemoteServerView } from './RemoteServerView'
import { ConnectingView } from './ConnectingView'
import { ConnectionSuccessView } from './ConnectionSuccessView'
import { ConnectionFailedView } from './ConnectionFailedView'
import { SetupCompleteView } from './SetupCompleteView'

type View =
  | 'select'
  | 'local-server'
  | 'remote-server'
  | 'connecting'
  | 'connection-success'
  | 'connection-failed'
  | 'setup-complete'

function stepperProps(view: View) {
  switch (view) {
    case 'select':
    case 'local-server':
    case 'remote-server':
      return { currentStep: 1 }
    case 'connecting':
    case 'connection-failed':
      return { completedSteps: [1], currentStep: 2 }
    case 'connection-success':
      return { completedSteps: [1, 2], nextStep: 3 }
    case 'setup-complete':
      return { completedSteps: [1, 2, 3] }
  }
}

export function SelectServer() {
  const navigate = useNavigate()
  const [jobId, setJobId] = useState<string | null>(() => localStorage.getItem(PENDING_JOB_KEY))
  const [view, setView] = useState<View>(() =>
    localStorage.getItem(PENDING_JOB_KEY) ? 'connecting' : 'select',
  )
  const [server, setServer] = useState<JobCompletePayload | null>(null)
  const [jobError, setJobError] = useState<JobErrorPayload | null>(null)

  function handleSelect(id: ServerOptionId) {
    if (id === 'remote-server') setView('remote-server')
    if (id === 'this-machine') setView('local-server')
  }

  function handleConnect(newJobId: string) {
    localStorage.setItem(PENDING_JOB_KEY, newJobId)
    setJobId(newJobId)
    setJobError(null)
    setView('connecting')
  }

  function handleJobComplete(payload: JobCompletePayload) {
    localStorage.removeItem(PENDING_JOB_KEY)
    setServer(payload)
    setView('connection-success')
  }

  function handleJobError(payload: JobErrorPayload) {
    if (payload.errorType !== 'networkError') {
      localStorage.removeItem(PENDING_JOB_KEY)
      setJobError(payload)
      setView('connection-failed')
    }
  }

  function handleRetry() {
    localStorage.removeItem(PENDING_JOB_KEY)
    setView('remote-server')
  }

  return (
    <PageLayout>
      <Stepper {...stepperProps(view)} />

      {view === 'select' && <SelectView onSelect={handleSelect} />}

      {view === 'local-server' && (
        <LocalServerView
          onBack={() => setView('select')}
          onSuccess={(s) => {
            setServer({ ...s, port: s.port ?? 0 })
            setView('connection-success')
          }}
        />
      )}

      {view === 'remote-server' && (
        <RemoteServerView onBack={() => setView('select')} onConnect={handleConnect} />
      )}

      {view === 'connecting' && jobId && (
        <ConnectingView jobId={jobId} onSuccess={handleJobComplete} onError={handleJobError} />
      )}

      {view === 'connection-success' && server && (
        <ConnectionSuccessView
          server={server}
          subtitle={
            server.serverType === 'local'
              ? 'Local server configured successfully'
              : 'SSH session established and validated'
          }
          onContinue={() => setView('setup-complete')}
        />
      )}

      {view === 'connection-failed' && (
        <ConnectionFailedView
          errorMessage={jobError?.message ?? 'An unexpected error occurred.'}
          onGoBack={() => setView('remote-server')}
          onRetry={handleRetry}
        />
      )}

      {view === 'setup-complete' && (
        <SetupCompleteView onGoToDashboard={() => navigate({ to: '/dashboard' })} />
      )}
    </PageLayout>
  )
}
