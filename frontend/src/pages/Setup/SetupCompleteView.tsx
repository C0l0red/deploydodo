import { CheckCircleOutlineIcon } from '@/assets/icons'
import { useToast } from '@/components/Toast'
import { Card } from '@/layouts/PageLayout'

export function SetupCompleteView({ onGoToDashboard }: { onGoToDashboard: () => void }) {
  const { toast } = useToast()

  function handleGoToDashboard() {
    toast('Setup complete — welcome to DeployDodo', 'success')
    onGoToDashboard()
  }

  return (
    <Card className="p-10">
      <div className="flex flex-col items-center gap-8">
        <div className="flex flex-col items-center gap-2">
          <CheckCircleOutlineIcon />
          <h2 className="font-sans font-semibold text-2xl leading-8 text-high-contrast m-0">
            Setup Complete
          </h2>
          <p className="font-sans font-normal text-lg leading-7 text-high-contrast m-0">
            Your setup is complete and your server is ready. Start deploying your first resource.
          </p>
        </div>

        <button
          type="button"
          onClick={handleGoToDashboard}
          className="w-150 bg-secondary text-pure-white font-manrope font-bold text-sm leading-6 rounded-lg px-4 py-2 hover:opacity-[0.88] active:opacity-75 transition-opacity duration-150"
        >
          Go to Dashboard
        </button>
      </div>
    </Card>
  )
}
