import { useFormik } from 'formik'
import { ArrowBackIcon, WarningCircleIcon } from '@/assets/icons'
import { cn } from '@/utilities/cn'
import { useCreateRemoteServer } from '@/api/mutations'
import { Card } from '@/pages/Dashboard/Servers/PageLayout'
import { addServerValidationSchema } from '@/validations/addServer'
import { Button } from '@/components/Button'
import { ButtonGroup } from '@/components/ButtonGroup'
import { Textarea } from '@/components/Textarea'
import { TextInput } from '@/components/TextInput'

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
  const createRemoteServer = useCreateRemoteServer({
    onSuccess: (data) => {
      onConnect(data.jobId)
    },
  })

  const formik = useFormik({
    initialValues: {
      name: '',
      hostname: '',
      port: '22',
      username: 'root',
      authMethod: 'password',
      privateKey: '',
      password: '',
    },
    validationSchema: addServerValidationSchema,
    onSubmit: (values) => {
      const auth =
        values.authMethod === 'keypair'
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

      createRemoteServer.mutate({
        name: values.name.trim(),
        hostname: values.hostname.trim(),
        port: Number(values.port),
        auth,
      })
    },
  })

  const isPending = createRemoteServer.isPending
  const isKeyPair = formik.values.authMethod === 'keypair'

  return (
    <>
      <Card className="p-8">
        <div className="flex flex-col gap-6">
          <Button
            type="button"
            onClick={onBack}
            className="text-high-contrast hover:opacity-70 transition-opacity shrink-0"
            aria-label="Go back"
          >
            <ArrowBackIcon />
          </Button>

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
            <TextInput
              label="Server Name"
              name="name"
              value={formik.values.name}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              placeholder="e.g. Production server"
              helperText="A friendly name to identify this server"
              hasError={formik.touched.name && !!formik.errors.name}
              errorMessage={formik.errors.name}
            />

            <TextInput
              label="Host / IP Address"
              name="hostname"
              value={formik.values.hostname}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              placeholder="e.g. 192.168.1.100 or server.example.com"
              helperText="The IP address or hostname of your remote server"
              hasError={formik.touched.hostname && !!formik.errors.hostname}
              errorMessage={formik.errors.hostname}
            />

            <div className="flex gap-6">
              <div className="flex-1">
                <TextInput
                  label="SSH Port"
                  name="port"
                  value={formik.values.port}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  placeholder="22"
                  helperText="Default: 22"
                  hasError={formik.touched.port && !!formik.errors.port}
                  errorMessage={formik.errors.port}
                />
              </div>
              <div className="flex-1">
                <TextInput
                  label="Username"
                  name="username"
                  value={formik.values.username}
                  onChange={formik.handleChange}
                  onBlur={formik.handleBlur}
                  placeholder="root"
                  helperText="SSH user (recommended: root)"
                  hasError={formik.touched.username && !!formik.errors.username}
                  errorMessage={formik.errors.username}
                />
              </div>
            </div>
            <ButtonGroup
              label="Authentication Method"
              options={[
                { label: 'Password', value: 'password' },
                { label: 'Key Pair', value: 'keypair' },
              ]}
              value={formik.values.authMethod}
              onSelect={(value) => formik.setFieldValue('authMethod', value)}
            />

            {isKeyPair ? (
              <Textarea
                label="Private Key"
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
            ) : (
              <TextInput
                label="Password"
                name="password"
                type="password"
                value={formik.values.password}
                onChange={formik.handleChange}
                onBlur={formik.handleBlur}
                placeholder="Enter password"
                helperText="SSH password for authentication"
                hasError={formik.touched.password && !!formik.errors.password}
                errorMessage={formik.errors.password}
              />
            )}

            {isKeyPair && (
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

            {!!createRemoteServer.error && (
              <p className="font-manrope font-normal text-sm leading-6 text-error">
                Failed to start connection. Please check your details and try again.
              </p>
            )}

            <Button
              type="submit"
              disabled={isPending}
              className="w-full bg-secondary text-pure-white font-manrope font-bold text-sm leading-6 rounded-lg px-4 py-2 hover:opacity-[0.88] active:opacity-75 transition-opacity duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isPending ? 'Connecting…' : 'Connect & Continue'}
            </Button>
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
