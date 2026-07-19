import { useState } from 'react'
import { ArrowBackIcon, WarningCircleIcon } from '@/assets/icons'
import { useToast } from '@/components/Toast'
import { useCreateLocalServer } from '@/api/mutations'
import type { CreateLocalServerResponse } from '@/api/Api'
import { Card } from '@/pages/Dashboard/Servers/PageLayout'
import { invalidateStatusQuery } from '@/api/queries'
import { Button } from '@/components/Button'
import { TextInput } from '@/components/TextInput'

const whatHappensNext = [
  {
    label: 'Immediate Setup:',
    body: 'No SSH connection needed — DeployDodo registers the local machine as a deployment target.',
  },
  {
    label: 'Local Runtime:',
    body: 'Docker must be installed on this machine. Resources you deploy will run locally.',
  },
]

export function LocalServerView({
  onBack,
  onSuccess,
}: {
  onBack: () => void
  onSuccess: (server: CreateLocalServerResponse) => void
}) {
  const [name, setName] = useState('')
  const [error, setError] = useState<string | null>(null)
  const { toast } = useToast()

  const createLocal = useCreateLocalServer({
    onSuccess: async (data) => {
      toast('Local server configured', 'success')
      await invalidateStatusQuery()
      onSuccess(data)
    },
    onError: (error) => {
      if (error.status === 409) {
        setError('A local server has already been configured.')
      } else {
        setError(error.message)
      }
    },
  })

  function handleSubmit(e: React.SubmitEvent) {
    e.preventDefault()
    if (!name.trim()) return
    setError(null)
    createLocal.mutate({ name: name.trim() })
  }

  return (
    <>
      <Card className="p-8">
        <div className="flex flex-col gap-6">
          <button
            type="button"
            onClick={onBack}
            className="text-high-contrast hover:opacity-70 transition-opacity shrink-0"
            aria-label="Go back"
          >
            <ArrowBackIcon />
          </button>

          <div className="flex flex-col gap-2">
            <h2 className="font-sans font-semibold text-2xl leading-8 text-high-contrast m-0">
              Configure Local Server
            </h2>
            <p className="font-sans font-normal text-lg leading-7 text-high-contrast m-0">
              Register this machine as a deployment target. No SSH configuration required.
            </p>
          </div>

          <form onSubmit={handleSubmit} className="flex flex-col gap-6" noValidate>
            <TextInput
              label="Server Name"
              name="name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g. Local Machine"
              helperText="A friendly name to identify this server"
            />

            {error && (
              <div className="bg-[rgba(211,48,48,0.12)] border border-[#d33030] rounded-lg p-3 flex flex-col gap-1">
                <div className="flex items-center gap-1">
                  <WarningCircleIcon className="shrink-0 w-5 h-5" />
                  <span className="font-manrope font-bold text-sm leading-6 text-[#d33030]">
                    Error
                  </span>
                </div>
                <p className="font-manrope font-normal text-sm leading-6 text-[#d33030] m-0">
                  {error}
                </p>
              </div>
            )}

            <Button
              type="submit"
              disabled={createLocal.isPending || !name}
              className="w-full bg-secondary text-pure-white font-manrope font-bold text-sm leading-6 rounded-lg px-4 py-2 hover:opacity-[0.88] active:opacity-75 transition-opacity duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {createLocal.isPending ? 'Configuring…' : 'Configure Server'}
            </Button>
          </form>
        </div>
      </Card>

      <Card className="p-8">
        <div className="flex flex-col gap-3">
          <h3 className="font-sans font-semibold text-lg leading-7 text-primary-darker m-0">
            What Happens Next
          </h3>
          <div className="flex flex-col gap-2">
            {whatHappensNext.map((item) => (
              <p
                key={item.label}
                className="font-sans font-normal text-base leading-6 text-secondary m-0"
              >
                <span className="font-manrope font-bold">{item.label}</span> {item.body}
              </p>
            ))}
          </div>
        </div>
      </Card>
    </>
  )
}
