import { cn } from '@/utilities/cn'

export function Sidebar<T extends string>({ options, active, onChange, }: { options: T[]; active: T; onChange: (v: T) => void }) {
  return (
    <div className="w-44 flex flex-col gap-0.5 shrink-0">
      {options.map((opt) => (
        <button
          key={opt}
          onClick={() => onChange(opt)}
          className={cn(
            'w-full text-left font-manrope text-sm leading-6 px-3 py-1.5 rounded-lg transition-colors duration-150 outline-none whitespace-nowrap',
            active === opt
              ? 'font-bold bg-neutral-200 text-high-contrast'
              : 'font-normal text-text-secondary hover:bg-neutral-100 hover:text-high-contrast'
          )}
        >
          {opt}
        </button>
      ))}
    </div>
  )
}
