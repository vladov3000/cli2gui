"use strict";

const invoke = window.__TAURI__.tauri.invoke;

async function generate() {
  const commandInput   = document.getElementById("commandInput");
  const generateButton = document.getElementById("generateButton");

  generateButton.disabled = true;

  const helpText = commandInput.value;
  const prompt = `There is already a function defined called run that executes commands. Use checkboxes for flags. Use text inputs for other arguments. Write javascript that programatically creates a ui for a cli command with the following description: ${helpText}.`;

  const response = await invoke("query", { content: prompt });
  let   message  = JSON.parse(response)["choices"][0].message.content;

  const badPrefix = "```javascript";
  const badSuffix = "```";
  if (message.startsWith(badPrefix)) {
    message = message.slice(badPrefix.length);
  }
  if (message.endsWith(badSuffix)) {
    message = message.slice(0, -badSuffix.length);
  }

  console.log(message);
  eval(message);
  document.body.appendChild(document.createElement("hr"));

  generateButton.disabled = false;
}

function run(command) {
  window.confirm(`Run ${command}?`).then((confirmed) => {
    invoke("run", { command }).then((output) => {
      console.log(output);
      const outputSpan = document.getElementById("outputSpan");
      outputSpan.textContent = output;
    });
  });
}

function initialize() {
  const commandText    = document.getElementById("commandInput");
  const generateButton = document.getElementById("generateButton");
  generateButton.addEventListener("click", generate);
}

document.addEventListener("DOMContentLoaded", initialize);
