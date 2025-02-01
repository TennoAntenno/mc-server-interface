import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";

let serverOutput: HTMLElement = document.getElementById("server-output") as HTMLElement;
let serverInput: HTMLInputElement = document.getElementById("server-input") as HTMLInputElement;

invoke("watch_latest_log");
const currentWindow = Window.getCurrent();
currentWindow.listen<string>("log-updated", (event) => {
  const logContent = event.payload;
  console.log("Log updated:", logContent);

  // Update the UI with the log content
  if (serverOutput) {
    serverOutput.innerHTML = logContent.replace(/\n/g, "<br>");
    // scroll to bottom if already at the bottom (with margin of 100px)
    if (serverOutput.scrollHeight - serverOutput.scrollTop <= serverOutput.clientHeight + 100) { 
      serverOutput.scrollTop = serverOutput.scrollHeight;
    }
    
  }
});

serverInput.addEventListener("keyup", (event) => {
    if (event.key === "Enter") {
        const command = serverInput.value;
        serverInput.value = "";

        try {
            invoke("run_command", { command });
            console.log("Command executed successfully:", command);
        } catch (error) {
            console.error("Failed to execute command:", error);
        }
    }
});