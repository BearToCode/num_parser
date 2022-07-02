<script lang="ts">
	import './app.scss';

	import { Route, Router } from 'svelte-navigator';
	import Layout from './lib/layout.svelte';

	import TwoDimensions from './routes/two-dimensions.svelte';
	import ThreeDimensions from './routes/three-dimensions.svelte';
	import NavbarItem from './lib/nav/navbar-item.svelte';

	// icons
	import Icon from 'svelte-icons-pack';
	import AiOutlineFunction from 'svelte-icons-pack/ai/AiOutlineFunction';
	import AiOutlineAppstore from 'svelte-icons-pack/ai/AiOutlineAppstore';
	import BsBraces from 'svelte-icons-pack/bs/BsBraces';
	import IoDocumentText from 'svelte-icons-pack/io/IoDocumentText';
	import BiCube from 'svelte-icons-pack/bi/BiCube';

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
