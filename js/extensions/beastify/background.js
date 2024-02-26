console.log("load background.js");

chrome.action.onClicked.addListener((tab) => {
  console.log("chrome.action.onClicked", tab.id);

  if (!tab.url.includes("chrome://")) {
    chrome.scripting.executeScript({
      target: { tabId: tab.id },
      files: ["./content_scripts/beastify.js"],
    });
  }
  chrome.action.setPopup({ popup: "./popup/choose_beast.html" });
});
