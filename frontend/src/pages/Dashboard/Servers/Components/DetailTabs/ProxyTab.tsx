import { useState } from 'react'
import { Sidebar, SectionCard, SectionHeader, SaveButton, OutlineButton, Toggle, FieldLabel } from '..'

type ProxySidebar = 'Configuration' | 'Dynamic Configuration' | 'Logs'

export function ProxyTab() {
  const [activeProxySidebar, setActiveProxySidebar] = useState<ProxySidebar>('Configuration')

  const [generateLabelsOnly, setGenerateLabelsOnly] = useState(false)
  const [overrideDefaultHandler, setOverrideDefaultHandler] = useState(true)
  const [proxyRedirectTo, setProxyRedirectTo] = useState('https://app.DeployDodo.co')

  const [proxyLogLines, setProxyLogLines] = useState('100')
  const [streamingEnabled, setStreamingEnabled] = useState(true)
  const [includeTimestamps, setIncludeTimestamps] = useState(true)

  const proxyOptions: ProxySidebar[] = ['Configuration', 'Dynamic Configuration', 'Logs']

  return (
    <div className="flex gap-6 items-start">
      <Sidebar options={proxyOptions} active={activeProxySidebar} onChange={setActiveProxySidebar} />

        <div className="flex-1 min-w-0 flex flex-col gap-5">

          {/* ── Proxy > Configuration ── */}
          {activeProxySidebar === 'Configuration' && (
            <div className="flex flex-col gap-5">
              <SectionCard>
                <SectionHeader
                  title="Configuration"
                  subtitle="Configure your proxy settings and advanced options."
                  right={
                    <div className="flex items-center gap-2">
                      <OutlineButton>Switch proxy</OutlineButton>
                      <SaveButton />
                    </div>
                  }
                />

                {/* Advanced sub-section */}
                <div className="flex flex-col gap-5">
                  <h3 className="font-sans font-semibold text-lg leading-7 text-high-contrast m-0">Advanced</h3>

                  <div className="flex items-center justify-between">
                    <FieldLabel>Generate labels only for Traefik</FieldLabel>
                    <Toggle enabled={generateLabelsOnly} onToggle={() => setGenerateLabelsOnly(!generateLabelsOnly)} />
                  </div>

                  <div className="flex items-center justify-between">
                    <FieldLabel>Override default request handler</FieldLabel>
                    <Toggle enabled={overrideDefaultHandler} onToggle={() => setOverrideDefaultHandler(!overrideDefaultHandler)} />
                  </div>

                  <div className="flex flex-col gap-2">
                    <FieldLabel>Redirect to (optional)</FieldLabel>
                    <input
                      type="text"
                      value={proxyRedirectTo}
                      onChange={(e) => setProxyRedirectTo(e.target.value)}
                      className="w-full bg-background border border-neutral-100 rounded-lg px-3 py-2 font-manrope font-normal text-sm leading-6 text-secondary outline-none focus:border-secondary transition-[border-color] duration-150"
                    />
                  </div>
                </div>
              </SectionCard>

              {/* Traefik docker-compose card */}
              <SectionCard>
                <SectionHeader
                  title="Traefik (DeployDodo Proxy)"
                  subtitle="Configuration file ( /data/DeployDodo/proxy/docker-compose.yml )"
                />
                <div className="border border-neutral-100 rounded-xl py-3 bg-white font-mono text-sm leading-relaxed text-secondary select-text">
                  {[
                    'name: DeployDodo-proxy',
                    'networks:',
                    '  DeployDodo:',
                    '    external: true',
                    'services:',
                    '  traefik:',
                    '    container_name: DeployDodo-proxy',
                    '    image: "traefik:v3.1"',
                    '    restart: unless-stopped',
                    '    extra_hosts:',
                    '      - "host.docker.internal:host-gateway"',
                    '    networks:',
                  ].map((line, i) => (
                    <div key={i} className="flex leading-6 py-0.5 min-h-[24px]">
                      <span className="w-10 shrink-0 text-right pr-4 text-text-secondary/40 select-none font-mono text-sm">{i + 1}</span>
                      <span className="flex-1 font-mono text-sm text-high-contrast whitespace-pre select-text">{line}</span>
                    </div>
                  ))}
                </div>
              </SectionCard>
            </div>
          )}

          {/* ── Proxy > Dynamic Configuration ── */}
          {activeProxySidebar === 'Dynamic Configuration' && (
            <SectionCard>
              <SectionHeader
                title="Dynamic Configuration"
                subtitle="Server is reachable and validated"
                right={
                  <div className="flex items-center gap-2">
                    <OutlineButton>Switch proxy</OutlineButton>
                    <SaveButton />
                  </div>
                }
              />

              <div className="flex flex-col gap-2">
                <FieldLabel>File: Caddyfile</FieldLabel>
                <textarea
                  readOnly
                  rows={4}
                  defaultValue="import dynamic/*.caddy"
                  className="w-full bg-background border border-neutral-100 rounded-lg px-3 py-2 font-mono font-normal text-sm text-secondary outline-none focus:border-secondary resize-y transition-[border-color] duration-150"
                />
              </div>

              <div className="flex flex-col gap-2">
                <FieldLabel>Custom FluentBit Configuration</FieldLabel>
                <textarea
                  readOnly
                  rows={16}
                  defaultValue={`# This file is generated by DeployDodo, do not edit it manually.
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
                  className="w-full bg-background border border-neutral-100 rounded-lg px-3 py-2 font-mono font-normal text-xs leading-relaxed text-secondary outline-none focus:border-secondary resize-y transition-[border-color] duration-150"
                />
              </div>
            </SectionCard>
          )}

          {/* ── Proxy > Logs ── */}
          {activeProxySidebar === 'Logs' && (
            <div className="flex flex-col gap-4">
              <h2 className="font-sans font-bold text-3xl text-high-contrast m-0">Logs</h2>
              <SectionCard>
                <SectionHeader
                  title="DeployDodo Proxy"
                  right={<OutlineButton>Refresh</OutlineButton>}
                />

                <div className="flex flex-col gap-4">
                  {/* Only show number of lines */}
                  <div className="flex flex-col gap-2">
                    <FieldLabel>Only Show Number of Lines*</FieldLabel>
                    <input
                      type="text"
                      value={proxyLogLines}
                      onChange={(e) => setProxyLogLines(e.target.value)}
                      className="w-full bg-background border border-neutral-100 rounded-lg px-3 py-2 font-manrope font-normal text-sm leading-6 text-secondary outline-none focus:border-secondary transition-[border-color] duration-150 max-w-xs"
                    />
                  </div>

                  {/* Toggle row: Stream Logs & Include Timestamps */}
                  <div className="flex items-center gap-6 mt-2">
                    <div className="flex items-center gap-3">
                      <span className="font-sans font-normal text-sm leading-6 text-secondary">Stream Logs</span>
                      <Toggle enabled={streamingEnabled} onToggle={() => setStreamingEnabled(!streamingEnabled)} />
                    </div>
                    <div className="flex items-center gap-3">
                      <span className="font-sans font-normal text-sm leading-6 text-secondary">Include Timestamps</span>
                      <Toggle enabled={includeTimestamps} onToggle={() => setIncludeTimestamps(!includeTimestamps)} />
                    </div>
                  </div>
                </div>

                {/* Log viewer */}
                <div className="flex flex-col gap-2">
                  <FieldLabel>File: Caddyfile</FieldLabel>
                  <textarea
                    readOnly
                    rows={6}
                    defaultValue={`2025-11-17T15:46:51.611087579Z 2025-11-17T15:46:51.611Z ERR Error while peeking first byte: error="read tcp 10.0.3.56:54316->10.0.3.56:80: read: connection reset by peer"
2025-11-17T15:47:52.611087579Z 2025-11-17T15:47:52.611Z ERR Error while peeking first byte: error="read tcp 10.0.3.56:54316->10.0.3.56:80: read: connection reset by peer"`}
                    className="w-full bg-background border border-neutral-100 rounded-lg px-3 py-2 font-mono font-normal text-xs leading-relaxed text-secondary outline-none focus:border-secondary resize-y transition-[border-color] duration-150"
                  />
                </div>
              </SectionCard>
            </div>
          )}
        </div>
      </div>
  )
}
