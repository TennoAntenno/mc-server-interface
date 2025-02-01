import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";

let serverDownloadBtn: HTMLButtonElement = document.getElementById("get-server") as HTMLButtonElement;
let serverDownloadStatus: HTMLParagraphElement = document.getElementById("server-download-status") as HTMLParagraphElement;

let serverOpenBtn: HTMLButtonElement = document.getElementById("open-server") as HTMLButtonElement;
let serverOpenStatus: HTMLParagraphElement = document.getElementById("server-open-status") as HTMLParagraphElement;

let restartServerBtn: HTMLButtonElement = document.getElementById("restart-server") as HTMLButtonElement;

// perform server download
serverDownloadBtn?.addEventListener("click", async () => {
	serverDownloadBtn.disabled = true;
    try {
        await invoke("get_paper_server");
        console.log("Paper server fetched successfully!");
        serverDownloadStatus.classList.remove("negative");
		serverDownloadStatus.classList.add("positive");
		serverDownloadStatus.textContent = "Paper server downloaded successfully!";

        serverOpenBtn.disabled = false;
    } catch (error) {
        console.error("Failed to fetch the Paper server:", error);
		serverDownloadStatus.classList.remove("positive");
		serverDownloadStatus.classList.add("negative");
		serverDownloadStatus.textContent = "Failed to fetch the Paper server: " + error;
    }
	serverDownloadBtn.disabled = false;
});

// perform server launch
serverOpenBtn?.addEventListener("click", async () => {
    serverOpenStatus.classList.remove("positive");
	serverOpenStatus.classList.add("negative");
    serverOpenStatus.textContent = "Paper server running...";
    serverOpenBtn.disabled = true;
    try {
        await invoke("open_paper_server");
        console.log("Paper server opened successfully!");
	    serverOpenStatus.classList.remove("negative");
		serverOpenStatus.classList.add("positive");
		serverOpenStatus.textContent = "Paper ran successfully!";

        serverOpenBtn.disabled = false;
    } catch (error) {
        console.error("Failed to open the Paper server:", error);
		serverOpenStatus.classList.remove("positive");
		serverOpenStatus.classList.add("negative");
		serverOpenStatus.textContent = "Failed to open the Paper server: " + error;
    }
    serverOpenBtn.disabled = false;
});

// perform server restart
restartServerBtn.addEventListener("click", async () => {
    try {
        await invoke("restart_paper_server");
        console.log("Paper server restarted successfully!");
    } catch (error) {
        console.error("Failed to restart the Paper server:", error);
    }
});
