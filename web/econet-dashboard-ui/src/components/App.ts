import { fetchNodes } from "../api";
import { NodeCard } from "./NodeCard";

export async function App(baseUrl: string): Promise<void> {
  const root = document.getElementById("app");
  if (!root) {
    return;
  }
  try {
    const nodes = await fetchNodes(baseUrl);
    const cards = nodes.map(NodeCard).join("");
    root.innerHTML = `
      <main>
        <h1>Phoenix EcoNet Corridor</h1>
        <section class="nodes">
          ${cards}
        </section>
      </main>
    `;
  } catch (err) {
    root.innerHTML = `<p>Failed to load nodes: ${(err as Error).message}</p>`;
  }
}
