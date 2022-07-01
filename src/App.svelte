<script lang="ts">
	import './app.css';

	import { Route, Router } from 'svelte-navigator';
	import Layout from './lib/layout.svelte';

	import TwoDimensions from './routes/two-dimensions.svelte';
	import ThreeDimensions from './routes/three-dimensions.svelte';
	import NavbarItem from './lib/navigation/navbar-item.svelte';

	// icons
	const Icon = require('svelte-icons-pack');
	const AiOutlineFunction = require('svelte-icons-pack/ai/AiOutlineFunction');
	const AiOutlineAppstore = require('svelte-icons-pack/ai/AiOutlineAppstore');
	const BsBraces = require('svelte-icons-pack/bs/BsBraces');
	const IoDocumentText = require('svelte-icons-pack/io/IoDocumentText');
	const BiCube = require('svelte-icons-pack/bi/BiCube');

	export let url = '';
</script>

<Router {url}>
	<Layout>
		<div slot="links" class="flex flex-col">
			<NavbarItem to="/2d">
				<Icon color="currentcolor" src={AiOutlineFunction} />
			</NavbarItem>
			<NavbarItem to="/3d">
				<Icon color="currentcolor" src={BiCube} />
			</NavbarItem>
			<NavbarItem to="/code">
				<Icon color="currentcolor" src={BsBraces} />
			</NavbarItem>
			<NavbarItem to="/document">
				<Icon color="currentcolor" src={IoDocumentText} />
			</NavbarItem>
			<NavbarItem to="/addons">
				<Icon color="currentcolor" src={AiOutlineAppstore} />
			</NavbarItem>
		</div>

		<div slot="routes">
			<Route path="/2d" component={TwoDimensions} />
			<Route path="/3d" component={ThreeDimensions} />
			<Route path="/code" component={ThreeDimensions} />
			<Route path="/document" component={ThreeDimensions} />
			<Route path="/addons" component={ThreeDimensions} />
		</div>
	</Layout>
</Router>
