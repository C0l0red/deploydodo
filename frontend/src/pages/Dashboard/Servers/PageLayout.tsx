export function PageLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="w-full min-h-screen bg-linear-to-b from-[rgba(255,122,73,0.01)] to-[rgba(252,140,99,0.12)] flex items-center justify-center py-[120px] px-4">
      <div className="flex flex-col items-center gap-5 w-full max-w-[904px]">{children}</div>
    </div>
  )
}

export function Card({ children, className }: { children: React.ReactNode; className?: string }) {
  return (
    <div
      className={[
        'bg-background rounded-xl shadow-[0px_8px_24px_0px_rgba(0,0,0,0.08),0px_0px_1px_0px_rgba(0,0,0,0.2)] w-full',
        className ?? '',
      ].join(' ')}
    >
      {children}
    </div>
  )
}
