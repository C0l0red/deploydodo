import { createLazyRoute } from "@tanstack/react-router";
import { Pending } from "../Pending/Pending";
import { SelectServer } from "./SelectServer";

export const SetupRoute = createLazyRoute('setup')({
  pendingComponent: Pending,
  component: SelectServer,
})
