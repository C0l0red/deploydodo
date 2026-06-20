import { useState } from 'react'
import { useFormik } from 'formik'
import * as Yup from 'yup'
import { TextInput } from '@/components/TextInput'
import { Button } from '@/components/Button'
import { useToast } from '@/components/Toast'
import { useCreateAdmin } from '@/api/mutations'
import { EyeOpenIcon, EyeClosedIcon } from '@/assets/icons'

const RULES = [
  { label: 'At least 8 characters', test: (p: string) => p.length >= 8 },
  { label: 'One uppercase letter', test: (p: string) => /[A-Z]/.test(p) },
  { label: 'One lowercase letter', test: (p: string) => /[a-z]/.test(p) },
  { label: 'One number', test: (p: string) => /[0-9]/.test(p) },
  { label: 'One symbol', test: (p: string) => /[^A-Za-z0-9]/.test(p) },
]

const EyeIcon = ({ open }: { open: boolean }) => (open ? <EyeOpenIcon /> : <EyeClosedIcon />)

function PasswordGuidelines({ password }: { password: string }) {
  return (
    <ul className="flex flex-col gap-1.5">
      {RULES.map(({ label, test }) => {
        const passing = password.length > 0 && test(password)
        return (
          <li key={label} className="flex items-center gap-2">
            <svg
              width="14"
              height="14"
              viewBox="0 0 14 14"
              fill="none"
              className={passing ? 'text-green-500' : 'text-neutral-300'}
            >
              <circle cx="7" cy="7" r="7" fill="currentColor" opacity="0.2" />
              <path
                d="M4 7l2 2 4-4"
                stroke="currentColor"
                strokeWidth="1.5"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </svg>
            <span
              className={`font-sans text-xs leading-5 transition-colors duration-150 ${passing ? 'text-green-600' : 'text-text-secondary'}`}
            >
              {label}
            </span>
          </li>
        )
      })}
    </ul>
  )
}

const schema = Yup.object({
  password: Yup.string()
    .min(8, 'Password must be at least 8 characters')
    .matches(/[A-Z]/, 'Must contain at least one uppercase letter')
    .matches(/[a-z]/, 'Must contain at least one lowercase letter')
    .matches(/[0-9]/, 'Must contain at least one number')
    .matches(/[^A-Za-z0-9]/, 'Must contain at least one symbol')
    .required('Password is required'),
  confirmPassword: Yup.string()
    .oneOf([Yup.ref('password')], 'Passwords do not match')
    .required('Please re-enter your password'),
})

type ChoosePasswordProps = {
  name: string
  email: string
  onBack: () => void
  onCreateAccount: () => void
}

export function ChoosePassword({ name, email, onBack, onCreateAccount }: ChoosePasswordProps) {
  const [showPassword, setShowPassword] = useState(false)
  const [showConfirm, setShowConfirm] = useState(false)
  const { toast } = useToast()

  const createAdmin = useCreateAdmin({
    onSuccess: ({ data, error }) => {
      if (error || !data) return
      localStorage.setItem('session_token', data.sessionToken)
      toast('Account created successfully', 'success')
      onCreateAccount()
    },
  })

  const formik = useFormik({
    initialValues: { password: '', confirmPassword: '' },
    validationSchema: schema,
    onSubmit: (values) => createAdmin.mutate({ name, email, password: values.password }),
  })

  return (
    <div className="bg-background rounded-xl shadow-[0px_8px_24px_0px_rgba(0,0,0,0.08),0px_0px_1px_0px_rgba(0,0,0,0.2)] w-[456px] max-w-full px-6 pt-4 pb-2.5">
      <div className="flex flex-col gap-6">
        <div className="flex items-center gap-3">
          <button
            type="button"
            onClick={onBack}
            className="text-secondary hover:opacity-70 transition-opacity"
            aria-label="Go back"
          >
            <svg
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <line x1="19" y1="12" x2="5" y2="12" />
              <polyline points="12 19 5 12 12 5" />
            </svg>
          </button>
          <h1 className="font-sans font-semibold text-2xl leading-8 text-secondary m-0">
            Choose a password
          </h1>
        </div>

        <form className="flex flex-col gap-4" onSubmit={formik.handleSubmit}>
          <div className="flex flex-col gap-4">
            <TextInput
              label="Password"
              name="password"
              type={showPassword ? 'text' : 'password'}
              placeholder="••••••••••••"
              value={formik.values.password}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              hasError={formik.touched.password && !!formik.errors.password}
              errorMessage={formik.errors.password}
              suffix={
                <button type="button" onClick={() => setShowPassword((v) => !v)}>
                  <EyeIcon open={showPassword} />
                </button>
              }
            />
            <PasswordGuidelines password={formik.values.password} />
            <TextInput
              label="Re-enter password"
              name="confirmPassword"
              type={showConfirm ? 'text' : 'password'}
              placeholder="••••••••••••"
              value={formik.values.confirmPassword}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              hasError={formik.touched.confirmPassword && !!formik.errors.confirmPassword}
              errorMessage={formik.errors.confirmPassword}
              suffix={
                <button type="button" onClick={() => setShowConfirm((v) => !v)}>
                  <EyeIcon open={showConfirm} />
                </button>
              }
            />
            {createAdmin.error && (
              <p className="font-sans font-normal text-sm leading-6 text-error">
                Failed to create account. Please try again.
              </p>
            )}
          </div>

          <div className="flex flex-col gap-4 pb-6">
            <Button type="submit" fullWidth disabled={createAdmin.isPending}>
              {createAdmin.isPending ? 'Creating account…' : 'Create account'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  )
}
