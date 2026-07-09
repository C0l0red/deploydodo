import { motion } from 'framer-motion'
import { PlusIcon } from '@/assets/icons'
import { StaggerContainer, StaggerItem, staggerItemVariants } from '@/components/Animated'

export function SharedVariables() {
  return (
    <StaggerContainer className="flex items-end justify-between">
      <StaggerItem className="flex flex-col gap-2">
        <h1 className="font-sans font-semibold text-[40px] leading-12 tracking-[-0.5px] text-high-contrast m-0">
          Shared Variables
        </h1>
        <p className="font-sans font-normal text-base leading-6 text-text-secondary m-0">
          Define variables shared across your deployments
        </p>
      </StaggerItem>
      <motion.button
        variants={staggerItemVariants}
        className="flex items-center gap-2 pl-2 pr-4 py-2 border border-text-secondary rounded-lg hover:bg-neutral-200 transition-colors"
      >
        <PlusIcon className="size-4 shrink-0" />
        <span className="font-manrope font-bold text-sm leading-6 text-high-contrast">
          New
        </span>
      </motion.button>
    </StaggerContainer>
  )
}
