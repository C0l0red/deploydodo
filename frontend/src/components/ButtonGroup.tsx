import { cn } from '@/utilities/cn'
import { useMemo, useState } from 'react'

interface ButtonGroupItem<T> {
  label: string
  value: T
}

interface ButtonGroupProps<T> {
  onSelect?: (option: T) => void
  options: ButtonGroupItem<T>[]
  label: string
  value?: T
}

export function ButtonGroup<T>({ onSelect, label, options, value }: ButtonGroupProps<T>) {
  if (options.length < 2) {
    throw new Error('You must pass at least 2 items to ButtonGroup')
  }

  const [innerValue, setInnerValue] = useState(options[0].value)

  const resolvedValue = useMemo(() => value ?? innerValue, [value, innerValue])

  const onOptionSelected = (value: T) => {
    setInnerValue(value)
    onSelect?.(value)
  }

  const selectedIndex = Math.max(
    options.findIndex((item) => item.value === resolvedValue),
    0,
  )

  return (
    <div className="flex flex-col gap-2">
      <label className="font-sans text-base leading-6 text-secondary font-bold">{label}</label>
      <div className="relative flex rounded-lg border border-neutral-100 overflow-hidden">
        <div
          className="absolute inset-y-0 left-0 bg-secondary transition-transform duration-200 ease-out"
          style={{
            width: `${100 / options.length}%`,
            transform: `translateX(${selectedIndex * 100}%)`,
          }}
        />
        {options.map((item, i) => (
          <button
            key={i}
            type="button"
            onClick={() => onOptionSelected(item.value)}
            className={cn(
              'relative z-10 flex-1 py-2 font-manrope text-sm leading-6 transition-colors',
              resolvedValue === item.value
                ? 'text-pure-white font-bold'
                : 'text-text-secondary font-normal hover:text-high-contrast',
            )}
          >
            {item.label}
          </button>
        ))}
      </div>
    </div>
  )
}
