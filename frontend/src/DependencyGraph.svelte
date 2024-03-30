<script>
	import { onMount } from 'svelte';
	import * as d3 from 'd3';

	export let packageName;
	let data;

	async function fetchData() {
		const response = await fetch(`http://127.0.0.1:8081/package?name=${packageName}`);
		const json = await response.json();
		data = json;
		console.log(data);
	}

	onMount(() => {
		fetchData().then(() => {
			renderTree();
		});
	});

	function renderTree() {
		const nodeSize = 17;
		const root = d3.hierarchy(data).eachBefore(
			(
				(i) => (d) =>
					(d.index = i++)
			)(0)
		);
		const nodes = root.descendants();
		const width = 928;
		const height = (nodes.length + 1) * nodeSize;

		const svg = d3
			.select('#tree-container')
			.append('svg')
			.attr('width', width)
			.attr('height', height)
			.attr('viewBox', [-nodeSize / 2, (-nodeSize * 3) / 2, width, height])
			.attr('style', 'max-width: 100%; height: auto; font: 10px sans-serif; overflow: visible;');

		const link = svg
			.append('g')
			.attr('fill', 'none')
			.attr('stroke', '#F7F7F7')
			.selectAll()
			.data(root.links())
			.join('path')
			.attr(
				'd',
				(d) => `
                      M${d.source.depth * nodeSize},${d.source.index * nodeSize}
                      V${d.target.index * nodeSize}
                      h${nodeSize}
                    `
			);

		const node = svg
			.append('g')
			.selectAll()
			.data(nodes)
			.join('g')
			.attr('transform', (d) => `translate(0,${d.index * nodeSize})`);

		node
			.append('circle')
			.attr('cx', (d) => d.depth * nodeSize)
			.attr('r', 5)
			.attr('fill', (d) => (d.children ? '#ff3e00' : '#ffaa80'));

		node
			.append('text')
			.attr('dy', '0.3em')
			.attr('font-size', '14px')
			.attr('x', (d) => d.depth * nodeSize + 10)
			.attr('fill', '#F7F7F7')
			.text((d) => d.data.name);

		node.append('title').text((d) =>
			d
				.ancestors()
				.reverse()
				.map((d) => d.data.name)
				.join('/')
		);
	}
</script>

<div id="tree-container" style="background-color: #333f49;"></div>

<style>
	#tree-container {
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
