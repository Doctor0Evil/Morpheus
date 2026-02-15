export interface NodeView {
  node_id: string;
  contaminant: string;
  k_n: number;
  ecoimpact_band: number;
}

export async function fetchNodes(baseUrl: string): Promise<NodeView[]> {
  const resp = await fetch(`${baseUrl}/nodes`);
  if (!resp.ok) {
    throw new Error(`failed to fetch nodes: ${resp.status}`);
  }
  return await resp.json();
}
