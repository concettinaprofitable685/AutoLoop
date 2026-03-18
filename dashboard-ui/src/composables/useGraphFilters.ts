import { computed, ref } from "vue";
import type { UiGraphModel } from "../types/graph";

export function useGraphFilters(graphModel: { value: UiGraphModel }) {
  const selectedEntityTypes = ref<string[]>([]);
  const selectedEdgeKinds = ref<string[]>([]);
  const searchQuery = ref("");

  const entityFilters = computed(() =>
    Array.from(new Set(graphModel.value.nodes.map((node) => node.kind)))
  );
  const relationFilters = computed(() =>
    Array.from(new Set(graphModel.value.edges.map((edge) => edge.kind)))
  );

  const filteredGraph = computed<UiGraphModel>(() => {
    const query = searchQuery.value.trim().toLowerCase();
    const nodes = graphModel.value.nodes.filter((node) => {
      const entityMatch =
        selectedEntityTypes.value.length === 0 || selectedEntityTypes.value.includes(node.kind);
      const searchMatch = query.length === 0 || node.label.toLowerCase().includes(query);
      return entityMatch && searchMatch;
    });
    const nodeIds = new Set(nodes.map((node) => node.id));
    const edges = graphModel.value.edges.filter((edge) => {
      const relationMatch =
        selectedEdgeKinds.value.length === 0 || selectedEdgeKinds.value.includes(edge.kind);
      return relationMatch && nodeIds.has(edge.source) && nodeIds.has(edge.target);
    });
    return { nodes, edges };
  });

  function toggleEntityType(kind: string) {
    selectedEntityTypes.value = selectedEntityTypes.value.includes(kind)
      ? selectedEntityTypes.value.filter((item) => item !== kind)
      : [...selectedEntityTypes.value, kind];
  }

  function toggleEdgeKind(kind: string) {
    selectedEdgeKinds.value = selectedEdgeKinds.value.includes(kind)
      ? selectedEdgeKinds.value.filter((item) => item !== kind)
      : [...selectedEdgeKinds.value, kind];
  }

  function setSearch(value: string) {
    searchQuery.value = value;
  }

  function reset() {
    selectedEntityTypes.value = [];
    selectedEdgeKinds.value = [];
    searchQuery.value = "";
  }

  return {
    selectedEntityTypes,
    selectedEdgeKinds,
    searchQuery,
    entityFilters,
    relationFilters,
    filteredGraph,
    toggleEntityType,
    toggleEdgeKind,
    setSearch,
    reset
  };
}
