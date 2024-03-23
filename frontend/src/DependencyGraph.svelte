<script>
  import { onMount } from 'svelte';
  import * as d3 from 'd3';

  export let packageName;

  onMount(() => {
    console.log('package');
  });
</script>

<div id="dependency-graph"></div>

<style>
</style>


