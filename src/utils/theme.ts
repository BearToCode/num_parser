import resolveConfig from 'tailwindcss/resolveConfig';
import tailwindConfig from 'tailwind.config.cjs';
const fullConfig = resolveConfig(tailwindConfig);

export default fullConfig.theme;
