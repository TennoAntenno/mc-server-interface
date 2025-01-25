import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";

let serverDownloadBtn: HTMLButtonElement = document.getElementById("get-server") as HTMLButtonElement;
let serverDownloadStatus: HTMLParagraphElement = document.getElementById("server-download-status") as HTMLParagraphElement;

let serverOpenBtn: HTMLButtonElement = document.getElementById("open-server") as HTMLButtonElement;
let serverOpenStatus: HTMLParagraphElement = document.getElementById("server-open-status") as HTMLParagraphElement;

let serverOutput: HTMLElement = document.getElementById("server-output") as HTMLElement;
let serverInput: HTMLInputElement = document.getElementById("server-input") as HTMLInputElement;

// perform server download
serverDownloadBtn?.addEventListener("click", async () => {
	serverDownloadBtn.disabled = true;
    try {
        await invoke("get_paper_server");
        console.log("Paper server fetched successfully!");
		serverDownloadStatus.style.color = "green";
		serverDownloadStatus.textContent = "Paper server downloaded successfully!";

        serverOpenBtn.disabled = false;
    } catch (error) {
        console.error("Failed to fetch the Paper server:", error);
		serverDownloadStatus.style.color = "red";
		serverDownloadStatus.textContent = "Failed to fetch the Paper server: " + error;
    }
	serverDownloadBtn.disabled = false;
});

// perform server launch
serverOpenBtn.disabled = true;
serverOpenBtn?.addEventListener("click", async () => {
    serverOpenStatus.style.color = "white";
    serverOpenStatus.textContent = "Paper server running...";
    serverOpenBtn.disabled = true;
    try {
        await invoke("open_paper_server");
        console.log("Paper server opened successfully!");
		serverOpenStatus.style.color = "green";
		serverOpenStatus.textContent = "Paper ran successfully!";

        serverOpenBtn.disabled = false;
    } catch (error) {
        console.error("Failed to open the Paper server:", error);
		serverOpenStatus.style.color = "red";
		serverOpenStatus.textContent = "Failed to open the Paper server: " + error;
    }
    serverOpenBtn.disabled = false;
});

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