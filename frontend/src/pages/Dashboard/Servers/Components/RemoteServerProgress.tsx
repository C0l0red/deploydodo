import { Button } from '@/components/Button'
import { useJobEvents } from '@/hooks/useJobEvents'
import type { ConnectingStep } from '@/api/types'
import { SpinnerIcon, CheckCircleIcon, WarningCircleIcon } from '@/assets/icons'
import { cn } from '@/utilities/cn'
import { useState } from 'react'

export function RemoteServerProgress({
  jobId,
  onSuccess,
  onGoBack,
}: {
  jobId: string
  onSuccess: () => void
  onGoBack: () => void
}) {
  const [steps, setSteps] = useState<ConnectingStep[]>([])
  const [internalError, setInternalError] = useState<string | null>(null)

  useJobEvents(jobId, {
    onProgress: (newSteps) => setSteps(newSteps),
    onComplete: () => {
      setTimeout(() => onSuccess(), 1500)
    },
    onError: (err) => {
      setInternalError(err.message)
    },
  })

  return (
    <div className="flex-1 px-6 py-8 flex flex-col gap-6 items-center justify-center min-h-75">
      {internalError ? (
        <div className="flex flex-col items-center gap-4 text-center">
          <WarningCircleIcon className="size-12 text-error!" />
          <div className="flex flex-col gap-2">
            <h3 className="font-sans font-semibold text-lg text-high-contrast m-0">
              Connection Failed
            </h3>
            <p className="font-sans text-sm text-text-secondary m-0 max-w-sm">{internalError}</p>
          </div>
          <Button type="button" onClick={() => onGoBack()} variant="ghost" className="mt-2">
            Go Back
          </Button>
        </div>
      ) : (
        <div className="flex flex-col gap-4 max-w-sm">
          {steps.map((step) => (
            <div key={step.key} className="flex items-center gap-3">
              <div className="size-6 flex items-center justify-center shrink-0">
                {step.status === 'done' ? (
                  <CheckCircleIcon className="size-5 text-[#00C16A]" />
                ) : step.status === 'loading' ? (
                  <SpinnerIcon className="size-5 text-secondary animate-spin" />
                ) : step.status === 'warning' ? (
                  <WarningCircleIcon className="size-5 text-error" />
                ) : (
                  <div className="size-2 rounded-full bg-neutral-200" />
                )}
              </div>
              <span
                className={cn(
                  'font-sans text-sm transition-colors',
                  step.status === 'done'
                    ? 'text-high-contrast font-medium'
                    : step.status === 'loading'
                      ? 'text-secondary font-medium'
                      : 'text-text-secondary',
                )}
              >
                {step.label}
              </span>
            </div>
          ))}
          {steps.length === 0 && (
            <div className="flex items-center justify-center py-4">
              <SpinnerIcon className="size-6 text-secondary animate-spin" />
            </div>
          )}
        </div>
      )}
    </div>
  )
}
