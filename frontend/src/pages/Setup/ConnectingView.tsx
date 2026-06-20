import { useState } from 'react'
import {
  CheckCircleHollowIcon,
  SpinnerIcon,
  PendingSpinnerIcon,
  WarningTriangleIcon,
} from '@/assets/icons'
import { useJobEvents } from '@/hooks/useJobEvents'
import type {
  ConnectingStep,
  ConnectingStepStatus,
  JobCompletePayload,
  JobErrorPayload,
} from '@/api/types'
import { Card } from '@/layouts/PageLayout'

function CheckListItem({ label, status }: { label: string; status: ConnectingStepStatus }) {
  return (
    <div className="flex items-center gap-2">
      {status === 'done' && (
        <div className="relative shrink-0 size-6">
          <CheckCircleHollowIcon />
        </div>
      )}
      {status === 'loading' && (
        <div className="shrink-0 size-6 flex items-center justify-center">
          <SpinnerIcon className="size-4.5 animate-spin" />
        </div>
      )}
      {status === 'pending' && (
        <div className="relative shrink-0 size-6">
          <PendingSpinnerIcon className="size-full" />
        </div>
      )}
      {status === 'warning' && (
        <div className="relative shrink-0 size-6 flex items-center justify-center">
          <WarningTriangleIcon className="size-5 text-amber-500" />
        </div>
      )}
      <span
        className={[
          'font-manrope text-sm leading-6',
          status === 'done' ? 'font-bold text-secondary' : 'font-normal text-text-secondary',
        ].join(' ')}
      >
        {label}
      </span>
    </div>
  )
}

export function ConnectingView({
  jobId,
  onSuccess,
  onError,
}: {
  jobId: string
  onSuccess: (server: JobCompletePayload) => void
  onError: (errorPayload: JobErrorPayload) => void
}) {
  const [steps, setSteps] = useState<ConnectingStep[]>([])

  useJobEvents(jobId, {
    onProgress: (incoming) => setSteps(incoming),
    onComplete: onSuccess,
    onError: onError,
  })

  return (
    <Card className="p-10">
      <div className="flex flex-col items-center gap-2">
        <div className="flex flex-col items-center gap-8">
          <h2 className="font-sans font-semibold text-2xl leading-8 text-high-contrast m-0">
            Connecting to Server
          </h2>
          <p className="font-sans font-normal text-base leading-6 text-high-contrast m-0">
            Establishing and validating SSH connection…
          </p>
        </div>
        <div className="flex flex-col gap-5 w-81.75">
          {steps.map((step) => (
            <CheckListItem key={step.key} label={step.label} status={step.status} />
          ))}
        </div>
      </div>
    </Card>
  )
}
