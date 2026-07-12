import { RouterProvider } from '@tanstack/react-router'
import { createAppRouter } from '@/router'

export default async function App() {
  return <RouterProvider router={await createAppRouter()} />
}
