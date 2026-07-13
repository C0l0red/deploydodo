import { useState } from 'react'
import { createLazyRoute, useNavigate } from '@tanstack/react-router'
import { Logo } from '@/components/Logo'
import { ChoosePassword } from './ChoosePassword'
import { CreateAdminAccount } from './CreateAdminAccount'
import { Pending } from '../Pending/Pending'

type Step = 'create-account' | 'choose-password'

export const OnboardingRoute = createLazyRoute('onboarding')({
  pendingComponent: Pending,
  component: Onboarding,
})

function Onboarding() {
  const [step, setStep] = useState<Step>('create-account')
  const [accountInfo, setAccountInfo] = useState({ name: '', email: '' })
  const navigate = useNavigate()

  function handleProceed(name: string, email: string) {
    setAccountInfo({ name, email })
    setStep('choose-password')
  }

  function getActiveStep() {
    if (step === 'choose-password') {
      return (
        <ChoosePassword
          name={accountInfo.name}
          email={accountInfo.email}
          onBack={() => setStep('create-account')}
          onCreateAccount={() => {
            navigate({
              to: '/welcome',
            })
          }}
        />
      )
    }

    return <CreateAdminAccount onProceed={handleProceed} />
  }

  return (
    <div className="w-full min-h-screen bg-linear-to-b from-[rgba(255,122,73,0.01)] to-[rgba(252,140,99,0.12)] flex items-center justify-center py-10 px-4">
      <div className="flex flex-col items-center gap-5">
        <Logo size="lg" />
        {getActiveStep()}
      </div>
    </div>
  )
}
