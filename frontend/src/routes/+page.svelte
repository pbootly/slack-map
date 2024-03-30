<script>
	import { onMount } from 'svelte';
	import { createEventDispatcher } from 'svelte';
	import DependencyGraph from '../DependencyGraph.svelte';

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
	<h1>Packages Found: {packages.length}</h1>
	<input
		type="text"
		placeholder="Search Package"
		class="input input-bordered w-full max-w-xs"
		bind:value={searchTerm}
		on:input={handleSearch}
	/>
	<div class="flex flex-col w-full border-opacity-50">
		<div class="divider"></div>
	</div>
	<table>
		<thead>
			<tr>
				<th>Name</th>
				<th>Version</th>
			</tr>
		</thead>
		<tbody>
			{#each packages.filter((pkg) => pkg.name.toLowerCase().includes(searchTerm)) as pkg}
				<tr>
					{#if pkg.requires.length}
						<td>
							<a
								style="color: #ff3e00;"
								href="javascript:void(0)"
								on:click={() => handlePackageClick(pkg.name)}>{pkg.name}</a
							></td
						>
					{:else}
						<td>{pkg.name}</td>
					{/if}

					<td>{pkg.version}</td>
				</tr>
				<tr>
					<td colspan="2" style="text-align: center;">
						{#if selectedPackage === pkg.name}
							<DependencyGraph packageName={selectedPackage} {packages} />
						{/if}
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
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
