SINGLE SOURCE SHORTEST/LONGEST PATH
- BFS when weight is 1 (queue)
- Dijkstra when weight > 1 (priority queue; min/max heap)
- Bellman Ford when negative weights present (array of nodes)(DP)

DETECTING CYCLES

- Undirected graph
    - DFS
    - keep track of parents
    - if node visited node encountered and node not parent; cycle detected

- Directed graph:
    - for each node(to tackle disjointed graph) do
    - push unvisited node to stack; mark the node as tracked; mark the node visited
    - DFS traverse; do same for each child
    - if no child, then pop and mark the node as untracked;
    - if child present and already marked, we encountered a cycle.

- in Bellman Ford (negative edge cost); we initialise distance vector with 0 length;
    relaxations will update as negative costs will be lower than 0.
