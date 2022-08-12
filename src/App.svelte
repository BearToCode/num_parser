<script lang="ts">
	import './app.scss';

	import { Route, Router } from 'svelte-navigator';
	import Layout from './lib/layout.svelte';

	import TwoDimensions from './routes/two-dimensions.svelte';
	import ThreeDimensions from './routes/three-dimensions.svelte';
	import NavbarItem from './lib/nav/navbar-item.svelte';

	// icons
	import Icon from '@components/common/icon.svelte';
	import HiOutlineFolderOpen from 'svelte-icons-pack/hi/HiOutlineFolderOpen';
	import AiOutlineAppstore from 'svelte-icons-pack/ai/AiOutlineAppstore';
	import BsBraces from 'svelte-icons-pack/bs/BsBraces';
	import IoDocumentText from 'svelte-icons-pack/io/IoDocumentText';
	import HiOutlineCubeTransparent from 'svelte-icons-pack/hi/HiOutlineCubeTransparent';

	export let url = '';
</script>

<Router {url}>
	<Layout>
		<div slot="links" class="flex flex-col">
			<NavbarItem to="/2d">
				<Icon src={HiOutlineFolderOpen} />
			</NavbarItem>
			<NavbarItem to="/3d">
				<Icon src={HiOutlineCubeTransparent} />
			</NavbarItem>
			<NavbarItem to="/code">
				<Icon src={BsBraces} />
			</NavbarItem>
			<NavbarItem to="/document">
				<Icon src={IoDocumentText} />
			</NavbarItem>
			<NavbarItem to="/addons">
				<Icon src={AiOutlineAppstore} />
			</NavbarItem>
		</div>

		<div slot="routes">
			<Route path="/2d"><TwoDimensions /></Route>
			<Route path="/3d"><ThreeDimensions /></Route>
			<Route path="/code"><ThreeDimensions /></Route>
			<Route path="/document"><ThreeDimensions /></Route>
			<Route path="/addons"><ThreeDimensions /></Route>
		</div>
	</Layout>
</Router>
