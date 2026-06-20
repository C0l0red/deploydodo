import { useState } from 'react'
import { useNavigate } from '@tanstack/react-router'
import { useFormik } from 'formik'
import * as Yup from 'yup'
import { TextInput } from '@/components/TextInput'
import { Button } from '@/components/Button'
import { LogoIcon, EyeOpenIcon, EyeClosedIcon } from '@/assets/icons'
import { useToast } from '@/components/Toast'
import { loginRoute } from './route'
import { api } from '@/api/client'

export function Login() {
  const navigate = useNavigate()
  const { toast } = useToast()
  const status = loginRoute.useLoaderData()
  const [showPassword, setShowPassword] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [isSubmitting, setIsSubmitting] = useState(false)

  const formik = useFormik({
    initialValues: { email: '', password: '' },
    validationSchema: Yup.object({
      email: Yup.string().email('Invalid email').required('Email is required'),
      password: Yup.string().required('Password is required'),
    }),
    onSubmit: async (values) => {
      setError(null)
      setIsSubmitting(true)
      const { data, error } = await api.POST('/api/auth/login', {
        body: { email: values.email.trim(), password: values.password },
      })
      setIsSubmitting(false)

      if (error || !data) {
        setError('Invalid email or password')
        return
      }

      localStorage.setItem('session_token', data.sessionToken)
      toast('Welcome back', 'success')

      const { getServers } = await import('@/api/queries')
      const servers = await getServers()
      navigate({ to: servers.length > 0 ? '/dashboard' : '/welcome', replace: true })
    },
  })

  return (
    <div className="w-full min-h-screen bg-linear-to-b from-[rgba(255,122,73,0.01)] to-[rgba(252,140,99,0.12)] flex items-center justify-center py-10 px-4">
      <div className="bg-background rounded-xl shadow-[0px_8px_24px_0px_rgba(0,0,0,0.08),0px_0px_1px_0px_rgba(0,0,0,0.2)] w-[456px] max-w-full px-6 pt-4 pb-2.5">
        <div className="flex flex-col gap-6">
          <div className="flex flex-col items-center gap-3 pt-6">
            <div className="w-9 h-9 bg-primary-darker rounded-[10.8px] flex items-center justify-center">
              <LogoIcon />
            </div>
            <h1 className="font-sans font-semibold text-2xl leading-8 text-secondary m-0">
              Sign in to DeployDodo
            </h1>
          </div>

          <form className="flex flex-col gap-4" onSubmit={formik.handleSubmit}>
            <TextInput
              label="Email"
              name="email"
              type="email"
              placeholder="you@example.com"
              value={formik.values.email}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              hasError={formik.touched.email && !!formik.errors.email}
              errorMessage={formik.errors.email}
            />
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
                  {showPassword ? <EyeOpenIcon /> : <EyeClosedIcon />}
                </button>
              }
            />
            {error && (
              <p className="font-sans font-normal text-sm leading-6 text-error m-0">{error}</p>
            )}
            <div className="flex flex-col gap-4 pb-6">
              <Button type="submit" fullWidth disabled={isSubmitting}>
                {isSubmitting ? (
                  <>
                    <svg
                      className="animate-spin -ml-1 mr-2 h-4 w-4"
                      viewBox="0 0 24 24"
                      fill="none"
                    >
                      <circle
                        className="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        strokeWidth="4"
                      />
                      <path
                        className="opacity-75"
                        fill="currentColor"
                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
                      />
                    </svg>
                    Signing in...
                  </>
                ) : (
                  'Sign in'
                )}
              </Button>
            </div>
          </form>
        </div>
      </div>
    </div>
  )
}
