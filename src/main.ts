import { invoke } from "@tauri-apps/api/core";
import { Window } from "@tauri-apps/api/window";
import { Notyf } from 'notyf';
import 'notyf/notyf.min.css';

const notyf = new Notyf();
notyf.options.duration = 5000;

let serverDownloadBtn: HTMLButtonElement = document.getElementById("get-server") as HTMLButtonElement;

let serverOpenBtn: HTMLButtonElement = document.getElementById("open-server") as HTMLButtonElement;

let restartServerBtn: HTMLButtonElement = document.getElementById("restart-server") as HTMLButtonElement;

// perform server download
serverDownloadBtn?.addEventListener("click", async () => {
	serverDownloadBtn.disabled = true;
    try {
        await invoke("get_paper_server");
        console.log("Paper server fetched successfully!");
        notyf.success("Paper server downloaded successfully!");
        serverOpenBtn.disabled = false;
    } catch (error) {
        console.error("Failed to fetch the Paper server:", error);
		notyf.error("Failed to download the Paper server: " + error);
    }
	serverDownloadBtn.disabled = false;
});

// perform server launch
serverOpenBtn?.addEventListener("click", async () => {
    notyf.success("Paper server starting...");
    serverOpenBtn.disabled = true;
    try {
        await invoke("open_paper_server");
        console.log("Paper server opened successfully!");
	    notyf.success("Paper ran successfully!");

        serverOpenBtn.disabled = false;
    } catch (error) {
        console.error("Failed to open the Paper server:", error);
		notyf.error("Failed to run the Paper server: " + error);
    }
    serverOpenBtn.disabled = false;
});

// perform server restart
restartServerBtn.addEventListener("click", async () => {
    try {
        await invoke("restart_paper_server");
        console.log("Paper server restarted successfully!");
        notyf.success("Paper server restarted successfully!");
    } catch (error) {
        console.error("Failed to restart the Paper server:", error);
        notyf.error("Failed to restart the Paper server: " + error);
    }
});
