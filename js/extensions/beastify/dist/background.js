console.log("load background.js");

chrome.action.onClicked.addListener((tab) => {
  console.log("chrome.action.onClicked", tab.id);

  if (!tab.url.includes("chrome://")) {
    chrome.scripting.executeScript({
      target: { tabId: tab.id },
      files: ["./content_scripts/beastify.js"],
    });
  }
});

// ポップアップは、インストール時に設定しておく
chrome.runtime.onInstalled.addListener(() => {
  chrome.action.setPopup({ popup: "./popup/choose_beast.html" });
});
