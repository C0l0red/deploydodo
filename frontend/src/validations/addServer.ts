import * as Yup from 'yup'

type AddServerSchema = {
  serverType: 'local' | 'remote'
  name: string
  hostname: string
  port: string
  username: string
  authMethod: 'password' | 'keypair'
  password: string
  privateKey: string
}

export const addServerValidationSchema = Yup.object<AddServerSchema>({
  name: Yup.string().trim().required('Server name is required'),
  hostname: Yup.string()
    .trim()
    .when('serverType', {
      is: 'remote' as const,
      then: (s) => s.trim().required('Host / IP is required'),
      otherwise: (s) => s,
    }),
  port: Yup.string().when('serverType', {
    is: 'remote' as const,
    then: (s) =>
      s
        .test('is-number', 'Port must be a number', (v) => v === '' || !isNaN(Number(v)))
        .test('is-in-range', 'Port must be between 1 and 65535', (v) => {
          if (v === '') return false
          const n = Number(v)
          return n >= 1 && n <= 65535
        })
        .required('Port is required'),
    otherwise: (s) => s,
  }),
  username: Yup.string().when('serverType', {
    is: 'remote' as const,
    then: (s) => s.trim().required('Username is required'),
    otherwise: (s) => s,
  }),
  privateKey: Yup.string().when(['authMethod', 'serverType'], {
    is: (authMethod: string, serverType: string) =>
      authMethod === 'keypair' && serverType === 'remote',
    then: (s) => s.trim().required('Private key is required'),
    otherwise: (s) => s,
  }),
  password: Yup.string().when(['authMethod', 'serverType'], {
    is: (authMethod: string, serverType: string) =>
      authMethod === 'password' && serverType === 'remote',
    then: (s) => s.required('Password is required'),
    otherwise: (s) => s,
  }),
})
