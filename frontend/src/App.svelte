<script>
    import { onMount } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import DependencyGraph from './DependencyGraph.svelte';

    let packages = [];
    let selectedPackage = null;
    let searchTerm = '';
    const dispatch = createEventDispatcher();

    onMount(async () => {
        const response = await fetch('http://127.0.0.1:8081/packages');
        const jsonData = await response.json();
        packages = Object.values(jsonData);
    });

    function handleSearch(event) {
        searchTerm = event.target.value.toLowerCase();
        dispatch('search', searchTerm);
    }

    function handlePackageClick(packageName) {
        selectedPackage = packageName;
    }
</script>

<main>
	<h1>Packages Found: {packages.length} </h1>    
    <input type="text" placeholder="Search by key" bind:value={searchTerm} on:input={handleSearch} />
    <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Version</th>
            <th>Dependencies</th>
          </tr>
        </thead>
    <tbody>
      {#each packages.filter(pkg => pkg.name.toLowerCase().includes(searchTerm)) as pkg}
        <tr>
          <td> <a href="javascript:void(0)" on:click={() => handlePackageClick(name)}>{pkg.name}</a></td>
          <td>{pkg.version}</td>
          <td>{pkg.requires.length}</td>
        </tr>
      {/each}
    </tbody>
  </table>
  {#if selectedPackage}
    <DependencyGraph packageName={selectedPackage} />
  {/if}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

    table {
        margin: 0 auto;
    }

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>

