import { useState } from 'react'
import { useFormik } from 'formik'
import * as Yup from 'yup'
import { EyeClosedIcon, EyeOpenIcon } from '@/assets/icons'
import { TextInput } from '@/components/TextInput'
import { Toggle, SectionCard, SectionHeader, SaveButton, OutlineButton, SelectField, FieldLabel, AreaChart, BarChart, Sidebar } from '..'

type ConfigSidebar =
  | 'General'
  | 'Advanced'
  | 'Private Key'
  | 'CA Certificate'
  | 'Docker Cleanup'
  // | 'Destinations'
  | 'Log Drains'
  | 'Metrics'

const configOptions: ConfigSidebar[] = [
  'General', 'Advanced', 'Private Key', 'CA Certificate',
  'Docker Cleanup', /* 'Destinations', */ 'Log Drains', 'Metrics',
]

export function ConfigurationTab() {
  const [activeConfigSidebar, setActiveConfigSidebar] = useState<ConfigSidebar>('General')

  const [showHostUrl, setShowHostUrl] = useState(false)
  const [showSentinelUrl, setShowSentinelUrl] = useState(false)
  const [showConcurrentBuilds, setShowConcurrentBuilds] = useState(false)

  const [sentinelEnabled, setSentinelEnabled] = useState(true)

  const [keys, setKeys] = useState([
    {
      id: 1,
      name: "Local host's key",
      description: 'The private key for the DeployDodo host machine (localhost).',
      current: true,
      menuOpen: false,
    },
  ])

  const [caCert] = useState(
    '-----BEGIN CERTIFICATE-----\n' +
    'MIICZjCCAcagAwIBAgIIcno/XxLokqAwCgYIKoZIzj0EAwQwUjEFMBOGA1UEAwwW\n' +
    'Q29vbGlmeSBDQSBkZXJoAWzpY2FOZTEQMA4GA1UECgwhQ29vbGlmeTELMAkGA1UE\n' +
    'BhMCWFgxEDAOBgNVBAgMBORIZmF1bHhWcHMtMTA3MTgyNTExWhcNMzUxMTA1\n' +
    'MTgyNTExWjBSMS8wHQYDVQQDDBBzdWI29saWZ5IENBIEN1cnZlYXlXSEIMRAWdGYD\n' +
    'VQQKDADdb29saWZ5MQswCQYDVQQGEwJYWDEQMA4GA1UECAwHRGVmYXVsdDCBmzAQ\n' +
    'BgcqhkjOPQIBBgUrgQQAIwOBhgAEAQoAOSO7/Qn//p8a2JOS/Z+vLMraexeTVxEo\n' +
    '/bKjCDMKh9g98d/caBWDy+jUSnz/YbjTbrKgIpur+jKYwvbqjxd3AAqAUu3Ova5c\n' +
    'LAe9pmVdSg8/n9Fs6/AFqlb4P6cpYo3nKrO1Yu2pZaHxVjNjCtCPBuhn7fWuDKoK\n' +
    'UzjDdkw8jpKNoOUwQzASBgNVHRMBAf8ECDAGAQH/AgEAMA4GA1UdDwEB/wQEAwIB\n' +
    'BjAdBgNVHQ4EFgQUP7mudZGoASoA/C9okVgZpocateeEwCgYIKoZIzj0EAwQDgYsA\n' +
    'MIGHAkBxgFeGFvNaovayKhzzFZfeqME75jTbL4r4EDME+emmy7qwF7Iza4XS397\n' +
    'DsINWp1reeZs9x4Bxl44tXtTeEbPRwwCQRomSQhBBRCM/mCQmjbo5vFkB7Z1WvE3\n' +
    'shYfSeAgrOh/GLkyeGF/sINquN4PBSmb265Lrw5pwYA4qrlDC3FtiPD\n' +
    '-----END CERTIFICATE-----'
  )
  const [showCaCertContent, setShowCaCertContent] = useState(true)

  const [forceDockerCleanup, setForceDockerCleanup] = useState(true)
  const [cleanupSchedule, setCleanupSchedule] = useState('')
  const [danglingVolumes, setDanglingVolumes] = useState(true)
  const [deleteUnusedNetworks, setDeleteUnusedNetworks] = useState(true)

  const [newRelicLicenseKey, setNewRelicLicenseKey] = useState('')
  const [newRelicEndpoint, setNewRelicEndpoint] = useState('')
  const [showNewRelicLicenseKey, setShowNewRelicLicenseKey] = useState(false)
  const [showNewRelicEndpoint, setShowNewRelicEndpoint] = useState(false)

  const [axiomApiKey, setAxiomApiKey] = useState('')
  const [axiomDatasetName, setAxiomDatasetName] = useState('')
  const [showAxiomApiKey, setShowAxiomApiKey] = useState(false)
  const [showAxiomDatasetName, setShowAxiomDatasetName] = useState(false)

  const [metricsInterval, setMetricsInterval] = useState('5mins')

  const generalForm = useFormik({
    initialValues: { name: 'Localhost', description: '', wildcardDomain: '', hostUrl: 'ssh://root@127.0.0.1:22', user: 'root', port: '22', timezone: 'UTC' },
    validationSchema: Yup.object({ name: Yup.string().required('Name is required') }),
    onSubmit: () => { },
  })

  const sentinelForm = useFormik({
    initialValues: { deployDodoUrl: 'https://host.docker.internal:1234', metricsRate: '10', metricsHistory: '7', pushInterval: '60' },
    onSubmit: () => { },
  })

  const advancedForm = useFormik({
    initialValues: {
      diskCheckFrequency: '',
      diskNotificationThreshold: '',
      concurrentBuilds: '',
      deploymentTimeout: '',
    },
    onSubmit: () => { },
  })

  return (
    <div className="flex gap-6 items-start">
      <Sidebar options={configOptions} active={activeConfigSidebar} onChange={setActiveConfigSidebar} />

        <div className="flex-1 min-w-0 flex flex-col gap-5">

          {/* ── General ── */}
          {activeConfigSidebar === 'General' && (
            <>
              <form onSubmit={generalForm.handleSubmit}>
                <SectionCard>
                  <SectionHeader title="General" subtitle="Server is reachable and validated" right={<SaveButton />} />
                  <div className="flex flex-col gap-4">
                    <div className="grid grid-cols-3 gap-4">
                      <TextInput label="Name" name="name"
                        value={generalForm.values.name} onChange={generalForm.handleChange} onBlur={generalForm.handleBlur}
                        hasError={generalForm.touched.name && !!generalForm.errors.name} errorMessage={generalForm.errors.name} />
                      <TextInput label="Description" name="description"
                        value={generalForm.values.description} onChange={generalForm.handleChange} onBlur={generalForm.handleBlur} />
                      <TextInput label="Wildcard Domain" name="wildcardDomain"
                        value={generalForm.values.wildcardDomain} onChange={generalForm.handleChange} onBlur={generalForm.handleBlur} />
                    </div>
                    <div className="flex gap-4">
                      <div className="flex-[3]">
                        <TextInput label="Host URL" name="hostUrl"
                          type={showHostUrl ? 'text' : 'password'}
                          value={generalForm.values.hostUrl} onChange={generalForm.handleChange} onBlur={generalForm.handleBlur}
                          suffix={
                            <button type="button" onClick={() => setShowHostUrl(!showHostUrl)}
                              className="text-text-secondary hover:text-high-contrast outline-none cursor-pointer">
                              {showHostUrl ? <EyeClosedIcon className="size-4" /> : <EyeOpenIcon className="size-4" />}
                            </button>
                          }
                        />
                      </div>
                      <div className="flex-1">
                        <TextInput label="User" name="user"
                          value={generalForm.values.user} onChange={generalForm.handleChange} onBlur={generalForm.handleBlur} />
                      </div>
                      <div className="flex-1">
                        <TextInput label="Port" name="port"
                          value={generalForm.values.port} onChange={generalForm.handleChange} onBlur={generalForm.handleBlur} />
                      </div>
                    </div>
                    <SelectField id="timezone" label="Server Timezone"
                      value={generalForm.values.timezone} onChange={generalForm.handleChange} className="max-w-xs">
                      <option value="UTC">UTC</option>
                      <option value="America/New_York">America/New_York</option>
                      <option value="Europe/London">Europe/London</option>
                      <option value="Africa/Lagos">Africa/Lagos</option>
                    </SelectField>
                  </div>
                </SectionCard>
              </form>

              {/* Sentinel */}
              <form onSubmit={sentinelForm.handleSubmit}>
                <SectionCard>
                  <SectionHeader
                    title={
                      <div className="flex items-center gap-2">
                        <h2 className="font-sans font-semibold text-xl leading-7 text-high-contrast m-0">Sentinel</h2>
                        <span className="font-manrope font-semibold text-xs px-2 py-0.5 rounded bg-[#eaf6ec] text-[#2e7d32]">In Sync</span>
                      </div>
                    }
                    right={
                      <div className="flex items-center gap-3">
                        <button type="submit" className="text-text-secondary hover:text-high-contrast transition-colors outline-none cursor-pointer" aria-label="Save">
                          <svg className="size-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
                            <polyline points="17 21 17 13 7 13 7 21" /><polyline points="7 3 7 8 15 8" />
                          </svg>
                        </button>
                        <button type="button" className="text-text-secondary hover:text-high-contrast transition-colors outline-none cursor-pointer" aria-label="Refresh">
                          <svg className="size-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                            <path d="M23 4v6h-6" /><path d="M1 20v-6h6" />
                            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
                          </svg>
                        </button>
                        <button type="button" className="text-text-secondary hover:text-high-contrast transition-colors outline-none cursor-pointer" aria-label="History">
                          <svg className="size-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                            <circle cx="12" cy="12" r="10" /><polyline points="12 6 12 12 16 14" />
                          </svg>
                        </button>
                      </div>
                    }
                  />
                  <div className="flex flex-col gap-5">
                    <div className="flex flex-col gap-2">
                      <FieldLabel>Enable Sentinel</FieldLabel>
                      <Toggle enabled={sentinelEnabled} onToggle={() => setSentinelEnabled(!sentinelEnabled)} />
                    </div>

                    <div className="flex flex-col gap-2">
                      <FieldLabel>Host URL</FieldLabel>
                      <div className="flex gap-3 items-center">
                        <div className="relative flex-1">
                          <input type={showSentinelUrl ? 'text' : 'password'} value="sentinel-secret-token-url" readOnly
                            className="w-full bg-background border border-neutral-100 rounded-lg px-3 py-2 font-manrope font-normal text-sm leading-6 text-text-secondary outline-none" />
                          <button type="button" onClick={() => setShowSentinelUrl(!showSentinelUrl)}
                            className="absolute inset-y-0 right-0 flex items-center pr-3 text-text-secondary hover:text-high-contrast outline-none cursor-pointer">
                            {showSentinelUrl ? <EyeClosedIcon className="size-4" /> : <EyeOpenIcon className="size-4" />}
                          </button>
                        </div>
                        <OutlineButton>Regenerate</OutlineButton>
                      </div>
                    </div>

                    <TextInput label="DeployDodo URL" name="deployDodoUrl"
                      value={sentinelForm.values.deployDodoUrl} onChange={sentinelForm.handleChange} onBlur={sentinelForm.handleBlur} />

                    <div className="grid grid-cols-3 gap-4">
                      <TextInput label="Metrics rate (seconds)*" name="metricsRate"
                        value={sentinelForm.values.metricsRate} onChange={sentinelForm.handleChange} onBlur={sentinelForm.handleBlur} />
                      <TextInput label="Metrics history (days)*" name="metricsHistory"
                        value={sentinelForm.values.metricsHistory} onChange={sentinelForm.handleChange} onBlur={sentinelForm.handleBlur} />
                      <TextInput label="Push interval (seconds)*" name="pushInterval"
                        value={sentinelForm.values.pushInterval} onChange={sentinelForm.handleChange} onBlur={sentinelForm.handleBlur} />
                    </div>
                  </div>
                </SectionCard>
              </form>
            </>
          )}

          {activeConfigSidebar === 'Advanced' && (
            <form onSubmit={advancedForm.handleSubmit} className="flex flex-col gap-5">
              <div className="flex items-center justify-between">
                <div className="flex flex-col gap-1">
                  <h2 className="font-sans font-bold text-3xl text-high-contrast m-0">Advanced</h2>
                  <span className="font-sans font-normal text-sm text-text-secondary">
                    Advanced configuration for your server
                  </span>
                </div>
                <SaveButton />
              </div>

              {/* Disk Usage Card */}
              <SectionCard>
                <div className="flex flex-col gap-4">
                  <h3 className="font-sans font-bold text-lg text-high-contrast m-0">Disk Usage</h3>
                  <div className="grid grid-cols-2 gap-4">
                    <TextInput
                      label="Disk usage check frequency"
                      name="diskCheckFrequency"
                      value={advancedForm.values.diskCheckFrequency}
                      onChange={advancedForm.handleChange}
                      onBlur={advancedForm.handleBlur}
                    />
                    <TextInput
                      label="Server disk usage notification threshold (%)"
                      name="diskNotificationThreshold"
                      value={advancedForm.values.diskNotificationThreshold}
                      onChange={advancedForm.handleChange}
                      onBlur={advancedForm.handleBlur}
                    />
                  </div>
                </div>
              </SectionCard>

              {/* Builds Card */}
              <SectionCard>
                <div className="flex flex-col gap-4">
                  <h3 className="font-sans font-bold text-lg text-high-contrast m-0">Builds</h3>
                  <div className="grid grid-cols-2 gap-4">
                    <TextInput
                      label="Number of concurrent builds*"
                      name="concurrentBuilds"
                      type={showConcurrentBuilds ? 'text' : 'password'}
                      value={advancedForm.values.concurrentBuilds}
                      onChange={advancedForm.handleChange}
                      onBlur={advancedForm.handleBlur}
                      suffix={
                        <button
                          type="button"
                          onClick={() => setShowConcurrentBuilds(!showConcurrentBuilds)}
                          className="text-text-secondary hover:text-high-contrast outline-none cursor-pointer"
                        >
                          {showConcurrentBuilds ? (
                            <EyeClosedIcon className="size-4" />
                          ) : (
                            <EyeOpenIcon className="size-4" />
                          )}
                        </button>
                      }
                    />
                    <TextInput
                      label="Deployment timeout (seconds)*"
                      name="deploymentTimeout"
                      value={advancedForm.values.deploymentTimeout}
                      onChange={advancedForm.handleChange}
                      onBlur={advancedForm.handleBlur}
                    />
                  </div>
                </div>
              </SectionCard>
            </form>
          )}

          {/* ── Private Key ── */}
          {activeConfigSidebar === 'Private Key' && (
            <div className="flex flex-col gap-5">
              <div className="flex items-center justify-between">
                <h2 className="font-sans font-bold text-3xl text-high-contrast m-0">Private Key</h2>
                <div className="flex items-center gap-2">
                  <OutlineButton onClick={() => { }}>
                    <span className="font-sans font-bold mr-1">+</span> Add
                  </OutlineButton>
                  <OutlineButton onClick={() => { }}>
                    <svg className="size-4 mr-1.5 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                      <path d="M5 12.55a11 11 0 0 1 14.08 0" /><path d="M1.42 9a16 16 0 0 1 21.16 0" />
                      <path d="M8.53 16.11a6 6 0 0 1 6.95 0" /><line x1="12" y1="20" x2="12.01" y2="20" />
                    </svg>
                    Check connection
                  </OutlineButton>
                </div>
              </div>

              <div className="flex flex-col gap-3 w-[28rem] max-w-full">
                {keys.map((key) => (
                  <div key={key.id} className="border border-neutral-100 rounded-xl p-5 flex flex-col gap-3 bg-white">
                    <div className="flex items-start justify-between">
                      <div className="flex flex-col gap-1">
                        <span className="font-sans font-semibold text-base leading-6 text-high-contrast">{key.name}</span>
                        <p className="font-sans font-normal text-sm leading-5 text-text-secondary m-0">{key.description}</p>
                      </div>
                      <div className="relative">
                        <button
                          type="button"
                          onClick={() => setKeys(keys.map((k) => k.id === key.id ? { ...k, menuOpen: !k.menuOpen } : { ...k, menuOpen: false }))}
                          className="text-text-secondary hover:text-high-contrast outline-none cursor-pointer px-1 py-0.5 rounded hover:bg-neutral-100"
                        >
                          <svg className="size-4" viewBox="0 0 24 24" fill="currentColor">
                            <circle cx="5" cy="12" r="1.5" /><circle cx="12" cy="12" r="1.5" /><circle cx="19" cy="12" r="1.5" />
                          </svg>
                        </button>
                        {key.menuOpen && (
                          <div className="absolute right-0 top-7 bg-white border border-neutral-100 rounded-lg shadow-md py-1 z-10 min-w-[130px]">
                            <button type="button" onClick={() => setKeys(keys.map((k) => ({ ...k, menuOpen: false })))}
                              className="w-full text-left px-3 py-1.5 font-manrope text-sm text-high-contrast hover:bg-neutral-100">Edit</button>
                            <button type="button"
                              onClick={() => { setKeys(keys.filter((k) => k.id !== key.id)) }}
                              className="w-full text-left px-3 py-1.5 font-manrope text-sm text-error hover:bg-neutral-100">Delete</button>
                          </div>
                        )}
                      </div>
                    </div>
                    {key.current && (
                      <span className="font-manrope font-semibold text-xs px-2 py-0.5 rounded bg-[#eaf6ec] text-[#2e7d32] w-fit">
                        Currently used
                      </span>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* ── CA Certificate ── */}
          {activeConfigSidebar === 'CA Certificate' && (
            <form onSubmit={(e) => e.preventDefault()} className="flex flex-col gap-5">
              <div className="flex items-start justify-between">
                <div className="flex flex-col gap-1 max-w-3xl">
                  <h2 className="font-sans font-bold text-3xl text-high-contrast m-0">CA Certificate</h2>
                  <span className="font-sans font-normal text-sm leading-6 text-text-secondary">
                    Mount DeployDodo's CA certificate into any container that needs to connect to a database over SSL. You can view and copy the bind mount example below. Learn more about when and why this configuration is needed here.
                  </span>
                </div>
                <div className="flex items-center gap-2 shrink-0">
                  <OutlineButton onClick={() => { }}>Regenerate</OutlineButton>
                  <SaveButton type="button" onClick={() => { }} />
                </div>
              </div>

              {/* Bind Mount Example box */}
              <div className="w-full bg-neutral-200/30 border border-neutral-100 rounded-lg px-4 py-2.5 font-mono text-sm text-secondary select-all">
                - /data/DeployDodo/ssl/DeployDodo-ca.crt:/etc/ssl/certs/DeployDodo-ca.crt:ro
              </div>

              <div className="flex flex-col gap-2 mt-2">
                <div className="flex items-center justify-between">
                  <span className="font-sans font-semibold text-base text-high-contrast">
                    CA Certificate <span className="font-normal text-text-secondary/70 text-sm ml-1">(Valid until: 05.11.2035 18:25:11)</span>
                  </span>
                  <button
                    type="button"
                    onClick={() => setShowCaCertContent(!showCaCertContent)}
                    className="inline-flex items-center gap-1.5 font-sans font-normal text-sm text-text-secondary hover:text-high-contrast outline-none cursor-pointer"
                  >
                    {showCaCertContent ? (
                      <>
                        <EyeClosedIcon className="size-4" />
                        <span>Hide</span>
                      </>
                    ) : (
                      <>
                        <EyeOpenIcon className="size-4" />
                        <span>Show</span>
                      </>
                    )}
                  </button>
                </div>

                {showCaCertContent && (
                  <textarea
                    readOnly
                    value={caCert}
                    className="w-full bg-background border border-neutral-100 rounded-lg px-3 py-3 font-mono font-normal text-xs leading-relaxed text-secondary outline-none resize-none focus:border-secondary transition-[border-color] duration-150 select-all"
                    rows={14}
                  />
                )}
              </div>
            </form>
          )}

          {/* ── Docker Cleanup ── */}
          {activeConfigSidebar === 'Docker Cleanup' && (
            <div className="flex flex-col gap-5">
              <div className="flex items-center justify-between">
                <div className="flex flex-col gap-1">
                  <h2 className="font-sans font-bold text-3xl text-high-contrast m-0">Docker Cleanup</h2>
                  <span className="font-sans font-normal text-sm text-text-secondary">
                    Configure Docker cleanup settings for your server.
                  </span>
                </div>
                <div className="flex items-center gap-2">
                  <OutlineButton onClick={() => { }}>Start clean up</OutlineButton>
                  <SaveButton type="button" onClick={() => { }} />
                </div>
              </div>

              {/* Card 1: Cleanup Configuration */}
              <SectionCard>
                <div className="flex flex-col gap-4">
                  <h3 className="font-sans font-bold text-lg text-high-contrast m-0">Cleanup Configuration</h3>

                  <div className="flex flex-col gap-2">
                    <FieldLabel>Docker cleanup frequency</FieldLabel>
                    <input
                      type="text"
                      value={cleanupSchedule}
                      onChange={(e) => setCleanupSchedule(e.target.value)}
                      className="w-full bg-background border border-neutral-100 rounded-lg px-3 py-2 font-mono font-normal text-sm leading-6 text-secondary outline-none focus:border-secondary transition-[border-color] duration-150"
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <span className="font-sans font-normal text-sm text-high-contrast">
                      Force Docker Cleanup
                    </span>
                    <Toggle enabled={forceDockerCleanup} onToggle={() => setForceDockerCleanup(!forceDockerCleanup)} />
                  </div>
                </div>
              </SectionCard>

              {/* Card 2: Advanced */}
              <SectionCard>
                <div className="flex flex-col gap-4">
                  <h3 className="font-sans font-bold text-lg text-high-contrast m-0">Advanced</h3>

                  {/* Alert banner */}
                  <div className="bg-[rgba(255,113,62,0.08)] border border-[rgba(255,113,62,0.2)] rounded-xl p-4 flex flex-col gap-1.5">
                    <div className="flex items-center gap-2">
                      <svg className="size-4 text-primary shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                        <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
                        <line x1="12" y1="9" x2="12" y2="13" /><line x1="12" y1="17" x2="12.01" y2="17" />
                      </svg>
                      <span className="font-sans font-bold text-sm text-primary">Alert title</span>
                    </div>
                    <p className="font-sans font-normal text-xs leading-5 text-high-contrast m-0">
                      Pull request #9999 merged after a successful build
                    </p>
                  </div>

                  <div className="grid grid-cols-2 gap-4 mt-2">
                    <div className="flex items-center justify-between border border-neutral-100/50 rounded-xl p-4 bg-white">
                      <span className="font-sans font-normal text-sm text-high-contrast">Delete Unused Volumes</span>
                      <Toggle enabled={danglingVolumes} onToggle={() => setDanglingVolumes(!danglingVolumes)} />
                    </div>
                    <div className="flex items-center justify-between border border-neutral-100/50 rounded-xl p-4 bg-white">
                      <span className="font-sans font-normal text-sm text-high-contrast">Delete Unused Networks</span>
                      <Toggle enabled={deleteUnusedNetworks} onToggle={() => setDeleteUnusedNetworks(!deleteUnusedNetworks)} />
                    </div>
                  </div>
                </div>
              </SectionCard>

              {/* Card 3: Recent executions */}
              <SectionCard>
                <div className="flex items-center justify-between">
                  <h3 className="font-sans font-bold text-lg text-high-contrast m-0">Recent executions</h3>
                  <div className="flex items-center gap-4 text-text-secondary">
                    <button type="button" className="hover:text-high-contrast outline-none cursor-pointer" aria-label="More options">
                      <svg className="size-5" viewBox="0 0 24 24" fill="currentColor">
                        <circle cx="5" cy="12" r="1.5" /><circle cx="12" cy="12" r="1.5" /><circle cx="19" cy="12" r="1.5" />
                      </svg>
                    </button>
                    <button type="button" className="hover:text-high-contrast outline-none cursor-pointer" aria-label="Collapse section">
                      <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                        <path d="M18 15l-6-6-6 6" />
                      </svg>
                    </button>
                  </div>
                </div>

                <div className="border border-neutral-100 rounded-xl overflow-hidden">
                  <table className="w-full border-collapse">
                    <thead>
                      <tr className="bg-neutral-200 border-b border-neutral-100">
                        <th className="px-4 py-2.5 text-left font-manrope font-bold text-xs text-text-secondary uppercase tracking-wide border-r-inset w-12">#</th>
                        <th className="px-4 py-2.5 text-left font-manrope font-bold text-xs text-text-secondary uppercase tracking-wide border-r-inset">Started</th>
                        <th className="px-4 py-2.5 text-left font-manrope font-bold text-xs text-text-secondary uppercase tracking-wide border-r-inset">Ended</th>
                        <th className="px-4 py-2.5 text-left font-manrope font-bold text-xs text-text-secondary uppercase tracking-wide border-r-inset">Status</th>
                        <th className="px-4 py-2.5 text-left font-manrope font-bold text-xs text-text-secondary uppercase tracking-wide border-r-inset">Runtime</th>
                        <th className="px-4 py-2.5 text-left font-manrope font-bold text-xs text-text-secondary uppercase tracking-wide border-r-inset">Finished</th>
                        <th className="px-4 py-2.5 text-left font-manrope font-bold text-xs text-text-secondary uppercase tracking-wide w-12"></th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr className="border-b border-neutral-100 last:border-b-0">
                        <td className="px-4 py-3 font-manrope text-sm text-text-secondary border-r-inset">1</td>
                        <td className="px-4 py-3 font-manrope text-sm text-text-secondary border-r-inset">2025-11-13 00:00:08 UTC</td>
                        <td className="px-4 py-3 font-manrope text-sm text-text-secondary border-r-inset">2025-11-13 00:00:10 UTC</td>
                        <td className="px-4 py-3 border-r-inset">
                          <span className="font-manrope font-semibold text-xs px-2 py-0.5 rounded bg-[#eaf6ec] text-[#2e7d32]">
                            Success
                          </span>
                        </td>
                        <td className="px-4 py-3 font-manrope text-sm text-text-secondary border-r-inset">00m 02s</td>
                        <td className="px-4 py-3 font-manrope text-sm text-text-secondary border-r-inset">11 hours ago</td>
                        <td className="px-4 py-3 text-center">
                          <button type="button" className="text-text-secondary hover:text-high-contrast outline-none cursor-pointer" aria-label="Download log">
                            <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
                              <polyline points="14 2 14 8 20 8" />
                              <line x1="12" y1="18" x2="12" y2="12" />
                              <polyline points="9 15 12 18 15 15" />
                            </svg>
                          </button>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </SectionCard>
            </div>
          )}

          {/* ── Destinations ── */}
          {/*
          {activeConfigSidebar === 'Destinations' && (
            <div className="flex flex-col gap-5">
              <div className="flex items-center justify-between">
                <div className="flex flex-col gap-1">
                  <h2 className="font-sans font-bold text-3xl text-high-contrast m-0">Destination</h2>
                  <span className="font-sans font-normal text-sm text-text-secondary">
                    Destinations are used to segregate resources by network.
                  </span>
                </div>
                <div className="flex items-center gap-2">
                  <OutlineButton onClick={() => { }}>Add</OutlineButton>
                  <OutlineButton onClick={() => { }}>Find Destinations</OutlineButton>
                </div>
              </div>

              <div className="flex flex-wrap gap-4">
                <div className="border border-neutral-100 rounded-xl p-5 flex flex-col gap-3 bg-white w-64">
                  <div className="flex flex-col gap-2">
                    <h3 className="font-sans font-bold text-lg text-high-contrast m-0">DeployDodo</h3>
                    <div className="flex flex-col gap-1 text-sm text-text-secondary">
                      <div>
                        <span className="font-sans font-bold text-high-contrast text-sm">Server IP: </span>
                        <span className="font-sans text-sm text-text-secondary">host.docker.internal</span>
                      </div>
                      <div>
                        <span className="font-sans font-bold text-high-contrast text-sm">Docker Network: </span>
                        <span className="font-sans text-sm text-text-secondary">DeployDodo</span>
                      </div>
                    </div>
                  </div>
                  <span className="font-manrope font-semibold text-xs px-2 py-0.5 rounded bg-[#eaf6ec] text-[#2e7d32] w-fit">
                    Currently used
                  </span>
                </div>
              </div>
            </div>
          )}
          */}

          {/* ── Log Drains ── */}
          {activeConfigSidebar === 'Log Drains' && (
            <div className="flex flex-col gap-5">
              <div className="flex flex-col gap-1">
                <h2 className="font-sans font-bold text-3xl text-high-contrast m-0">Log Drains</h2>
                <span className="font-sans font-normal text-sm text-text-secondary">
                  Advanced configuration for your server
                </span>
              </div>

              {/* Card 1: New Relic */}
              <SectionCard>
                <div className="flex flex-col gap-4">
                  <h3 className="font-sans font-bold text-lg text-high-contrast m-0">New Relic</h3>
                  <div className="grid grid-cols-2 gap-4">
                    <TextInput
                      label="License Key*"
                      name="newRelicLicenseKey"
                      type={showNewRelicLicenseKey ? 'text' : 'password'}
                      value={newRelicLicenseKey}
                      onChange={(e) => setNewRelicLicenseKey(e.target.value)}
                      suffix={
                        <button
                          type="button"
                          onClick={() => setShowNewRelicLicenseKey(!showNewRelicLicenseKey)}
                          className="text-text-secondary hover:text-high-contrast outline-none cursor-pointer"
                        >
                          {showNewRelicLicenseKey ? (
                            <EyeClosedIcon className="size-4" />
                          ) : (
                            <EyeOpenIcon className="size-4" />
                          )}
                        </button>
                      }
                    />
                    <TextInput
                      label="Endpoint*"
                      name="newRelicEndpoint"
                      type={showNewRelicEndpoint ? 'text' : 'password'}
                      value={newRelicEndpoint}
                      onChange={(e) => setNewRelicEndpoint(e.target.value)}
                      suffix={
                        <button
                          type="button"
                          onClick={() => setShowNewRelicEndpoint(!showNewRelicEndpoint)}
                          className="text-text-secondary hover:text-high-contrast outline-none cursor-pointer"
                        >
                          {showNewRelicEndpoint ? (
                            <EyeClosedIcon className="size-4" />
                          ) : (
                            <EyeOpenIcon className="size-4" />
                          )}
                        </button>
                      }
                    />
                  </div>
                  <div>
                    <SaveButton type="button" onClick={() => { }} />
                  </div>
                </div>
              </SectionCard>

              {/* Card 2: Axiom */}
              <SectionCard>
                <div className="flex flex-col gap-4">
                  <h3 className="font-sans font-bold text-lg text-high-contrast m-0">Axiom</h3>
                  <div className="grid grid-cols-2 gap-4">
                    <TextInput
                      label="API Key*"
                      name="axiomApiKey"
                      type={showAxiomApiKey ? 'text' : 'password'}
                      value={axiomApiKey}
                      onChange={(e) => setAxiomApiKey(e.target.value)}
                      suffix={
                        <button
                          type="button"
                          onClick={() => setShowAxiomApiKey(!showAxiomApiKey)}
                          className="text-text-secondary hover:text-high-contrast outline-none cursor-pointer"
                        >
                          {showAxiomApiKey ? (
                            <EyeClosedIcon className="size-4" />
                          ) : (
                            <EyeOpenIcon className="size-4" />
                          )}
                        </button>
                      }
                    />
                    <TextInput
                      label="Dataset Name *"
                      name="axiomDatasetName"
                      type={showAxiomDatasetName ? 'text' : 'password'}
                      value={axiomDatasetName}
                      onChange={(e) => setAxiomDatasetName(e.target.value)}
                      suffix={
                        <button
                          type="button"
                          onClick={() => setShowAxiomDatasetName(!showAxiomDatasetName)}
                          className="text-text-secondary hover:text-high-contrast outline-none cursor-pointer"
                        >
                          {showAxiomDatasetName ? (
                            <EyeClosedIcon className="size-4" />
                          ) : (
                            <EyeOpenIcon className="size-4" />
                          )}
                        </button>
                      }
                    />
                  </div>
                  <div>
                    <SaveButton type="button" onClick={() => { }} />
                  </div>
                </div>
              </SectionCard>

              {/* Card 3: Custom FluentBit */}
              <SectionCard>
                <div className="flex flex-col gap-4">
                  <div className="flex flex-col gap-0.5">
                    <h3 className="font-sans font-bold text-lg text-high-contrast m-0">Custom FluentBit</h3>
                    <p className="font-sans font-normal text-sm leading-5 text-text-secondary m-0">
                      Custom FluentBit Configuration
                    </p>
                  </div>
                  <textarea
                    readOnly
                    rows={14}
                    value={`# This file is generated by DeployDodo, do not edit it manually.
# Disable the default redirect to customize (only if you know what you are doing)
http:
  routers:
    defRedirect:
      entryPoints:
        - http
        - https
      service: noop
      rule: PathPrefix(\`/\`)
      tls:
        certResolver: letsencrypt
      priority: 1000
  services:
    noop:
      loadBalancer:
        servers: []`}
                    className="w-full bg-background border border-neutral-100 rounded-lg px-3 py-3 font-mono font-normal text-xs leading-relaxed text-secondary outline-none resize-none focus:border-secondary transition-[border-color] duration-150 select-all"
                  />
                </div>
              </SectionCard>
            </div>
          )}

          {/* ── Metrics ── */}
          {activeConfigSidebar === 'Metrics' && (
            <div className="flex flex-col gap-5">
              <div className="flex items-center justify-between">
                <div className="flex flex-col gap-1">
                  <h2 className="font-sans font-bold text-3xl text-high-contrast m-0">Metrics</h2>
                  <span className="font-sans font-normal text-sm text-text-secondary">
                    Advanced configuration for your server
                  </span>
                </div>
                <SaveButton type="button" onClick={() => { }} />
              </div>

              <div className="flex flex-col gap-2 max-w-full">
                <SelectField
                  id="metricsInterval"
                  label="Interval"
                  value={metricsInterval}
                  onChange={(e) => setMetricsInterval(e.target.value)}
                >
                  <option value="5mins">5mins (live)</option>
                  <option value="15mins">15mins</option>
                  <option value="1hour">1hour</option>
                </SelectField>
              </div>

              {/* Memory Usage Card */}
              <SectionCard>
                <div className="flex flex-col gap-6">
                  <h3 className="font-sans font-bold text-xl text-high-contrast m-0">Memory Usage</h3>
                  <div className="-mx-2">
                    <AreaChart />
                  </div>
                </div>
              </SectionCard>

              {/* CPU Usage Card */}
              <SectionCard>
                <div className="flex flex-col gap-6">
                  <h3 className="font-sans font-bold text-xl text-high-contrast m-0">CPU Usage</h3>
                  <div className="-mx-2">
                    <BarChart />
                  </div>
                </div>
              </SectionCard>
            </div>
          )}
        </div>
      </div>
  )
}
