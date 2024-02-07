SINGLE SOURCE SHORTEST/LONGEST PATH
- BFS when weight is 1 (queue)
- Dijkstra when weight > 1 (priority queue; min/max heap)
- Bellman Ford when negative weights present (array of nodes)(DP)

DETECTING CYCLES
- in Bellman Ford (negative edge cost); we initialise distance vector with 0 length;
    relaxations will update as negative costs will be lower than 0.