import { cn } from '@/utilities/cn'
import { type TextareaHTMLAttributes } from 'react'

type TextareaProps = {
  label: string
  errorMessage?: string
  hasError?: boolean
} & TextareaHTMLAttributes<HTMLTextAreaElement>

export function Textarea({
  label,
  id,
  errorMessage,
  hasError = false,
  className,
  ...props
}: TextareaProps) {
  const textareaId = id ?? label.toLowerCase().replace(/\s+/g, '-')
  return (
    <div className="flex flex-col gap-2">
      <label
        htmlFor={textareaId}
        className="font-sans font-normal text-base leading-6 text-secondary"
      >
        {label}
      </label>
      <textarea
        id={textareaId}
        className={cn(
          'w-full bg-background border border-neutral-100 rounded-lg px-3 py-2 font-mono font-normal text-xs leading-5 text-secondary outline-none resize-none focus:border-secondary transition-[border-color] duration-150',
          { 'border-error!': hasError },
          className,
        )}
        {...props}
      />
      {hasError && errorMessage && (
        <p className="font-sans font-normal text-sm leading-6 text-error m-0">{errorMessage}</p>
      )}
    </div>
  )
}
