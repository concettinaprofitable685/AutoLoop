import { computed, ref } from "vue";
import type { UiGraphModel } from "../types/graph";

export function useGraphSelection(graphModel: { value: UiGraphModel }) {
  const selectedNodeId = ref<string | null>(null);

  const selectedNode = computed(
    () => graphModel.value.nodes.find((node) => node.id === selectedNodeId.value) ?? null
  );

  const relatedNodes = computed(() => {
    if (!selectedNodeId.value) return [];
    const relatedIds = graphModel.value.edges
      .filter(
        (edge) => edge.source === selectedNodeId.value || edge.target === selectedNodeId.value
      )
      .flatMap((edge) => [edge.source, edge.target]);
    return graphModel.value.nodes.filter(
      (node) => node.id !== selectedNodeId.value && relatedIds.includes(node.id)
    );
  });

  function selectNode(nodeId: string | null) {
    selectedNodeId.value = nodeId;
  }

  return {
    selectedNodeId,
    selectedNode,
    relatedNodes,
    selectNode
  };
}
