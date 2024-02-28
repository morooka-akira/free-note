const hidePage = `body > :not(.beastify-image) { display: none; }`;

function beastNameToURL(beastName: string) {
  switch (beastName) {
    case "Frog":
      return chrome.runtime.getURL("beasts/frog.jpg");
    case "Snake":
      return chrome.runtime.getURL("beasts/snake.jpg");
    case "Turtle":
      return chrome.runtime.getURL("beasts/turtle.jpg");
  }
}

document.addEventListener("click", async (e) => {
  const [tab] = await chrome.tabs.query({
    active: true,
  });

  const tabId = tab.id;

  function applyCSS(tabId: number) {
    chrome.scripting.insertCSS({
      target: { tabId: tabId },
      css: hidePage,
    });
  }

  function removeCSS(tabId: number) {
    chrome.scripting.removeCSS({
      target: { tabId: tabId },
      css: hidePage,
    });
  }

  function sendMessage(tabId: number, command: string, url: string) {
    chrome.tabs
      .sendMessage(tabId, {
        command: command,
        beastURL: url,
      })
      .then((response) => {
        console.log("response", response);
      })
      .catch((error) => {
        console.error(`Could not beastify: ${error}`);
      });
  }

  if (e.target && (e.target as HTMLButtonElement).type === "reset") {
    if (tabId) {
      removeCSS(tabId);
      sendMessage(tabId, "reset", "");
    }
  } else {
    const url = beastNameToURL(
      (e.target as HTMLElement)?.textContent as string,
    );
    if (tabId && url) {
      applyCSS(tabId);
      sendMessage(tabId, "beastify", url);
    }
  }
});
