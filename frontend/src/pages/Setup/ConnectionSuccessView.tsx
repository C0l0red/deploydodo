import { CheckCircleOutlineIcon } from '@/assets/icons'
import type { JobCompletePayload } from '@/api/types'
import { Card } from '@/layouts/PageLayout'

function InfoTile({ label, value }: { label: string; value: string }) {
  return (
    <div className="border border-neutral-100 rounded-lg p-4 flex flex-col items-center gap-1 flex-1">
      <span className="font-manrope font-bold text-base leading-6 text-high-contrast text-center">
        {label}
      </span>
      <span className="font-manrope font-normal text-sm leading-5 text-high-contrast text-center">
        {value}
      </span>
    </div>
  )
}

export function ConnectionSuccessView({
  server,
  onContinue,
  subtitle = 'SSH session established and validated',
}: {
  server: JobCompletePayload
  onContinue: () => void
  subtitle?: string
}) {
  return (
    <Card className="p-10">
      <div className="flex flex-col items-center gap-6">
        <CheckCircleOutlineIcon />

        <div className="flex flex-col items-center gap-2">
          <h2 className="font-sans font-semibold text-2xl leading-8 text-high-contrast m-0">
            Connection Successful!
          </h2>
          <p className="font-sans font-normal text-lg leading-7 text-high-contrast m-0">
            {subtitle}
          </p>
        </div>

        <div className="flex flex-col gap-6 w-120">
          <div className="flex gap-6">
            <InfoTile label="Server Name" value={server.name} />
            <InfoTile label="Host" value={server.hostname} />
          </div>
          <div className="flex gap-6">
            <InfoTile label="SSH Port" value={server.port ? String(server.port) : '—'} />
            <InfoTile label="Type" value={server.serverType} />
          </div>
        </div>

        <button
          type="button"
          onClick={onContinue}
          className="w-120 bg-secondary text-pure-white font-manrope font-bold text-sm leading-6 rounded-lg px-4 py-2 hover:opacity-[0.88] active:opacity-75 transition-opacity duration-150"
        >
          Finish Setup
        </button>
      </div>
    </Card>
  )
}
