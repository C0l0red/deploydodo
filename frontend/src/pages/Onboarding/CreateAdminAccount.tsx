import { useFormik } from 'formik'
import * as Yup from 'yup'
import { TextInput } from '@/components/TextInput'
import { Button } from '@/components/Button'

const schema = Yup.object({
  name: Yup.string().trim().required('Name is required'),
  email: Yup.string().trim().email('Enter a valid email').required('Email is required'),
})

type CreateAdminAccountProps = {
  onProceed: (name: string, email: string) => void
}

export function CreateAdminAccount({ onProceed }: CreateAdminAccountProps) {
  const formik = useFormik({
    initialValues: { name: '', email: '' },
    validationSchema: schema,
    onSubmit: async (values) => {
      onProceed(values.name.trim(), values.email.trim())
    },
  })

  return (
    <div className="bg-background rounded-xl shadow-[0px_8px_24px_0px_rgba(0,0,0,0.08),0px_0px_1px_0px_rgba(0,0,0,0.2)] w-114 max-w-full px-6 pt-4 pb-2.5">
      <div className="flex flex-col gap-6">
        <h1 className="font-sans font-semibold text-2xl leading-8 text-secondary m-0">
          Create admin account
        </h1>

        <form className="flex flex-col gap-4" onSubmit={formik.handleSubmit}>
          <div className="flex flex-col gap-4">
            <TextInput
              label="Name"
              name="name"
              placeholder="John Smith"
              value={formik.values.name}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              hasError={formik.touched.name && !!formik.errors.name}
              errorMessage={formik.errors.name}
            />
            <TextInput
              label="Email"
              name="email"
              type="email"
              placeholder="name@example.com"
              value={formik.values.email}
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              hasError={formik.touched.email && !!formik.errors.email}
              errorMessage={formik.errors.email}
            />
            {/* <p className="font-sans font-normal text-sm leading-6 text-secondary">
              By choosing to proceed you agree to our{' '}
              <a
                href="#"
                className="font-semibold text-secondary underline [text-decoration-skip-ink:none]"
              >
                Terms &amp; Conditions
              </a>
            </p> */}
          </div>

          <div className="flex flex-col gap-4 pb-6 pt-2">
            <Button type="submit" fullWidth>
              Proceed
            </Button>
            {/* <p className="font-sans font-normal text-sm leading-6 text-secondary text-center">
              Already have an account?{' '}
              <a
                href="#"
                className="font-semibold text-secondary underline [text-decoration-skip-ink:none]"
              >
                Sign in
              </a>
            </p> */}
          </div>
        </form>
      </div>
    </div>
  )
}
