import { useFormik } from 'formik'
import * as Yup from 'yup'
import { ArrowBackIcon, WarningCircleIcon } from '@/assets/icons'
import { cn } from '@/utilities/cn'
import { useCreateRemoteServer } from '@/api/mutations'
import { Card } from '@/layouts/PageLayout'

type AuthMethod = 'ssh-key' | 'password'

type FormValues = {
  name: string
  hostname: string
  port: string
  username: string
  authMethod: AuthMethod
  privateKey: string
  password: string
}

const validationSchema = Yup.object({
  name: Yup.string().trim().required('Server name is required'),
  hostname: Yup.string().trim().required('Host / IP address is required'),
  port: Yup.number()
    .typeError('Port must be a number')
    .integer('Port must be a whole number')
    .min(1, 'Port must be between 1 and 65535')
    .max(65535, 'Port must be between 1 and 65535')
    .required('SSH port is required'),
  username: Yup.string().trim().required('Username is required'),
  privateKey: Yup.string().when('authMethod', {
    is: 'ssh-key',
    then: (s) => s.trim().required('Private key is required'),
    otherwise: (s) => s,
  }),
  password: Yup.string().when('authMethod', {
    is: 'password',
    then: (s) => s.required('Password is required'),
    otherwise: (s) => s,
  }),
})

function FormField({
  label,
  helperText,
  error,
  children,
}: {
  label: string
  helperText?: string
  error?: string
  children: React.ReactNode
}) {
  return (
    <div className="flex flex-col gap-1">
      <label className="font-manrope font-bold text-base leading-6 text-high-contrast">
        {label}
      </label>
      {children}
      {error ? (
        <span className="font-manrope font-normal text-xs leading-4 text-error pl-3">{error}</span>
      ) : helperText ? (
        <span className="font-manrope font-normal text-xs leading-4 text-[#999ca0] pl-3">
          {helperText}
        </span>
      ) : null}
    </div>
  )
}

function FieldInput({
  name,
  value,
  onChange,
  onBlur,
  placeholder,
  type = 'text',
  hasError = false,
}: {
  name: string
  value: string
  onChange: React.ChangeEventHandler<HTMLInputElement>
  onBlur: React.FocusEventHandler<HTMLInputElement>
  placeholder: string
  type?: string
  hasError?: boolean
}) {
  return (
    <input
      name={name}
      value={value}
      onChange={onChange}
      onBlur={onBlur}
      type={type}
      placeholder={placeholder}
      className={cn(
        'bg-background border rounded-lg px-3 py-2 font-manrope font-normal text-sm leading-6 text-text-secondary outline-none transition-[border-color] duration-150 placeholder:text-text-secondary focus:border-high-contrast w-full',
        hasError ? 'border-error!' : 'border-neutral-100',
      )}
    />
  )
}

const whatHappensNext = [
  {
    label: 'Connection Test:',
    body: 'DeployDodo will establish an SSH connection to verify credentials and server accessibility.',
  },
  {
    label: 'Docker Check:',
    body: "We'll verify Docker is installed or install it automatically.",
  },
]

export function RemoteServerView({
  onBack,
  onConnect,
}: {
  onBack: () => void
  onConnect: (jobId: string) => void
}) {
  const createRemoteServer = useCreateRemoteServer()

  const formik = useFormik<FormValues>({
    initialValues: {
      name: '',
      hostname: '',
      port: '22',
      username: 'root',
      authMethod: 'ssh-key',
      privateKey: '',
      password: '',
    },
    validationSchema,
    onSubmit: (values) => {
      const auth =
        values.authMethod === 'ssh-key'
          ? ({
              authType: 'keypair',
              username: values.username,
              privateKey: values.privateKey,
            } as const)
          : ({
              authType: 'password',
              username: values.username,
              password: values.password,
            } as const)

      createRemoteServer.mutate(
        {
          name: values.name.trim(),
          hostname: values.hostname.trim(),
          port: parseInt(values.port, 10),
          auth,
        },
        {
          onSuccess: ({ data, error }) => {
            if (error || !data) return
            onConnect(data.jobId)
          },
        },
      )
    },
  })

  const authMethod = formik.values.authMethod
  const isPending = createRemoteServer.isPending

  return (
    <>
      <Card className="p-8">
        <div className="flex flex-col gap-6">
          <button
            type="button"
            onClick={onBack}
            className="text-high-contrast hover:opacity-70 transition-opacity shrink-0"
            aria-label="Go back"
          >
            <ArrowBackIcon />
          </button>

          <div className="flex flex-col gap-2">
            <h2 className="font-sans font-semibold text-2xl leading-8 text-high-contrast m-0">
              Connect Remote Server
            </h2>
            <p className="font-sans font-normal text-lg leading-7 text-high-contrast m-0">
              Enter your server details to establish SSH connection and configure the deployment
              environment.
            </p>
          </div>

          <form onSubmit={formik.handleSubmit} className="flex flex-col gap-6" noValidate>
            <FormField
              label="Server Name"
              helperText="A friendly name to identify this server"
              error={formik.touched.name ? formik.errors.name : undefined}
            >
              <FieldInput
                name="name"
                value={formik.values.name}
                onChange={formik.handleChange}
                onBlur={formik.handleBlur}
                placeholder="e.g. Production server"
                hasError={formik.touched.name && !!formik.errors.name}
              />
            </FormField>

            <FormField
              label="Host / IP Address"
              helperText="The IP address or hostname of your remote server"
              error={formik.touched.hostname ? formik.errors.hostname : undefined}
            >
              <FieldInput
                name="hostname"
                value={formik.values.hostname}
                onChange={formik.handleChange}
                onBlur={formik.handleBlur}
                placeholder="e.g. 192.168.1.100 or server.example.com"
                hasError={formik.touched.hostname && !!formik.errors.hostname}
              />
            </FormField>

            <div className="flex gap-6">
              <FormField
                label="SSH Port"
                helperText="Default: 22"
                error={formik.touched.port ? formik.errors.port : undefined}
              >
                <FieldInput
                  name="port"
                  value={formik.values.port}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  placeholder="22"
                  hasError={formik.touched.port && !!formik.errors.port}
                />
              </FormField>
              <FormField
                label="Username"
                helperText="SSH user (recommended: root)"
                error={formik.touched.username ? formik.errors.username : undefined}
              >
                <FieldInput
                  name="username"
                  value={formik.values.username}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  placeholder="root"
                  hasError={formik.touched.username && !!formik.errors.username}
                />
              </FormField>
            </div>

            <div className="flex flex-col gap-3">
              <span className="font-manrope font-bold text-base leading-6 text-high-contrast">
                Authentication Method
              </span>
              <div className="inline-flex border border-secondary rounded-xl p-1 gap-1 w-fit">
                {(['ssh-key', 'password'] as AuthMethod[]).map((method) => (
                  <button
                    key={method}
                    type="button"
                    onClick={() => formik.setFieldValue('authMethod', method)}
                    className={[
                      'rounded-lg px-2 py-1 h-8 font-manrope font-bold text-sm leading-6 transition-colors duration-150',
                      authMethod === method ? 'bg-secondary text-pure-white' : 'text-high-contrast',
                    ].join(' ')}
                  >
                    {method === 'ssh-key' ? 'SSH Private Key' : 'Password'}
                  </button>
                ))}
              </div>
            </div>

            {authMethod === 'ssh-key' ? (
              <div className="flex flex-col gap-1">
                <textarea
                  name="privateKey"
                  value={formik.values.privateKey}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  placeholder="-----BEGIN OPENSSH PRIVATE KEY-----"
                  className={cn(
                    'bg-background border rounded-lg px-3.75 py-3.75 h-33 font-sans font-normal text-base leading-6 text-text-secondary outline-none resize-none transition-[border-color] duration-150 focus:border-high-contrast w-full',
                    formik.touched.privateKey && formik.errors.privateKey
                      ? 'border-error!'
                      : 'border-neutral-100',
                  )}
                />
                {formik.touched.privateKey && formik.errors.privateKey && (
                  <span className="font-manrope font-normal text-xs leading-4 text-error pl-3">
                    {formik.errors.privateKey}
                  </span>
                )}
              </div>
            ) : (
              <FormField
                label="Password"
                helperText="SSH password for authentication"
                error={formik.touched.password ? formik.errors.password : undefined}
              >
                <FieldInput
                  name="password"
                  value={formik.values.password}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  placeholder="Enter password"
                  type="password"
                  hasError={formik.touched.password && !!formik.errors.password}
                />
              </FormField>
            )}

            {authMethod === 'ssh-key' && (
              <div className="bg-[rgba(255,122,73,0.12)] border border-primary-darker rounded-lg p-3 flex flex-col gap-2">
                <div className="flex items-center gap-1">
                  <WarningCircleIcon className="shrink-0 w-5 h-5" />
                  <span className="font-manrope font-bold text-sm leading-6 text-high-contrast">
                    How to get your private key:
                  </span>
                </div>
                <p className="font-manrope font-normal text-sm leading-6 text-high-contrast m-0">
                  Run <code>cat ~/.ssh/id_rsa</code> or <code>cat ~/.ssh/id_ed25519</code> on your
                  local machine
                </p>
              </div>
            )}

            {!!createRemoteServer.data?.error && (
              <p className="font-manrope font-normal text-sm leading-6 text-error">
                Failed to start connection. Please check your details and try again.
              </p>
            )}

            <button
              type="submit"
              disabled={isPending}
              className="w-full bg-secondary text-pure-white font-manrope font-bold text-sm leading-6 rounded-lg px-4 py-2 hover:opacity-[0.88] active:opacity-75 transition-opacity duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isPending ? 'Connecting…' : 'Connect & Continue'}
            </button>
          </form>
        </div>
      </Card>

      <Card className="p-8">
        <div className="flex flex-col gap-3">
          <h3 className="font-sans font-semibold text-lg leading-7 text-primary-darker m-0">
            What Happens Next
          </h3>
          <div className="flex flex-col gap-2">
            {whatHappensNext.map((item) => (
              <p
                key={item.label}
                className="font-sans font-normal text-base leading-6 text-secondary m-0"
              >
                <span className="font-manrope font-bold">{item.label}</span> {item.body}
              </p>
            ))}
          </div>
        </div>
      </Card>
    </>
  )
}
