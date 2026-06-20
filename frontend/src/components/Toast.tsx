import { createContext, useContext, useState, useCallback, useRef, type ReactNode } from 'react'
import { createPortal } from 'react-dom'
import { CheckCircleIcon, WarningCircleIcon } from '@/assets/icons'

type ToastVariant = 'success' | 'error' | 'info'

type ToastItem = {
  id: number
  message: string
  variant: ToastVariant
}

type ToastContextValue = {
  toast: (message: string, variant?: ToastVariant) => void
}

const ToastContext = createContext<ToastContextValue | null>(null)

export function useToast() {
  const ctx = useContext(ToastContext)
  if (!ctx) throw new Error('useToast must be used within ToastProvider')
  return ctx
}

const iconMap: Record<ToastVariant, ReactNode> = {
  success: <CheckCircleIcon className="size-5 shrink-0" />,
  error: <WarningCircleIcon className="size-5 shrink-0 text-error" />,
  info: (
    <svg className="size-5 shrink-0 text-secondary" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <circle cx="12" cy="12" r="10" />
      <line x1="12" y1="16" x2="12" y2="12" />
      <line x1="12" y1="8" x2="12.01" y2="8" />
    </svg>
  ),
}

const bgMap: Record<ToastVariant, string> = {
  success: 'bg-[#edf7ed]',
  error: 'bg-[#fdeded]',
  info: 'bg-neutral-200',
}

const borderMap: Record<ToastVariant, string> = {
  success: 'border-[#4caf50]',
  error: 'border-error',
  info: 'border-neutral-100',
}

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<ToastItem[]>([])
  const nextId = useRef(0)

  const remove = useCallback((id: number) => {
    setToasts((prev) => prev.filter((t) => t.id !== id))
  }, [])

  const toast = useCallback(
    (message: string, variant: ToastVariant = 'success') => {
      const id = nextId.current++
      setToasts((prev) => [...prev, { id, message, variant }])
      setTimeout(() => remove(id), 4000)
    },
    [remove],
  )

  return (
    <ToastContext.Provider value={{ toast }}>
      {children}
      {createPortal(
        <div className="fixed bottom-4 right-4 z-[100] flex flex-col gap-2 pointer-events-none">
          {toasts.map((item) => (
            <div
              key={item.id}
              className={[
                'pointer-events-auto flex items-center gap-2.5 px-4 py-3 rounded-lg border shadow-lg',
                'animate-[slideIn_0.25s_ease-out]',
                bgMap[item.variant],
                borderMap[item.variant],
              ].join(' ')}
              style={{
                animation: 'slideIn 0.25s ease-out',
                maxWidth: 360,
              }}
            >
              {iconMap[item.variant]}
              <span className="font-manrope font-normal text-sm leading-5 text-secondary flex-1">
                {item.message}
              </span>
              <button
                onClick={() => remove(item.id)}
                className="size-5 flex items-center justify-center rounded hover:bg-black/5 transition-colors shrink-0"
              >
                <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round">
                  <line x1="2" y1="2" x2="10" y2="10" />
                  <line x1="10" y1="2" x2="2" y2="10" />
                </svg>
              </button>
            </div>
          ))}
        </div>,
        document.body,
      )}
      <style>{`
        @keyframes slideIn {
          from { transform: translateX(100%); opacity: 0; }
          to { transform: translateX(0); opacity: 1; }
        }
      `}</style>
    </ToastContext.Provider>
  )
}
