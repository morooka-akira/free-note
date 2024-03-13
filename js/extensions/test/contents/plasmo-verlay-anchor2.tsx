import type {
  PlasmoCSConfig,
  PlasmoCSUIProps,
  PlasmoGetInlineAnchor,
  PlasmoGetOverlayAnchor
} from "plasmo"
import type { FC } from "react"

// export const config: PlasmoCSConfig = {
//   matches: ["https://www.plasmo.com/*"]
// }

export const getInlineAnchor: PlasmoGetInlineAnchor = async () => {
  const menu = document.querySelector(
    `ul[role="menu"][data-testid="highlighted-line-menu"] > li`
  )
  return menu
}

const PlasmoPricingExtra: FC<PlasmoCSUIProps> = (props) => (
  <span
    style={{
      borderRadius: 4,
      background: "beige",
      padding: 4
    }}>
    {console.log(props)}
    CSUI OVERLAY ANCHOR
  </span>
)

export default PlasmoPricingExtra
