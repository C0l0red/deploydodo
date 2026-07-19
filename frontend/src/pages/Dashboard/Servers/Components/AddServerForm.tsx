import { useFormik } from 'formik'
import { useCreateLocalServer, useCreateRemoteServer } from '@/api/mutations'
import { TextInput } from '@/components/TextInput'
import { Textarea } from '@/components/Textarea'
import { Button } from '@/components/Button'
import { ButtonGroup } from '@/components/ButtonGroup'
import { invalidateServersQuery, invalidateStatusQuery } from '@/api/queries'
import type { SshAuthRequest } from '@/api/Api'
import { cn } from '@/utilities/cn'
import { AnimatePresence, motion } from 'framer-motion'
import { SpinnerIcon } from '@/assets/icons'
import { addServerValidationSchema } from '@/validations/addServer'

export type AddServerFormProps = {
  error: string | null
  onError: (message: string | null) => void
  onJobCreated: (jobId: string) => void
  onLocalServerCreated: () => void
  onCancel: () => void
  serverType: 'local' | 'remote'
}

export function AddServerForm({
  error,
  onError,
  onJobCreated,
  onLocalServerCreated,
  onCancel,
  serverType,
}: AddServerFormProps) {
  function refreshQueries() {
    return Promise.all([invalidateStatusQuery(), invalidateServersQuery()])
  }

  const createLocal = useCreateLocalServer({
    onSuccess: async () => {
      await refreshQueries()
      onLocalServerCreated()
    },
    onError: (e) => onError(e.message),
  })
  const createRemote = useCreateRemoteServer({
    onSuccess: async (data) => {
      await refreshQueries()
      onJobCreated(data.jobId)
    },
    onError: (e) => onError(e.message),
  })

  const isSubmitting = createLocal.isPending || createRemote.isPending

  const formik = useFormik({
    initialValues: {
      serverType,
      name: '',
      hostname: '',
      port: '22',
      username: 'root',
      authMethod: 'password',
      password: '',
      privateKey: '',
    },
    validationSchema: addServerValidationSchema,
    validateOnMount: false,
    onSubmit: (values) => {
      onError(null)

      if (values.serverType === 'local') {
        createLocal.mutate({ name: values.name.trim() })
      } else {
        const auth: SshAuthRequest =
          values.authMethod === 'password'
            ? {
                authType: 'password',
                username: values.username.trim(),
                password: values.password,
              }
            : {
                authType: 'keypair',
                username: values.username.trim(),
                privateKey: values.privateKey.trim(),
              }

        createRemote.mutate({
          name: values.name.trim(),
          hostname: values.hostname.trim(),
          port: Number(values.port),
          auth,
        })
      }
    },
  })

  const isLocal = formik.values.serverType === 'local'

  return (
    <form onSubmit={formik.handleSubmit} className="flex flex-col flex-1 min-h-0" noValidate>
      {/* Scrollable Body */}
      <div className="flex-1 overflow-y-auto px-6 py-6">
        <div className={cn('flex flex-col gap-5', isLocal ? 'min-h-70' : 'min-h-200')}>
          {/* Server type toggle */}
          <ButtonGroup
            value={formik.values.serverType}
            label="Server Type"
            onSelect={(value) => {
              formik.resetForm()
              onError(null)
              formik.setFieldValue('serverType', value)
            }}
            options={[
              {
                label: 'Local',
                value: 'local',
              },
              {
                label: 'Remote',
                value: 'remote',
              },
            ]}
          />
          {/* Common fields */}
          <TextInput
            label="Name"
            name="name"
            value={formik.values.name}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            placeholder="My Server"
            required
            hasError={formik.touched.name && !!formik.errors.name}
            errorMessage={formik.errors.name}
          />
          {/* Remote-only fields */}
          <AnimatePresence initial>
            {!isLocal && (
              <motion.div
                key="remote-fields"
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                transition={{ duration: 0.5, ease: 'easeOut' }}
                className="overflow-hidden"
              >
                <div className="flex flex-col gap-5">
                  <TextInput
                    label="Hostname"
                    name="hostname"
                    value={formik.values.hostname}
                    onChange={formik.handleChange}
                    onBlur={formik.handleBlur}
                    placeholder="192.168.1.100 or my.server.com"
                    required
                    hasError={formik.touched.hostname && !!formik.errors.hostname}
                    errorMessage={formik.errors.hostname}
                  />
                  <TextInput
                    label="Port"
                    name="port"
                    type="number"
                    value={formik.values.port}
                    onChange={formik.handleChange}
                    onBlur={formik.handleBlur}
                    placeholder="22"
                    hasError={formik.touched.port && !!formik.errors.port}
                    errorMessage={formik.errors.port}
                  />
                  {/* Auth type */}
                  <ButtonGroup
                    label="Authentication"
                    options={[
                      { label: 'Password', value: 'password' },
                      { label: 'Key Pair', value: 'keypair' },
                    ]}
                    value={formik.values.authMethod}
                    onSelect={(value) => {
                      formik.setTouched({
                        ...formik.touched,
                        privateKey: false,
                        password: false,
                      })
                      formik.setFieldValue('authMethod', value)
                    }}
                  />

                  <TextInput
                    label="Username"
                    name="username"
                    value={formik.values.username}
                    onChange={formik.handleChange}
                    onBlur={formik.handleBlur}
                    placeholder="root"
                    required
                    hasError={formik.touched.username && !!formik.errors.username}
                    errorMessage={formik.errors.username}
                  />
                  {formik.values.authMethod === 'password' ? (
                    <TextInput
                      label="Password"
                      name="password"
                      type="password"
                      value={formik.values.password}
                      onChange={formik.handleChange}
                      onBlur={formik.handleBlur}
                      required
                      hasError={formik.touched.password && !!formik.errors.password}
                      errorMessage={formik.errors.password}
                    />
                  ) : (
                    <Textarea
                      label="Private Key"
                      id="private-key"
                      name="privateKey"
                      value={formik.values.privateKey}
                      onChange={formik.handleChange}
                      onBlur={formik.handleBlur}
                      placeholder="-----BEGIN OPENSSH PRIVATE KEY-----"
                      rows={6}
                      hasError={formik.touched.privateKey && !!formik.errors.privateKey}
                      errorMessage={formik.errors.privateKey}
                    />
                  )}
                </div>
              </motion.div>
            )}
          </AnimatePresence>
          {error && <p className="font-manrope text-sm text-error m-0">{error}</p>}
        </div>
      </div>

      {/* Footer */}
      <div className="px-6 py-4 border-t border-neutral-100 flex items-center justify-end gap-3 shrink-0">
        <Button type="button" variant="ghost" onClick={onCancel} disabled={isSubmitting}>
          Cancel
        </Button>
        <Button type="submit" disabled={isSubmitting}>
          {isSubmitting ? (
            <>
              <SpinnerIcon className="animate-spin -ml-1 mr-2 h-4 w-4" />
              Creating...
            </>
          ) : (
            `Create ${isLocal ? 'Local' : 'Remote'} Server`
          )}
        </Button>
      </div>
    </form>
  )
}
