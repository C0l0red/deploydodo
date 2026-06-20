import { Card } from '@/layouts/PageLayout'

export type ServerOptionId = 'this-machine' | 'remote-server'

function ServerIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="2" y="3" width="16" height="5" rx="1.5" stroke="#181818" strokeWidth="1.5" />
      <rect x="2" y="12" width="16" height="5" rx="1.5" stroke="#181818" strokeWidth="1.5" />
      <circle cx="5.5" cy="5.5" r="1" fill="#181818" />
      <circle cx="5.5" cy="14.5" r="1" fill="#181818" />
    </svg>
  )
}

const serverOptions: {
  id: ServerOptionId
  title: string
  description: string
  badge: { label: string; color: string; bg: string }
}[] = [
  {
    id: 'this-machine',
    title: 'This Machine',
    description:
      'Deploy on the server running DeployDodo. Best for testing and single-server setups.',
    badge: { label: 'Quickstart', color: '#3448f0', bg: 'rgba(52,72,240,0.12)' },
  },
  {
    id: 'remote-server',
    title: 'Remote Server',
    description: 'Connect via SSH to any server—cloud VPS, bare metal, or home infrastructure.',
    badge: { label: 'Recommended', color: '#469b00', bg: 'rgba(70,155,0,0.12)' },
  },
]

const hints = [
  {
    label: 'Servers:',
    body: 'Host your applications, databases, and services (collectively called resources). All CPU-intensive operations run on the target server.',
  },
  {
    label: 'Remote Server:',
    body: 'Any SSH-accessible server—cloud providers (AWS, Hetzner, DigitalOcean), bare metal, or self-hosted infrastructure.',
  },
  {
    label: 'Cloud Integration:',
    body: 'Direct deployment from supported cloud provider accounts with automatic provisioning and configuration.',
  },
]

export function SelectView({ onSelect }: { onSelect: (id: ServerOptionId) => void }) {
  return (
    <>
      <Card className="p-8">
        <div className="flex flex-col gap-6">
          <div className="flex flex-col gap-1">
            <h2 className="font-sans font-semibold text-2xl leading-8 text-high-contrast m-0">
              Choose Server Type
            </h2>
            <p className="font-sans font-normal text-base leading-6 text-high-contrast m-0">
              Select where to deploy your applications and databases. You can add more servers
              later.
            </p>
          </div>
          <div className="flex gap-5">
            {serverOptions.map((option) => (
              <button
                key={option.id}
                type="button"
                onClick={() => onSelect(option.id)}
                className="flex-1 border border-neutral-100 rounded-lg p-4 flex flex-col gap-3 text-left hover:border-high-contrast transition-colors duration-150"
              >
                <div className="flex items-center justify-between">
                  <ServerIcon />
                  <span
                    className="font-manrope font-semibold text-xs leading-4 px-2 py-1 rounded"
                    style={{ color: option.badge.color, backgroundColor: option.badge.bg }}
                  >
                    {option.badge.label}
                  </span>
                </div>
                <div className="flex flex-col gap-1">
                  <span className="font-sans font-semibold text-lg leading-7 text-high-contrast">
                    {option.title}
                  </span>
                  <span className="font-manrope font-normal text-sm leading-5 text-high-contrast">
                    {option.description}
                  </span>
                </div>
              </button>
            ))}
          </div>
        </div>
      </Card>

      <Card className="p-8">
        <div className="flex flex-col gap-3">
          <h3 className="font-sans font-semibold text-lg leading-7 text-primary-darker m-0">
            Hints
          </h3>
          <div className="flex flex-col gap-3">
            {hints.map((hint) => (
              <p
                key={hint.label}
                className="font-sans font-normal text-base leading-6 text-secondary m-0"
              >
                <span className="font-manrope font-bold">{hint.label}</span> {hint.body}
              </p>
            ))}
          </div>
        </div>
      </Card>
    </>
  )
}
