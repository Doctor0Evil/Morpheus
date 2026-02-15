import type { NodeView } from "../api";

export function NodeCard(node: NodeView): string {
  const bandLabel =
    node.ecoimpact_band < 0.3
      ? "low"
      : node.ecoimpact_band < 0.7
      ? "solid"
      : "high";

  return `
    <div class="node-card">
      <h3>${node.node_id} – ${node.contaminant}</h3>
      <p>K_n: ${node.k_n.toFixed(3)}</p>
      <p>Ecoimpact band: ${bandLabel}</p>
    </div>
  `;
}
