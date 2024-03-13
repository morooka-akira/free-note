import type { PlasmoGetInlineAnchor } from "plasmo"

export {}

window.addEventListener("load", () => {
  console.log("画面がロードされたよ")
  // NOTE: debugように背景色を変える
  document.body.style.background = "pink"

  //// 新しい`<li>`要素を作成する
  const config = { childList: true, subtree: true }

  // コールバック関数を定義する
  const callback = function (mutationsList, observer) {
    console.log("なにかロードされたよ", mutationsList)
    for (let mutation of mutationsList) {
      if (mutation.type === "childList") {
        mutation.addedNodes.forEach((node) => {
          if (node.nodeType !== Node.ELEMENT_NODE) {
            return
          }
          const menu = node.querySelector(
            'ul[role="menu"][data-testid="highlighted-line-menu"]'
          )
          if (!menu) {
            return
          }
          // 既に拡張のリストを追加しているかチェックする
          const extension = node.querySelector('li[ext="code-ext"]')
          if (extension) {
            return
          }
          // 特定の要素が追加されたかどうかをチェック
          const newLi = document.createElement("li")

          const existingLis = menu.querySelectorAll("li")
          if (existingLis.length > 0) {
            const firstLi = existingLis[0]
            firstLi.classList.forEach((className) => {
              newLi.classList.add(className)
            })
          }
          newLi.setAttribute("role", "menuitem")
          newLi.setAttribute("ext", "code-ext")

          const newDiv = document.createElement("div")
          const newSpan = document.createElement("span")
          newSpan.textContent = "コードにコメントをつける"
          newDiv.appendChild(newSpan)
          newLi.appendChild(newDiv)
          newLi.addEventListener("click", () => {
            const currentUrl = window.location.href
            console.log("現在のURL:", currentUrl)
          })

          menu.appendChild(newLi)
        })
      }
    }
  }

  // MutationObserverのインスタンスを作成
  const observer = new MutationObserver(callback)

  // 監視を開始する
  observer.observe(document.body, config)
})
