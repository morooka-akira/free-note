import type {
  PlasmoCSConfig,
  PlasmoCSUIJSXContainer,
  PlasmoCSUIProps,
  PlasmoGetOverlayAnchor,
  PlasmoRender
} from "plasmo"
import type { FC } from "react"
import { createRoot } from "react-dom/client"

export const getRootContainer = () =>
  new Promise((resolve) => {
    const checkInterval = setInterval(() => {
      console.log("root containerを探しています")
      const rootContainerParent = document.querySelector(`ul[role="menu"]`)
      if (rootContainerParent) {
        clearInterval(checkInterval)
        const rootContainer = document.createElement("div")
        rootContainerParent.appendChild(rootContainer)
        resolve(rootContainer)
      }
    }, 137)
  })

const PlasmoOverlay: FC<PlasmoCSUIProps> = () => {
  return (
    <span
      style={{
        borderRadius: 4,
        background: "yellow",
        padding: 4,
        position: "absolute",
        top: 0,
        left: 0,
        transform: "translateY(-24px) translateX(42px)"
      }}>
      CSUI ROOT CONTAINER
    </span>
  )
}

export const render: PlasmoRender<PlasmoCSUIJSXContainer> = async ({
  createRootContainer
}) => {
  const rootContainer = await createRootContainer()
  const root = createRoot(rootContainer)
  root.render(<PlasmoOverlay />)
}

export default PlasmoOverlay

export const getOverlayAnchor: PlasmoGetOverlayAnchor = async () =>
  document.querySelector(`ul[role="menu"]`)
