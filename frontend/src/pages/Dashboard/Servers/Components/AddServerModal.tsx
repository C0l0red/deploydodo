import { useState } from 'react'
import { createPortal } from 'react-dom'
import { AddServerForm } from './AddServerForm'
import { RemoteServerProgress } from './RemoteServerProgress'

type AddServerModalProps = {
  open: boolean
  onClose: () => void
  onSuccess: () => void
}

export function AddServerModal({ open, onClose, onSuccess }: AddServerModalProps) {
  const [error, setError] = useState<string | null>(null)
  const [jobId, setJobId] = useState<string | null>(null)
  const [serverType, setServerType] = useState<'local' | 'remote'>('local')

  function handleClose() {
    setError(null)
    setJobId(null)
    onClose()
  }

  if (!open) return null

  return createPortal(
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="fixed inset-0 bg-black/30" />
      <div className="relative bg-white rounded-xl shadow-xl w-full max-w-2xl mx-4 max-h-[85vh] flex flex-col overflow-hidden">
        {/* Header */}
        <div className="px-6 py-5 border-b border-neutral-100 flex justify-between items-start shrink-0">
          <div className="flex flex-col gap-0.5">
            <h2 className="font-sans font-semibold text-xl leading-7 text-high-contrast m-0">
              Add Server
            </h2>
            <p className="font-sans font-normal text-sm leading-5 text-text-secondary m-0">
              Configure a new server for your deployments.
            </p>
          </div>
          <button
            type="button"
            onClick={handleClose}
            className="size-8 flex items-center justify-center rounded-lg text-text-secondary hover:text-high-contrast hover:bg-neutral-200 transition-colors disabled:opacity-50"
          >
            <svg
              className="size-5"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </div>

        {jobId ? (
          <RemoteServerProgress
            jobId={jobId}
            onSuccess={() => {
              onSuccess()
              handleClose()
            }}
            onGoBack={() => {
              setError(null)
              setJobId(null)
              setServerType('remote')
            }}
          />
        ) : (
          <AddServerForm
            serverType={serverType}
            error={error}
            onError={setError}
            onJobCreated={setJobId}
            onLocalServerCreated={() => {
              onSuccess()
              handleClose()
            }}
            onCancel={handleClose}
          />
        )}
      </div>
    </div>,
    document.body,
  )
}
