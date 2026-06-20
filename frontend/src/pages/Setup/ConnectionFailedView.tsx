import { WarningTriangleIcon } from '@/assets/icons'
import { Card } from '@/layouts/PageLayout'

export function ConnectionFailedView({
  errorMessage,
  onGoBack,
  onRetry,
}: {
  errorMessage: string
  onGoBack: () => void
  onRetry: () => void
}) {
  return (
    <Card className="p-10">
      <div className="flex flex-col items-center gap-8">
        <div className="flex flex-col items-center gap-2">
          <WarningTriangleIcon />
          <h2 className="font-sans font-semibold text-2xl leading-8 text-high-contrast m-0">
            Connection Failed
          </h2>
          <p className="font-sans font-normal text-lg leading-7 text-high-contrast m-0">
            Unable to establish SSH connection to the server
          </p>
        </div>

        <div className="w-[480px] bg-[rgba(211,48,48,0.12)] border border-[#d33030] rounded-lg p-4 flex flex-col gap-1">
          <span className="font-manrope font-bold text-base leading-6 text-[#d33030]">
            Error details
          </span>
          <p className="font-manrope font-normal text-sm leading-5 text-[#d33030] m-0">
            {errorMessage}
          </p>
        </div>

        <div className="flex gap-4 w-[480px]">
          <button
            type="button"
            onClick={onGoBack}
            className="flex-1 bg-secondary text-pure-white font-manrope font-bold text-sm leading-6 rounded-lg px-4 py-2 hover:opacity-[0.88] active:opacity-75 transition-opacity duration-150"
          >
            Go back
          </button>
          <button
            type="button"
            onClick={onRetry}
            className="flex-1 border border-secondary text-secondary font-manrope font-bold text-sm leading-6 rounded-lg px-4 py-2 hover:bg-secondary hover:text-pure-white transition-colors duration-150"
          >
            Retry connection
          </button>
        </div>
      </div>
    </Card>
  )
}
