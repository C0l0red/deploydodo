import { motion, AnimatePresence, type HTMLMotionProps } from 'framer-motion'
import { forwardRef } from 'react'

// ─── Stagger container + item variants ──────────────────────────────────────

export const staggerContainerVariants = {
  initial: { opacity: 0 },
  animate: {
    opacity: 1,
    transition: {
      duration: 0.2,
      ease: 'easeOut' as const,
      staggerChildren: 0.05,
      when: 'beforeChildren' as const,
    },
  },
  exit: {
    opacity: 0,
    transition: {
      duration: 0.1,
      ease: 'easeIn' as const,
    },
  },
} as const

export const staggerItemVariants = {
  initial: { opacity: 0, y: 8 },
  animate: {
    opacity: 1,
    y: 0,
    transition: {
      duration: 0.3,
      ease: 'easeOut' as const,
    },
  },
  exit: {
    opacity: 0,
    y: -4,
    transition: {
      duration: 0.1,
      ease: 'easeIn' as const,
    },
  },
} as const

export const StaggerContainer = forwardRef<HTMLDivElement, HTMLMotionProps<"div">>(
  ({ children, ...props }, ref) => (
    <motion.div
      ref={ref}
      initial="initial"
      animate="animate"
      variants={staggerContainerVariants}
      {...props}
    >
      {children}
    </motion.div>
  )
)

export const StaggerItem = forwardRef<HTMLDivElement, HTMLMotionProps<"div">>(
  ({ children, ...props }, ref) => (
    <motion.div
      ref={ref}
      variants={staggerItemVariants}
      {...props}
    >
      {children}
    </motion.div>
  )
)

// ─── Page transition wrapper (for full-page route changes) ──────────────────

const pageVariants = {
  initial: {
    opacity: 0,
    y: 12,
    filter: 'blur(4px)',
  },
  animate: {
    opacity: 1,
    y: 0,
    filter: 'none',
    transition: {
      duration: 0.35,
      ease: [0.25, 0.46, 0.45, 0.94] as const,
      staggerChildren: 0.06,
      when: 'beforeChildren' as const,
    },
  },
  exit: {
    opacity: 0,
    y: -8,
    filter: 'blur(2px)',
    transition: {
      duration: 0.15,
      ease: [0.55, 0.06, 0.68, 0.19] as const,
    },
  },
} as const

/**
 * Wraps page content with a smooth fade-up-blur entrance.
 * Use as a direct child of a route component.
 */
export function PageTransition({
  children,
  className,
  layoutKey,
}: {
  children: React.ReactNode
  className?: string
  layoutKey?: string
}) {
  return (
    <motion.div
      key={layoutKey}
      variants={pageVariants}
      initial="initial"
      animate="animate"
      exit="exit"
      className={className}
    >
      {children}
    </motion.div>
  )
}

// ─── Tab content transition (for tab/sidebar panel changes) ─────────────────

const tabContentVariants = {
  initial: {
    opacity: 0,
    y: 6,
    scale: 0.995,
  },
  animate: {
    opacity: 1,
    y: 0,
    scale: 1,
    transition: {
      duration: 0.25,
      ease: [0.25, 0.46, 0.45, 0.94] as const,
      staggerChildren: 0.04,
      when: 'beforeChildren' as const,
    },
  },
  exit: {
    opacity: 0,
    y: -4,
    scale: 0.998,
    transition: {
      duration: 0.12,
      ease: [0.55, 0.06, 0.68, 0.19] as const,
    },
  },
} as const

/**
 * AnimatePresence wrapper for tab or sidebar panel content.
 * Pass a unique `tabKey` so content animates on change.
 */
export function TabTransition({
  tabKey,
  children,
  className,
}: {
  tabKey: string
  children: React.ReactNode
  className?: string
}) {
  return (
    <AnimatePresence mode="wait">
      <motion.div
        key={tabKey}
        variants={tabContentVariants}
        initial="initial"
        animate="animate"
        exit="exit"
        className={className}
      >
        {children}
      </motion.div>
    </AnimatePresence>
  )
}

// ─── Stagger child variant for tab content items ────────────────────────────

export const tabStaggerItemVariants = {
  initial: { opacity: 0, y: 8 },
  animate: {
    opacity: 1,
    y: 0,
    transition: {
      duration: 0.25,
      ease: [0.25, 0.46, 0.45, 0.94] as const,
    },
  },
} as const

/**
 * Individual stagger item for use inside a TabTransition.
 */
export function TabStaggerItem({
  children,
  className,
}: {
  children: React.ReactNode
  className?: string
}) {
  return (
    <motion.div variants={tabStaggerItemVariants} className={className}>
      {children}
    </motion.div>
  )
}

// Re-export AnimatePresence for convenience
export { AnimatePresence }
