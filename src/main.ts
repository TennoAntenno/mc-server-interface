import { invoke } from "@tauri-apps/api/core";

let serverDownloadBtn: HTMLButtonElement = document.getElementById("get-server") as HTMLButtonElement;
let serverDownloadStatus: HTMLParagraphElement = document.getElementById("server-download-status") as HTMLParagraphElement;

serverDownloadBtn?.addEventListener("click", async () => {
	serverDownloadBtn.disabled = true;
    try {
        await invoke("get_paper_server");
        console.log("Paper server fetched successfully!");
		serverDownloadStatus.style.color = "green";
		serverDownloadStatus.textContent = "Paper server downloaded successfully!";
    } catch (error) {
        console.error("Failed to fetch the Paper server:", error);
		serverDownloadStatus.style.color = "red";
		serverDownloadStatus.textContent = "Failed to fetch the Paper server: " + error;
    }
	serverDownloadBtn.disabled = false;
});
