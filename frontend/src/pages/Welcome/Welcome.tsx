import { useNavigate } from '@tanstack/react-router'
import { welcomeRoute } from '@/pages/Welcome/route'
import { LogoIcon } from '@/assets/icons'
import { Button } from '@/components/Button'

function CheckCircleIcon() {
  return (
    <svg width="32" height="32" viewBox="0 0 32 32" fill="none">
      <circle cx="16" cy="16" r="16" fill="#ff713e" fillOpacity="0.12" />
      <circle cx="16" cy="16" r="10.5" fill="#ff713e" fillOpacity="0.2" />
      <path
        d="M11 16.5L14.5 20L21 13"
        stroke="#ff713e"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}

function ActiveCircleIcon() {
  return (
    <svg width="32" height="32" viewBox="0 0 32 32" fill="none">
      <circle cx="16" cy="16" r="15" fill="white" />
      <circle cx="16" cy="16" r="15" stroke="#ff713e" strokeWidth="2" />
      <circle cx="16" cy="16" r="7" fill="#ff713e" />
    </svg>
  )
}

function PendingCircleIcon() {
  return (
    <svg width="32" height="32" viewBox="0 0 32 32" fill="none">
      <circle cx="16" cy="16" r="15" fill="white" />
      <circle cx="16" cy="16" r="15" stroke="#d4d4d4" strokeWidth="2" />
    </svg>
  )
}

type ItemTitle = 'Admin Account' | 'Server Setup' | 'First Project'

const itemRoutes: Record<ItemTitle, string> = {
  'Admin Account': '/setup/server',
  'Server Setup': '/setup/server',
  'First Project': '/dashboard',
}

export function Welcome() {
  const navigate = useNavigate()
  const status = welcomeRoute.useLoaderData()

  const items = [
    {
      done: status.isAdminOnboarded,
      title: 'Admin Account' as const,
      description: 'Set up your account and secure your instance',
    },
    {
      done: status.isServerSetup,
      title: 'Server Setup' as const,
      description: 'Configure your server and connect to it',
    },
    {
      done: status.isProjectSetup,
      title: 'First Project' as const,
      description: 'Create a project to organize your applications',
    },
  ]

  const nextRoute = items.find((i) => !i.done)?.title ?? items[0].title

  return (
    <div className="w-full min-h-screen bg-linear-to-b from-[rgba(255,122,73,0.01)] to-[rgba(252,140,99,0.12)] flex items-center justify-center py-10 px-4">
      <div className="flex flex-col items-center gap-5">
        <div className="w-9 h-9 bg-primary-darker rounded-[10.8px] flex items-center justify-center">
          <LogoIcon />
        </div>

        <h1 className="font-sans font-semibold text-[32px] leading-10 tracking-[-0.5px] text-high-contrast m-0">
          Welcome to DeployDodo
        </h1>

        <div className="bg-background rounded-xl shadow-[0px_8px_24px_0px_rgba(0,0,0,0.08),0px_0px_1px_0px_rgba(0,0,0,0.2)] w-[484px] max-w-full p-8">
          <div className="flex flex-col gap-6">
            <h2 className="font-sans font-semibold text-2xl leading-8 text-high-contrast m-0">
              What You'll Set Up
            </h2>

            <div className="relative">
              <div className="absolute left-[15px] top-[16px] bottom-[16px] w-px bg-neutral-300" />
              <div className="flex flex-col gap-[25px] relative">
                {items.map((item, index) => {
                  const firstIncomplete = items.findIndex((i) => !i.done)
                  return (
                    <div key={item.title} className="flex gap-4">
                      <div className="flex flex-col items-center">
                        <div className="shrink-0">
                          {item.done ? (
                            <CheckCircleIcon />
                          ) : index === firstIncomplete ? (
                            <ActiveCircleIcon />
                          ) : (
                            <PendingCircleIcon />
                          )}
                        </div>
                      </div>
                      <div className="flex flex-col">
                        <span
                          className={`font-sans font-semibold text-lg leading-7 ${
                            item.done ? 'text-text-secondary' : 'text-high-contrast'
                          }`}
                        >
                          {item.title}
                        </span>
                        <span className="font-sans font-normal text-base leading-6 text-text-secondary">
                          {item.description}
                        </span>
                      </div>
                    </div>
                  )
                })}
              </div>
            </div>

            <div className="flex flex-col gap-2">
              <Button fullWidth onClick={() => navigate({ to: itemRoutes[nextRoute] })}>
                Continue setup
              </Button>
              <Button fullWidth variant="ghost" onClick={() => navigate({ to: '/dashboard' })}>
                Skip for now
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
