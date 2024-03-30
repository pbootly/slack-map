<script>
  import { onMount } from 'svelte';

  // Define your component's structure
  export let packageTree;

  // Retain module-scoped expansion state for each tree node
  const _expansionState = {};

  // Function to toggle expansion state
  function toggleExpansion(label) {
    _expansionState[label] = !_expansionState[label];
  }

  onMount(() => {
    // Initialize expansion state if necessary
    if (!packageTree) return;
    initializeExpansionState(packageTree);
  });

  function initializeExpansionState(tree) {
    if (!tree) return;
    _expansionState[tree.name] = false; // Initialize expansion state for the root node
    if (tree.children) {
      tree.children.forEach(child => initializeExpansionState(child));
    }
  }
</script>

<div id="dependency-graph">
  <!-- Render the tree recursively -->
  <TreeItem {treeItem=packageTree} />
</div>

<!-- Define a recursive component to render the tree -->
<script>
  let arrowDown;

  // Recursive component to render each node in the tree
  function TreeItem(props) {
    const { treeItem } = props;
    let expanded = _expansionState[treeItem.name] || false;

    function handleClick() {
      toggleExpansion(treeItem.name);
      expanded = _expansionState[treeItem.name];
    }

    $: arrowDown = expanded;

    return (
      <ul>
        <li>
          <span on:click={handleClick}>
            <span class="arrow" class:arrowDown>&#x25b6;</span>
            {treeItem.name}
          </span>
          {#if expanded && treeItem.children}
            <ul>
              {#each treeItem.children as child}
                <li>
                  <TreeItem treeItem={child} />
                </li>
              {/each}
            </ul>
          {/if}
        </li>
      </ul>
    );
  }
</script>

<style>
  ul {
    margin: 0;
    list-style: none;
    padding-left: 1.2rem; 
    user-select: none;
  }
  .arrow {
    cursor: pointer;
    display: inline-block;
    /* transition: transform 200ms; */
  }
  .arrowDown { transform: rotate(90deg); }
</style>

