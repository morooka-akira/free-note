const hidePage = `body > :not(.beastify-image) { display: none; }`;

function beastNameToURL(beastName) {
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

  console.log("click ", e.target);
  const tabId = tab.id;

  function applyCSS(tabId) {
    chrome.scripting.insertCSS({
      target: { tabId: tabId },
      css: hidePage,
    });
  }

  function removeCSS(tabId) {
    chrome.scripting.removeCSS({
      target: { tabId: tabId },
      css: hidePage,
    });
  }

  function sendMessage(tabId, command, url) {
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

  if (e.target.type === "reset") {
    console.log("hoge");
    removeCSS(tabId);
    sendMessage(tabId, "reset");
  } else {
    console.log("fuga");
    const url = beastNameToURL(e.target.textContent);
    applyCSS(tabId);
    sendMessage(tabId, "beastify", url);
  }
});

function reportError(error) {
  console.error(`Could not beastify: ${error}`);
}
