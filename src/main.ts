import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const received = document.querySelector<HTMLUListElement>("#received")!;
const sent = document.querySelector<HTMLUListElement>("#sent")!;
const form = document.querySelector<HTMLFormElement>("#message-form")!;
const input = document.querySelector<HTMLInputElement>("#message-input")!;
const connectForm = document.querySelector<HTMLFormElement>("#connect-form")!;
const ipInput = document.querySelector<HTMLInputElement>("#ip-input")!;
const portInput = document.querySelector<HTMLInputElement>("#port-input")!;
const status = document.querySelector<HTMLParagraphElement>("#status")!;

function append(list: HTMLUListElement, text: string) {
  const li = document.createElement("li");
  li.textContent = text;
  list.append(li);
  li.scrollIntoView();
}

await listen<string>("message_received", (event) => append(received, event.payload));
await listen<string>("message_sent", (event) => append(sent, event.payload));

await listen<string>("error_occurred", (event) => {
  status.textContent = `Error: ${event.payload}`;
});

await listen<string>("user_connected", (event) => {
  status.textContent = `Conectado con ${event.payload}`;
});

connectForm.addEventListener("submit", async (e) => {
  e.preventDefault();
  status.textContent = "Conectando...";
  try {
    // port es u16 en Rust: hay que mandarlo como número, no como texto
    await invoke("connect_peer", { address: ipInput.value.trim(), port: Number(portInput.value) });
  } catch (err) {
    status.textContent = `Error: ${err}`;
  }
});

// Enter en el input y click en el botón disparan ambos "submit"
form.addEventListener("submit", async (e) => {
  e.preventDefault();
  const message = input.value.trim();
  if (!message) return;
  input.value = "";
  try {
    await invoke("send_message", { message });
  } catch (err) {
    console.error("send_message failed:", err);
  }
});
