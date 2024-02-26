(() => {
  /**
   * グローバルなガード変数をチェック、設定する。
   * コンテンツスクリプトが再び同じページに挿入された場合、
   * 次は何もしない。
   */
  if (window.hasRun) {
    return;
  }
  window.hasRun = true;

  /**
   * 動物の画像の URL を受け取り、既存の動物をすべて削除し、次に
   * 画像を指す IMG 要素の作成・スタイル適用を行い、
   * 作成したノードをドキュメント内に挿入する
   */
  function insertBeast(beastURL) {
    removeExistingBeasts();
    const beastImage = document.createElement("img");
    beastImage.setAttribute("src", beastURL);
    beastImage.style.height = "100vh";
    beastImage.className = "beastify-image";
    document.body.appendChild(beastImage);
  }

  /**
   * ページからすべての動物を削除する
   */
  function removeExistingBeasts() {
    const existingBeasts = document.querySelectorAll(".beastify-image");
    for (const beast of existingBeasts) {
      beast.remove();
    }
  }

  /**
   * バックグラウンドスクリプトからのメッセージを待ち受けし、
   * "beastify()" か "reset()" を呼び出す。
   */
  browser.runtime.onMessage.addListener((message) => {
    if (message.command === "beastify") {
      insertBeast(message.beastURL);
    } else if (message.command === "reset") {
      removeExistingBeasts();
    }
  });
})();
