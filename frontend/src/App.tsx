import { RouterProvider } from '@tanstack/react-router'
import { createAppRouter } from '@/router'

export default function App() {
  return <RouterProvider router={createAppRouter()} />
}
